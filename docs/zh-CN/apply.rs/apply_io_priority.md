# apply_io_priority 函数 (apply.rs)

使用未公开的 `NtQueryInformationProcess` 和 `NtSetInformationProcess` 原生 API，结合信息类 `ProcessInformationClassIOPriority`（33），获取和设置进程的 I/O 优先级。

## 语法

```ProcGovernor/src/apply.rs#L403-410
pub fn apply_io_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

包含所需 `io_priority` 设置的 [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) 的引用。如果 `io_priority` 为 `IOPriority::None`，函数将立即返回，不执行任何操作。

`dry_run`

如果为 **true**，函数将记录将要进行的更改到 [ApplyConfigResult](ApplyConfigResult.md)，而不调用任何 Windows API 来修改状态。

`process_handle`

目标进程的 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用。通过 [get_handles](get_handles.md) 提取读句柄（用于查询）和写句柄（用于设置）。

`apply_config_result`

[ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于累加变更描述和错误消息。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 累加器传递。

## 备注

此函数使用 NT 原生 API 而非已记录的 Win32 API，因为没有公开的 Win32 函数用于获取或设置每个进程的 I/O 优先级。

信息类常量 `PROCESS_INFORMATION_IO_PRIORITY`（值 **33**）在函数体内局部定义。

### 查询阶段

通过调用 `NtQueryInformationProcess` 并使用 `u32` 输出缓冲区来读取当前 I/O 优先级。检查 NTSTATUS 返回值：

- 如果为负数（失败），则通过 [log_error_if_new](log_error_if_new.md) 记录错误，操作为 `NtQueryInformationProcess2ProcessInformationIOPriority`，函数返回而不尝试设置。
- 如果为零或正数（成功），则将当前值与配置的目标值进行比较。

### 设置阶段

如果当前 I/O 优先级与配置值不同：

- 在 **试运行** 模式下，记录变更消息。
- 否则，调用 `NtSetInformationProcess` 并传入目标 I/O 优先级值。失败时，通过 [log_error_if_new](log_error_if_new.md) 记录 NTSTATUS 错误。成功时，记录格式为 `"IO Priority: {current} -> {target}"` 的变更消息。

### I/O 优先级值

[IOPriority](../priority.rs/IOPriority.md) 枚举映射到 NT 内核调度器使用的 Windows `IO_PRIORITY_HINT` 值：

| IOPriority | 值 | 效果 |
|---|---|---|
| VeryLow | 0 | 后台 I/O，最低调度优先级 |
| Low | 1 | 低于正常的 I/O 调度 |
| Normal | 2 | 默认 I/O 调度优先级 |

### 错误处理

查询和设置操作的错误均通过 [log_error_if_new](log_error_if_new.md) 去重，使用 `(pid, operation, error_code)` 键。NTSTATUS 代码通过 `error_from_ntstatus` 格式化以生成人类可读的错误消息。

### 句柄需求

读句柄需要 `PROCESS_QUERY_INFORMATION` 或 `PROCESS_QUERY_LIMITED_INFORMATION` 访问权限。写句柄需要 `PROCESS_SET_INFORMATION` 访问权限。如果任一句柄不可用，函数将提前返回。

## 需求

| | |
|---|---|
| **模块** | `apply` |
| **调用者** | 主应用循环（进程级强制执行） |
| **被调函数** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md)、`NtQueryInformationProcess`、`NtSetInformationProcess` |
| **API** | NT 原生 API (`ntdll.dll`) |
| **权限** | 对受保护的进程可能需要 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | 变更和错误的累加器 |
| [apply_memory_priority](apply_memory_priority.md) | 用于内存优先级的配套函数（使用已记录的 Win32 API） |
| [apply_priority](apply_priority.md) | 设置进程调度优先级类 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `io_priority` 字段的配置结构体 |
| [IOPriority](../priority.rs/IOPriority.md) | I/O 优先级级别枚举 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
