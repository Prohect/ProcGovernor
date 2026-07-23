use crate::{
    config::cpu_indices_to_mask,
    error_codes::error_from_code_win32,
    logging::{Operation, is_new_error, log_to_find},
};

use std::collections::HashMap;
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE},
    System::{
        JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JOBOBJECT_BASIC_LIMIT_INFORMATION, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
            JOB_OBJECT_LIMIT_AFFINITY, JobObjectExtendedLimitInformation, SetInformationJobObject,
        },
        Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE, IO_COUNTERS},
    },
};
use windows::core::PCWSTR;

/// Manages Windows Job Objects for kernel-enforced CPU affinity limits.
///
/// Jobs are created as named objects using the raw config spec string
/// (e.g. `*ecore` → `_ecore`, `0-7` → `0-7`). This makes them human-
/// readable in tools like Process Explorer.
///
/// Job handles are cached by `(spec, affinity_mask)` so that changing
/// an alias definition on config reload properly updates the kernel-
/// enforced affinity mask on the existing job object.
///
/// All handles are closed at shutdown via `Drop`. Since we do NOT use
/// `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, closing handles does not
/// terminate assigned processes — they keep running with their affinity
/// limits intact (the named job object persists as long as processes
/// are assigned to it).
pub struct JobObjectManager {
    /// Cache key: (spec_string, affinity_mask) — must include mask so config
    /// reload with changed alias definitions can detect and update the limit.
    jobs: HashMap<(String, usize), HANDLE>,
}

impl JobObjectManager {
    pub fn new() -> Self {
        Self { jobs: HashMap::default() }
    }

    /// Returns a cached or newly-created job handle for the given spec + mask.
    ///
    /// The spec string (e.g. `*ecore`, `0-7`) is sanitized for use as a
    /// Windows kernel object name: `*` is replaced with `_` since `*` is
    /// a wildcard in some Windows APIs.
    ///
    /// If a job for this spec already exists but with a different mask
    /// (config reload with changed alias definition), the old cache entry
    /// is replaced and `SetInformationJobObject` is called to update the
    /// kernel affinity limit.
    fn get_or_create_job(
        &mut self,
        spec: &str,
        cpu_indices: &[u32],
        pid: u32,
        process_name: &str,
        errors: &mut Vec<String>,
    ) -> Option<HANDLE> {
        let affinity_mask = cpu_indices_to_mask(cpu_indices);
        if affinity_mask == 0 {
            if !cpu_indices.is_empty() {
                let msg = format!(
                    "JobObject: [WARNING] {:>5}-{} - All CPUs >= 64, affinity mask is 0 (not supported by single-group Job Object API). Job affinity NOT applied for spec '{}'.",
                    pid, process_name, spec
                );
                log_to_find(&msg);
                errors.push(msg);
            }
            return None;
        }

        let cache_key = (spec.to_string(), affinity_mask);

        // Fast path: exact cache hit
        if let Some(handle) = self.jobs.get(&cache_key) {
            return Some(*handle);
        }

        // Slow path: need to create or look up the named job object.
        // If there's an old entry for the same spec but different mask,
        // we'll replace it after creating/updating.
        let safe_name = spec.replace('*', "_");
        let job_name = format!("Local\\ProcGovernor_Job_{}", safe_name);
        let job_name_utf16: Vec<u16> = job_name.encode_utf16().chain(std::iter::once(0)).collect();

        let job_handle = match unsafe { CreateJobObjectW(None, PCWSTR::from_raw(job_name_utf16.as_ptr())) } {
            Ok(handle) => handle,
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                if is_new_error(pid, 0, process_name, Operation::CreateJobObject, error_code) {
                    let msg = format!(
                        "JobObject: [CREATE_JOB_OBJECT][{}] {:>5}-{} - '{}'",
                        error_from_code_win32(error_code),
                        pid,
                        process_name,
                        job_name
                    );
                    log_to_find(&msg);
                    errors.push(msg);
                }
                return None;
            }
        };

        if job_handle.is_invalid() {
            return None;
        }

