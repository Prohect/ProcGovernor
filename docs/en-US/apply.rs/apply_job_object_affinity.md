# apply_job_object_affinity function (apply.rs)

Applies a kernel-enforced job object CPU affinity limit. Unlike `apply_affinity` which uses per-process `SetProcessAffinityMask`, job objects prevent the process AND its children from ever running on CPUs outside the specified mask.

## Syntax

```ProcGovernor/src/apply.rs#L134-160
pub fn apply_job_object_affinity(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    job_manager: &mut JobObjectManager,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid: u32`

The process identifier of the target process.

`config: &ProcessLevelConfig`

The process-level configuration containing `job_object_affinity_cpus` (the CPU indices to enforce) and `job_object_affinity_spec` (the raw spec string used to name the job object). If `job_object_affinity_cpus` is empty, the function returns immediately without action.

`dry_run: bool`

If `true`, records the intended change in *apply_config_result* without calling any Windows APIs or modifying job objects.

`job_manager: &mut JobObjectManager`

The job object manager that caches and manages named Windows Job Objects. See [`JobObjectManager`](../job_object.rs/JobObjectManager.md).

`apply_config_result: &mut ApplyConfigResult`

Accumulator for change messages and errors produced during the operation.

## Return value

This function does not return a value. Results are communicated through `apply_config_result`.

## Remarks

This function delegates to [`JobObjectManager::assign_process`](../job_object.rs/JobObjectManager.md#assign_process), which handles job object creation, caching, and process assignment. The job object is named using the raw config spec (e.g. `*ecore` → `Local\ProcGovernor_Job__ecore`), making it human-readable in tools like Process Explorer.

### Side effects

- **Creates a named job object** on first use for a given affinity spec.
- **Assigns the process to the job** via `AssignProcessToJobObject`. Once assigned, a process cannot be reassigned to another job.
- **Updates the job's affinity limit** on config reload if the CPU mask changed (e.g. alias redefinition).

### Error handling

Errors from job object creation, affinity limit configuration, process handle opening, and process assignment are pushed into `apply_config_result.errors`. Each error is deduplicated via [`is_new_error`](../logging.rs/is_new_error.md) per unique `(pid, operation, error_code)` combination.

Failure to assign a process to a job (e.g. because it was already in another job) does not prevent other rule fields from being applied.

### Change message format

```/dev/null/example.txt#L1
Job Affinity: -> [0, 1, 2, 3]
```

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Callers** | [`apply_process_level`](../main.rs/apply_process_level.md) in the main polling loop |
| **Callees** | [`JobObjectManager::assign_process`](../job_object.rs/JobObjectManager.md), [`format_cpu_indices`](../config.rs/format_cpu_indices.md) |
| **Win32 API** | (Delegated to `JobObjectManager`) |
| **Privileges** | Admin (required for `AssignProcessToJobObject`; elevated privileges recommended) |

## See Also

| Topic | Description |
|---|---|
| [JobObjectManager](../job_object.rs/JobObjectManager.md) | The job object manager that creates, caches, and assigns job objects |
| [apply_affinity](apply_affinity.md) | Soft per-process CPU affinity (contrast with kernel-enforced job affinity) |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | Configuration struct containing `job_object_affinity_spec` and `job_object_affinity_cpus` |
| [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) | Converts CPU index lists to affinity bitmasks |
| [is_new_error](../logging.rs/is_new_error.md) | Error deduplication used by job object assignment |
