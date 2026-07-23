# job_object module (ProcGovernor)

The `job_object` module provides kernel-enforced CPU affinity limits via Windows Job Objects. Unlike the per-process affinity applied in the `apply` module, job object limits are enforced at the kernel level and cannot be bypassed by the target process or any of its children.

## Structs

| Name | Description |
|------|-------------|
| [JobObjectManager](JobObjectManager.md) | Manages named Windows Job Objects for kernel-enforced CPU affinity limits with handle caching and hot-reload support. |

## See Also

| Topic | Link |
|-------|------|
| Job Object affinity application | [apply_job_object_affinity](../apply.rs/apply_job_object_affinity.md) |
| Process-level config | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) |
| Error deduplication | [is_new_error](../logging.rs/is_new_error.md) |
| Operation enum (new variants) | [Operation](../logging.rs/Operation.md) |
