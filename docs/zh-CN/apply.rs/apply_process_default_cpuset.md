# apply_process_default_cpuset 函数 (apply.rs)

使用 Windows CPU 集合 API 为进程设置默认 CPU 集合，提供一个软性 CPU 偏好，调度器在不硬性限制线程执行的情况下遵守该偏好。

## 语法

```ProcGovernor/src/apply.rs#L298-308
pub fn apply_process_default_cpuset<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程 ID。

`config: &ProcessLevelConfig`

包含 `cpu_set_cpus`（所需的 CPU 索引）和 `cpu_set_reset_ideal`（是否在应用 CPU 集合后重置理想处理器）的进程级配置。

`dry_run: bool`

如果为 `true`，在 `apply_config_result` 中记录将要更改的内容，而不调用任何 Windows API。

`process_handle: &ProcessHandle`

目标进程的句柄包装器。需要读句柄和写句柄。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

进程线程映射的惰性访问器。仅当 `cpu_set_reset_ideal` 为 `true` 且应用了更改时才进行评估，此时它被转发给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。

`apply_config_result: &mut ApplyConfigResult`

操作过程中产生的变更消息和错误消息的累加器。

## 返回值

无。结果累加到 `apply_config_result` 中。

## 备注

与通过 `SetProcessAffinityMask` 设置的硬亲和性掩码不同，CPU 集合提供一个**软性偏好**。Windows 调度器偏好指定的 CPU，但在负载下可能在其它 CPU 上调度线程。这使得 CPU 集合成为现代 Windows 上工作负载引导的首选机制。

### 算法

1. **提前退出** — 如果 `config.cpu_set_cpus` 为空或全局 CPU 集合信息缓存为空，立即返回。
2. **试运行** — 如果 `dry_run` 为 `true`，记录预定的 CPU 集合并返回。
3. **转换索引** — 通过 `cpusetids_from_indices` 将配置的 CPU 索引转换为 Windows CPU 集合 ID。
4. **查询当前值** — 首先使用 `None` 缓冲区调用 `GetProcessDefaultCpuSets`：
   - 如果成功，进程没有分配默认 CPU 集合，因此需要更改。
   - 如果失败且错误代码为 **122** (`ERROR_INSUFFICIENT_BUFFER`)，进程已有 CPU 集合。第二次使用正确大小的缓冲区调用来检索当前集合 ID 以进行比较。
   - 任何其它错误通过 [log_error_if_new](log_error_if_new.md) 记录。
5. **比较** — 如果当前 CPU 集合 ID 与目标匹配，不执行任何操作。
6. **重置理想处理器（可选）** — 如果 `config.cpu_set_reset_ideal` 为 `true`，在应用新 CPU 集合*之前*使用 `config.cpu_set_cpus` 调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。这防止过时的理想处理器分配覆盖新的 CPU 偏好。
7. **应用** — 使用目标 CPU 集合 ID 调用 `SetProcessDefaultCpuSets`。
8. **日志记录** — 成功时，记录一条显示从旧到新 CPU 索引转换的变更消息。失败时，记录错误。

### 两阶段查询模式

`GetProcessDefaultCpuSets` API 使用一种常见的 Windows 模式，其中第一次调用确定所需的缓冲区大小。错误代码 122 (`ERROR_INSUFFICIENT_BUFFER`) 是一个预期条件而非真正的错误，并触发第二次使用适当大小缓冲区的调用。

### 与亲和性掩码的交互

CPU 集合和亲和性掩码是独立的机制。一个进程可以同时具有硬亲和性掩码和默认 CPU 集合。有效调度取决于 Windows 内部逻辑，但通常亲和性掩码作为硬约束优先，而 CPU 集合在该约束内作为提示起作用。

## 需求

| | |
|---|---|
| **模块** | [apply.rs](README.md) |
| **调用者** | `main.rs` 强制执行循环 |
| **被调函数** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md)、`cpusetids_from_indices`、`indices_from_cpusetids`、`format_cpu_indices` |
| **Win32 API** | [GetProcessDefaultCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets)、[SetProcessDefaultCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets)、[GetLastError](https://learn.microsoft.com/zh-cn/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| **权限** | 需要目标进程的 `PROCESS_QUERY_LIMITED_INFORMATION`（读）和 `PROCESS_SET_LIMITED_INFORMATION`（写）访问权限 |
| **最低操作系统** | Windows 10 版本 1607（CPU 集合 API） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [apply_affinity](apply_affinity.md) | 硬亲和性掩码替代方案 |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | 在 CPU 集合更改后重新分配线程理想处理器 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `cpu_set_cpus` 和 `cpu_set_reset_ideal` 字段的配置结构体 |
| [ApplyConfigResult](ApplyConfigResult.md) | 变更和错误的累加器 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
