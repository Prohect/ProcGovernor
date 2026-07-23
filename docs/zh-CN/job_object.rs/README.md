# job_object 模块 (ProcGovernor)

`job_object` 模块通过 Windows Job Objects 提供内核强制的 CPU 亲和性限制。与 `apply` 模块中应用的每进程亲和性不同，Job Object 限制在内核级别生效，目标进程及其任何子进程均无法绕过。

## 结构体

| 名称 | 描述 |
|------|------|
| [JobObjectManager](JobObjectManager.md) | 管理命名的 Windows Job Objects，用于内核强制的 CPU 亲和性限制，支持句柄缓存和热重载。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| Job Object 亲和性应用 | [apply_job_object_affinity](../apply.rs/apply_job_object_affinity.md) |
| 进程级配置 | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) |
| 错误去重 | [is_new_error](../logging.rs/is_new_error.md) |
| Operation 枚举（新变体） | [Operation](../logging.rs/Operation.md) |
