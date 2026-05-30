# ProcessHandle 结构体 (winapi.rs)

围绕为单个进程以不同访问级别打开的多个 Windows `HANDLE` 值的 RAII 包装器。提供受限和完全访问级别的读写句柄。所有有效句柄在结构体被释放时通过 `CloseHandle` 自动关闭。

## 语法

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `r_limited_handle` | `HANDLE` | 以 `PROCESS_QUERY_LIMITED_INFORMATION` 访问权限打开的进程句柄。当结构体存在时始终有效——如果此句柄无法获取，构造将失败。用于轻量级查询，如 `QueryFullProcessImageNameW` 和不需要完全查询权限的周期时间查询。 |
| `r_handle` | `Option<HANDLE>` | 以 `PROCESS_QUERY_INFORMATION` 访问权限打开的进程句柄。如果访问权限被拒绝则为 `None`（对于受保护/系统进程很常见）。对于 `GetProcessAffinityMask` 和 `NtQueryInformationProcess` 等操作是必需的。 |
| `w_limited_handle` | `HANDLE` | 以 `PROCESS_SET_LIMITED_INFORMATION` 访问权限打开的进程句柄。当结构体存在时始终有效。用于通过 `SetProcessDefaultCpuSets` 进行 CPU 集合分配。 |
| `w_handle` | `Option<HANDLE>` | 以 `PROCESS_SET_INFORMATION` 访问权限打开的进程句柄。如果访问权限被拒绝则为 `None`。对于 `SetProcessAffinityMask`、`SetPriorityClass` 和 `NtSetInformationProcess` 等操作是必需的。 |

## Drop

```rust
impl Drop for ProcessHandle {
    fn drop(&mut self);
}
```

通过 `CloseHandle` 关闭所有持有的句柄：

- `r_limited_handle` 和 `w_limited_handle` 始终被关闭（它们始终有效）。
- `r_handle` 和 `w_handle` 仅在为 `Some(_)` 时才被关闭。

每个 `CloseHandle` 调用的结果被有意丢弃（`let _ = CloseHandle(...)`），因为清理期间的句柄关闭失败是不可恢复的，不应触发 panic。

## 备注

### 访问级别层级

`ProcessHandle` 结构体捕获两个访问层级的四个句柄：

| 层级 | 读取 | 写入 |
|------|------|------|
| **受限** | `PROCESS_QUERY_LIMITED_INFORMATION` | `PROCESS_SET_LIMITED_INFORMATION` |
| **完全** | `PROCESS_QUERY_INFORMATION` | `PROCESS_SET_INFORMATION` |

受限访问句柄对大多数进程（包括其他会话中的进程）都能成功。完全访问句柄对于受保护进程、系统进程或安全描述符拒绝请求账户的进程可能会失败。[get_process_handle](get_process_handle.md) 工厂函数通过将完全访问句柄包装在 `Option` 中来优雅地处理此问题。

### 调用者中的句柄选择

[get_handles](../apply.rs/get_handles.md) 辅助函数从 `ProcessHandle` 中提取 `(read_handle, write_handle)` 对，在可用时优先使用完全访问句柄，并在不可用时回退到受限访问句柄。这允许应用函数以最佳可用访问级别工作，而无需自己检查 `Option`。

### 有效性保证

- **构造：** `ProcessHandle` 仅由 [get_process_handle](get_process_handle.md) 创建，如果任一受限访问句柄打开失败，它将返回 `None`。如果结构体存在，`r_limited_handle` 和 `w_limited_handle` 都保证有效。
- **生命周期：** 调用者（通常是主轮询循环）拥有 `ProcessHandle`，并在进程不再被跟踪时将其释放。
- **线程安全：** 由于原始 `HANDLE` 值，`ProcessHandle` 默认不是 `Send` 或 `Sync`。它在主轮询循环的单线程上下文中使用。

### 错误去重

当构造期间句柄打开失败时，错误通过 [is_new_error](../logging.rs/is_new_error.md) 记录，使得同一进程/操作/错误码组合的重复失败在第一次出现后被抑制。内部操作码映射到特定句柄：

| 操作码 | 句柄 |
|------|------|
| `0` | `r_limited_handle` (`PROCESS_QUERY_LIMITED_INFORMATION`) |
| `1` | `w_limited_handle` (`PROCESS_SET_LIMITED_INFORMATION`) |
| `2` | `r_handle` (`PROCESS_QUERY_INFORMATION`) |
| `3` | `w_handle` (`PROCESS_SET_INFORMATION`) |

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | 主轮询循环、[get_handles](../apply.rs/get_handles.md)、所有 `apply_*` 函数 |
| **工厂** | [get_process_handle](get_process_handle.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)、[CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | [SeDebugPrivilege](enable_debug_privilege.md) 扩展对受保护进程的访问 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 工厂函数 | [get_process_handle](get_process_handle.md) |
| 句柄提取辅助函数 | [get_handles](../apply.rs/get_handles.md) |
| 线程句柄对应物 | [ThreadHandle](ThreadHandle.md) |
| 错误去重 | [is_new_error](../logging.rs/is_new_error.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