        let limit_info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
            BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION {
                PerProcessUserTimeLimit: 0,
                PerJobUserTimeLimit: 0,
                LimitFlags: JOB_OBJECT_LIMIT_AFFINITY,
                MinimumWorkingSetSize: 0,
                MaximumWorkingSetSize: 0,
                ActiveProcessLimit: 0,
                Affinity: affinity_mask,
                PriorityClass: 0,
                SchedulingClass: 0,
            },
            IoInfo: IO_COUNTERS::default(),
            ProcessMemoryLimit: 0,
            JobMemoryLimit: 0,
            PeakProcessMemoryUsed: 0,
            PeakJobMemoryUsed: 0,
        };

        match unsafe {
            SetInformationJobObject(
                job_handle,
                JobObjectExtendedLimitInformation,
                &limit_info as *const _ as *const std::ffi::c_void,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        } {
            Ok(_) => {
                // Remove any stale entry for the same spec with a different mask
                self.jobs.retain(|(s, _), _| s != spec);
                self.jobs.insert(cache_key, job_handle);
                Some(job_handle)
            }
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                if is_new_error(pid, 0, process_name, Operation::SetInformationJobObject, error_code) {
                    let msg = format!(
                        "JobObject: [SET_INFORMATION_JOB_OBJECT][{}] {:>5}-{} - '{}'",
                        error_from_code_win32(error_code),
                        pid,
                        process_name,
                        job_name
                    );
                    log_to_find(&msg);
                    errors.push(msg);
                }
                unsafe { let _ = CloseHandle(job_handle); }
                None
            }
        }
    }

    /// Assigns a process to the job object identified by its config spec string.
    ///
    /// Opens a dedicated process handle with `PROCESS_SET_QUOTA | PROCESS_TERMINATE`
    /// (required by `AssignProcessToJobObject`).
    ///
    /// Note: Once a process is assigned to a job, it cannot be reassigned.
    /// Failure is expected if the process was already in another job (e.g.
    /// launched by a parent already under a job object). Errors are pushed
    /// into `errors` for per-process logging in `ApplyConfigResult`.
    pub fn assign_process(
        &mut self,
        pid: u32,
        spec: &str,
        cpu_indices: &[u32],
        process_name: &str,
        errors: &mut Vec<String>,
    ) -> bool {
        if cpu_indices.is_empty() {
            return true; // No job affinity configured, nothing to do
        }

        let job_handle = match self.get_or_create_job(spec, cpu_indices, pid, process_name, errors) {
            Some(h) => h,
            None => return false,
        };

        // Open process with required rights for job assignment
        let process_handle = unsafe { OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, false, pid) };

        let handle = match process_handle {
            Ok(h) if !h.is_invalid() => h,
            _ => {
                let error_code = if process_handle.is_err() {
                    unsafe { GetLastError().0 }
                } else {
                    u32::MAX // sentinel: handle was Ok but invalid; no real error code available
                };
                if is_new_error(pid, 0, process_name, Operation::OpenProcessForJobAssignment, error_code) {
                    let msg = format!(
                        "JobObject: [OPEN_PROCESS_FOR_JOB][{}] {:>5}-{}",
                        error_from_code_win32(error_code),
                        pid,
                        process_name
                    );
                    log_to_find(&msg);
                    errors.push(msg);
                }
                return false;
            }
        };

        match unsafe { AssignProcessToJobObject(job_handle, handle) } {
            Ok(_) => {
                unsafe { let _ = CloseHandle(handle); }
                true
            }
            Err(_) => {
                let error_code = unsafe { GetLastError().0 };
                if is_new_error(pid, 0, process_name, Operation::AssignProcessToJobObject, error_code) {
                    let msg = format!(
                        "JobObject: [ASSIGN_PROCESS_TO_JOB][{}] {:>5}-{}",
                        error_from_code_win32(error_code),
                        pid,
                        process_name
                    );
                    log_to_find(&msg);
                    errors.push(msg);
                }
                unsafe { let _ = CloseHandle(handle); }
                false
            }
        }
    }
}

impl Drop for JobObjectManager {
    fn drop(&mut self) {
        for (_, handle) in self.jobs.drain() {
            unsafe {
                let _ = CloseHandle(handle);
            }
        }
    }
}
