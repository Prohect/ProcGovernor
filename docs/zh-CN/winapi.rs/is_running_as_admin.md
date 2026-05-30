# is_running_as_admin 函数 (winapi.rs)

通过查询进程令牌的提权状态来检查当前进程是否以管理员（提权）权限运行。

## 语法

```rust
pub fn is_running_as_admin() -> bool
```

## 参数

此函数不接受任何参数。

## 返回值

`bool` — 如果当前进程令牌指示进程已提权（以管理员身份运行），则为 `true`；否则为 `false`，包括任何中间 API 调用失败时。

## 备注

该函数使用以下 Win32 API 调用序列查询当前进程令牌：

1. **OpenProcessToken** — 使用 `TOKEN_QUERY` 访问权限打开当前进程（`GetCurrentProcess()`）的令牌。
2. **GetTokenInformation** — 查询 `TokenElevation` 信息类，填充 `TOKEN_ELEVATION` 结构体。
3. **CloseHandle** — 在查询后关闭令牌句柄。

结果由 `TOKEN_ELEVATION` 结构体的 `TokenIsElevated` 字段确定。非零值表示进程已提权。

### 失败行为

如果 `OpenProcessToken` 或 `GetTokenInformation` 失败，函数返回 `false` 而不是传播错误。此保守默认值确保当令牌检查不可行时，服务将自身视为未提权，这会通过 [request_uac_elevation](request_uac_elevation.md) 触发 UAC 提权流程。

### 服务中的用法

此函数在服务启动早期调用，以确定是否需要 UAC 提权。如果 `is_running_as_admin()` 返回 `false` 且未设置 `noUAC` CLI 标志，服务将通过 [request_uac_elevation](request_uac_elevation.md) 启动自身的提权副本并退出当前（未提权）进程。

### 令牌提权与权限检查

此函数检查令牌*提权状态*，而不是特定权限（如 `SeDebugPrivilege`）是否已启用。提权表示进程是通过 UAC 同意提示启动的，或者以高完整性令牌运行。提权后，各个权限仍必须显式启用——参见 [enable_debug_privilege](enable_debug_privilege.md) 和 [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md)。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | `src/main.rs` 中的主启动逻辑 |
| **被调用者** | 无（叶子函数） |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)、[GetTokenInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)（`TokenElevation`）、[GetCurrentProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)、[CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | 无要求——对进程自身令牌的 `TOKEN_QUERY` 访问始终可用 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| UAC 提权请求 | [request_uac_elevation](request_uac_elevation.md) |
| 调试权限启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| 基础优先级权限启用 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
