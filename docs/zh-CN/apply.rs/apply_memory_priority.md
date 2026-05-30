# apply_memory_priority 函数 (apply.rs)

使用已记录的 `GetProcessInformation` / `SetProcessInformation` Windows API 和 `ProcessMemoryPriority` 信息类，将进程的内存优先级设置为配置中指定的值。

## 语法

```ProcGovernor/src/apply.rs#L491-498
pub fn apply_memory_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

包含所需 `memory_priority` 设置的 [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)。如果 `memory_priority` 为 `MemoryPriority::None`，函数将立即返回，不执行任何操作。

`dry_run: bool`

当为 **true** 时，函数将记录*将要*进行的更改到 `apply_config_result`，而不调用任何 Windows API 来修改进程。当为 **false** 时，更改将被应用。

`process_handle: &ProcessHandle`

一个 [ProcessHandle](../winapi.rs/ProcessHandle.md)，通过 [get_handles](get_handles.md) 从中提取读和写 `HANDLE` 值。两个句柄均需要；如果任一不可用，函数将提前返回。

`apply_config_result: &mut ApplyConfigResult`

一个 [ApplyConfigResult](ApplyConfigResult.md) 累加器，用于收集执行过程中产生的变更描述和错误消息。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 传递。

## 备注

### 算法

1. 通过 [get_handles](get_handles.md) 从 `process_handle` 提取读和写操作系统句柄。如果任一句柄缺失，则提前返回。
2. 检查 `config.memory_priority` 是否映射到有效的 Windows 常量。如果配置的值为 `None`，函数为无操作。
3. 通过调用 `GetProcessInformation`（使用 `ProcessMemoryPriority` 和 `MemoryPriorityInformation` 结构体）查询当前内存优先级。
4. 如果查询失败，通过 [log_error_if_new](log_error_if_new.md) 记录错误，操作为 `Operation::GetProcessInformation2ProcessMemoryPriority`，然后返回。
5. 将当前值与目标值进行比较。如果已经匹配，则不执行任何操作。
6. 在试运行模式下，记录预定的更改并返回。
7. 构建一个新的 `MemoryPriorityInformation`（带有目标值），并调用 `SetProcessInformation`（使用 `ProcessMemoryPriority`）。
8. 成功时，记录变更 `"Memory Priority: <old> -> <new>"`。
9. 失败时，通过 [log_error_if_new](log_error_if_new.md) 记录 Win32 错误，操作为 `Operation::SetProcessInformation2ProcessMemoryPriority`。

### 内存优先级级别

内存优先级控制在内存压力下内存管理器裁剪和重新利用进程页面的激进程度。Windows 定义的级别对应于 [MemoryPriority](../priority.rs/MemoryPriority.md) 中的值：

| 级别 | 数值 | 行为 |
|---|---|---|
| VeryLow | 1 | 页面最先被裁剪和重新利用。 |
| Low | 2 | 页面在 VeryLow 之后、Normal 之前被裁剪。 |
| MediumLow | 3 | 中间优先级。 |
| Medium | 4 | 中间优先级。 |
| Normal | 5 | 默认优先级 — 页面最后被裁剪。 |

### MemoryPriorityInformation 包装器

该函数使用 `MemoryPriorityInformation(u32)` 新类型包装器封装原始 `MEMORY_PRIORITY_INFORMATION` 值，以便与 Windows `ProcessMemoryPriority` 信息类接口。这使结构体布局与 `GetProcessInformation` / `SetProcessInformation` 期望的兼容。

### 错误处理

错误通过 [log_error_if_new](log_error_if_new.md) 报告，按 `(pid, operation, error_code)` 去重消息，防止对反复失败的进程产生日志刷屏。查询和设置路径都有独立的错误日志记录。

### 平台说明

- 此函数面向 **Windows 8 / Windows Server 2012** 及更高版本，其中 `ProcessMemoryPriority` 的 `GetProcessInformation` / `SetProcessInformation` 可用。
- 调用进程必须对目标进程持有适当的访问权限。写句柄需要 `PROCESS_SET_INFORMATION` 权限，读句柄需要 `PROCESS_QUERY_LIMITED_INFORMATION` 权限。

## 需求

| | |
|---|---|
| **源模块** | [apply.rs](README.md) |
| **调用者** | 主应用循环（每进程强制执行周期） |
| **被调函数** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md)、`GetProcessInformation`、`SetProcessInformation` |
| **Windows API** | `GetProcessInformation` (`ProcessMemoryPriority`)、`SetProcessInformation` (`ProcessMemoryPriority`)、`GetLastError` |
| **权限** | `PROCESS_QUERY_LIMITED_INFORMATION`（读）、`PROCESS_SET_INFORMATION`（写） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | 变更和错误的累加器 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `memory_priority` 的每进程配置结构体 |
| [MemoryPriority](../priority.rs/MemoryPriority.md) | 定义内存优先级级别的枚举 |
| [apply_io_priority](apply_io_priority.md) | 设置 IO 优先级的配套函数 |
| [apply_priority](apply_priority.md) | 设置进程（CPU 调度）优先级的配套函数 |
| [ProcessHandle](../winapi.rs/ProcessHandle.md) | 提供对进程读/写访问的句柄包装器 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
