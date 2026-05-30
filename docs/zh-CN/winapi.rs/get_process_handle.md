# get_process_handle 函数 (winapi.rs)

为给定的进程 ID 以多个访问级别打开一组进程句柄。返回一个 [ProcessHandle](ProcessHandle.md) RAII 包装器，在释放时自动关闭所有句柄。该函数尝试以逐步提高的权限要求打开四个句柄；两个受限访问句柄是必需的，而两个完全访问句柄是可选的并会优雅降级。

## 语法

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。 |
| `process_name` | `&str` | 进程映像名称，仅用于错误日志记录。传递到 [is_new_error](../logging.rs/is_new_error.md) 进行错误去重。 |

## 返回值

`Option<ProcessHandle>` — 如果两个必需的受限访问句柄都成功打开，则返回 `Some(ProcessHandle)`。如果 `PROCESS_QUERY_LIMITED_INFORMATION` 或 `PROCESS_SET_LIMITED_INFORMATION` 无法获取，则返回 `None`。

当返回 `Some` 时，返回的 [ProcessHandle](ProcessHandle.md) 具有以下保证：

| 字段 | 保证 |
|------|------|
| `r_limited_handle` | 始终有效 (`PROCESS_QUERY_LIMITED_INFORMATION`) |
| `w_limited_handle` | 始终有效 (`PROCESS_SET_LIMITED_INFORMATION`) |
| `r_handle` | 如果 `PROCESS_QUERY_INFORMATION` 成功则为 `Some(HANDLE)`，否则为 `None` |
| `w_handle` | 如果 `PROCESS_SET_INFORMATION` 成功则为 `Some(HANDLE)`，否则为 `None` |

## 备注

### 句柄获取顺序

该函数按以下顺序打开句柄，如果需要句柄失败则停止并返回 `None`：

| 步骤 | 访问权限 | 必需 | 内部错误码 | 失败时 |
|------|------|------|------|------|
| 1 | `PROCESS_QUERY_LIMITED_INFORMATION` | **是** | `0` | 通过 [is_new_error](../logging.rs/is_new_error.md) 记录，返回 `None` |
| 2 | `PROCESS_SET_LIMITED_INFORMATION` | **是** | `1` | 关闭步骤 1 句柄，记录，返回 `None` |
| 3 | `PROCESS_QUERY_INFORMATION` | 否 | `2` | 设置 `r_handle = None`，继续 |
| 4 | `PROCESS_SET_INFORMATION` | 否 | `3` | 设置 `w_handle = None`，继续 |

步骤 3 和 4 需要更高的权限（对于受保护进程通常需要 SeDebugPrivilege）。对于系统进程，它们的失败是预期的，并被静默吸收——源代码中这些步骤的错误日志记录已被注释掉。

### 错误去重

必需句柄（步骤 1–2）的失败仅在首次看到唯一的 `(pid, error_code)` 组合时通过 [is_new_error](../logging.rs/is_new_error.md) 记录。这可以防止在多次轮询迭代中重复遇到受保护进程时日志泛滥。

### 无效句柄检查

每次成功的 `OpenProcess` 调用后，返回的句柄会通过 `is_invalid()` 检查。尽管 API 返回成功但句柄无效的情况被视为带有自己 `Operation::InvalidHandle` 错误码的独特失败情况，确保与操作系统级别的错误分开记录。

### 部分失败时的句柄清理

如果步骤 1 成功但步骤 2 失败，步骤 1 的句柄在返回 `None` 之前被显式关闭。这可以防止在早期退出路径上的句柄泄漏。当完整的 [ProcessHandle](ProcessHandle.md) 被构造并返回时，其 `Drop` 实现处理清理。

### 调用者的句柄选择

下游函数（例如，[get_handles](../apply.rs/get_handles.md)）在可用时优先使用完全访问句柄（`r_handle`、`w_handle`），回退到受限句柄。这种分层方法允许服务在受保护进程上以降低的能力运行，而不是完全失败。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [apply.rs](../apply.rs/README.md)（主应用循环按进程打开句柄） |
| **被调用者** | `OpenProcess`（Win32）、[is_new_error](../logging.rs/is_new_error.md)、[log_to_find](../logging.rs/log_to_find.md)、[error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)、[GetLastError](https://learn.microsoft.com/zh-cn/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)、[CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | 对大多数进程，受限句柄不需要特殊权限。完全句柄（`PROCESS_QUERY_INFORMATION`、`PROCESS_SET_INFORMATION`）对受保护/系统进程需要 [SeDebugPrivilege](enable_debug_privilege.md)。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 此函数返回的 RAII 句柄包装器 | [ProcessHandle](ProcessHandle.md) |
| 线程句柄等效函数 | [get_thread_handle](get_thread_handle.md) |
| apply 模块中的句柄访问器 | [get_handles](../apply.rs/get_handles.md) |
| 调试权限启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| 模块概述 | [winapi.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
