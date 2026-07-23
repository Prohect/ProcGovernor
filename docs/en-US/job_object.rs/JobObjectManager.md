# JobObjectManager struct (job_object.rs)

Manages Windows Job Objects for kernel-enforced CPU affinity limits. Jobs are created as named objects using the raw config spec string (e.g. `*ecore` → `_ecore`, `0-7` → `0-7`), making them human-readable in tools like Process Explorer.

## Syntax

```ProcGovernor/src/job_object.rs#L20-39
pub struct JobObjectManager {
    jobs: HashMap<(String, usize), HANDLE>,
}
```

## Fields

`jobs: HashMap<(String, usize), HANDLE>`

A cache of job object handles keyed by `(spec_string, affinity_mask)`. The mask is included in the key so that config reload with changed alias definitions can detect and update the kernel-enforced affinity limit on the existing job object.

## Methods

### new

```rust
pub fn new() -> Self
```

Creates an empty `JobObjectManager` with no cached job handles.

---

### get_or_create_job

```rust
fn get_or_create_job(
    &mut self,
    spec: &str,
    cpu_indices: &[u32],
    pid: u32,
    process_name: &str,
    errors: &mut Vec<String>,
) -> Option<HANDLE>
```

Returns a cached or newly-created job handle for the given spec string + CPU mask.

**Behavior:**

1. Converts `cpu_indices` to an affinity mask via [`cpu_indices_to_mask`](../config.rs/cpu_indices_to_mask.md). If the mask is 0 (all CPUs ≥64), a warning is logged and no job is created (the single-group Job Object API only supports ≤64 logical processors).
2. Checks the cache for an exact `(spec, mask)` hit. If found, returns the cached handle immediately.
3. Creates or opens a named job object: `Local\ProcGovernor_Job_{sanitized_spec}`. The `*` character in specs is replaced with `_` since `*` is a wildcard in some Windows APIs.
4. Sets `JOB_OBJECT_LIMIT_AFFINITY` on the job to enforce the CPU affinity mask.
5. On success, replaces any stale cache entry for the same spec (with a different mask) and caches the new handle.
6. On failure, logs the error once per unique `(pid, operation, error_code)` via [`is_new_error`](../logging.rs/is_new_error.md) and closes the handle.

---

### assign_process

```rust
pub fn assign_process(
    &mut self,
    pid: u32,
    spec: &str,
    cpu_indices: &[u32],
    process_name: &str,
    errors: &mut Vec<String>,
) -> bool
```

Assigns a process to the job object identified by its config spec string.

**Behavior:**

1. Returns `true` immediately if `cpu_indices` is empty (no job affinity configured).
2. Calls `get_or_create_job` to obtain the job handle. Returns `false` if that fails.
3. Opens the target process with `PROCESS_SET_QUOTA | PROCESS_TERMINATE` (required by `AssignProcessToJobObject`).
4. Calls `AssignProcessToJobObject` to assign the process to the job.
5. Closes the process handle on both success and failure paths.
6. Returns `true` on success, `false` on failure.

**Important:** Once a process is assigned to a job, it cannot be reassigned. Failure is expected if the process was already in another job (e.g. launched by a parent already under a job object).

## Drop Implementation

```rust
impl Drop for JobObjectManager {
    fn drop(&mut self) { ... }
}
```

Closes all cached job object handles at shutdown. Since `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` is NOT used, closing handles does not terminate assigned processes — they keep running with their affinity limits intact (the named job object persists as long as processes are assigned to it).

## Requirements

| | |
|---|---|
| **Module** | `src/job_object.rs` |
| **Callers** | [`apply_job_object_affinity`](../apply.rs/apply_job_object_affinity.md), main polling loop |
| **Callees** | [`cpu_indices_to_mask`](../config.rs/cpu_indices_to_mask.md), [`is_new_error`](../logging.rs/is_new_error.md), [`log_to_find`](../logging.rs/log_to_find.md), [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | [`CreateJobObjectW`](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-createjobobjectw), [`SetInformationJobObject`](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject), [`AssignProcessToJobObject`](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-assignprocesstojobobject), [`OpenProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | `PROCESS_SET_QUOTA`, `PROCESS_TERMINATE` (for process open); admin recommended |

## See Also

| Topic | Description |
|---|---|
| [apply_job_object_affinity](../apply.rs/apply_job_object_affinity.md) | The apply function that calls `assign_process` |
| [apply_affinity](../apply.rs/apply_affinity.md) | Soft per-process CPU affinity (contrast) |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | Config struct with `job_object_affinity_spec` and `job_object_affinity_cpus` |
| [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) | CPU index list → affinity bitmask conversion |
| [Operation](../logging.rs/Operation.md) | Error operation codes: `CreateJobObject`, `SetInformationJobObject`, `AssignProcessToJobObject`, `OpenProcessForJobAssignment` |
