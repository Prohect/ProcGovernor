# enable_debug_privilege 函数 (winapi.rs)

在当前进程令牌上启用 `SeDebugPrivilege` 权限。此权限允许服务打开受保护和系统进程的句柄，这些进程通常会拒绝访问，从而能够对所有运行中的进程进行完整的进程/线程检查和修改。

## 语法

```rust
pub fn enable_debug_privilege(no_debug_priv: bool)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `no_debug_priv` | `bool` | 如果为 `true`，函数记录一条消息并立即返回，不尝试启用该权限。由 `-noDebugPriv` CLI 标志控制。 |

## 返回值

此函数不返回值。成功或失败通过日志消息传达。

## 备注

该函数执行以下 Win32 API 调用序列以启用权限：

1. **OpenProcessToken** — 以 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限打开当前进程令牌（`GetCurrentProcess()`）。
2. **LookupPrivilegeValueW** — 将 `SE_DEBUG_NAME` 字符串常量解析为在本地系统上标识 `SeDebugPrivilege` 的 `LUID`。
3. **AdjustTokenPrivileges** — 通过传递一个带有在已解析 LUID 上设置 `SE_PRIVILEGE_ENABLED` 的 `TOKEN_PRIVILEGES` 结构体来启用该权限。
4. **CloseHandle** — 在调整后关闭令牌句柄，无论成功或失败。

### 失败行为

每个 API 调用被单独检查。如果任何调用失败，函数记录一条描述性错误消息并提前返回。令牌句柄如果成功打开则始终关闭。失败不是致命的——服务以降低的能力继续运行（无法打开某些受保护进程的句柄）。

### CLI 标志上的提前退出

当 `no_debug_priv` 为 `true`（用户在命令行上传入了 `-noDebugPriv`）时，函数记录 `"SeDebugPrivilege disabled by -noDebugPriv flag"` 并返回，不修改令牌。这允许用户以降低的权限运行服务以进行测试或在无法授予该权限的受限环境中运行。

### 何时需要此权限？

没有 `SeDebugPrivilege`，针对其他用户拥有的进程、系统进程或受保护进程的 `OpenProcess` 和 `OpenThread` 调用将失败，返回 `ERROR_ACCESS_DENIED` (5)。启用该权限允许服务：

- 打开所有进程的句柄（包括 `csrss.exe`、`lsass.exe`、`System` 等）
- 查询和修改系统上任何线程的线程调度参数
- 枚举受保护进程中的模块以进行地址解析

### 权限与提权

`SeDebugPrivilege` 通常存在于提权管理员进程的令牌中，但默认情况下是禁用的。此函数*启用*一个已存在的权限——它不授予令牌没有的权限。以非管理员身份运行且令牌中没有该权限将导致 `AdjustTokenPrivileges` 成功，但权限实际上不会启用（此条件可通过 `GetLastError() == ERROR_NOT_ALL_ASSIGNED` 检测，但此处不执行此检查）。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | `src/main.rs` 中的主启动逻辑 |
| **被调用者** | 无（叶子函数） |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)、[LookupPrivilegeValueW](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew)（`SE_DEBUG_NAME`）、[AdjustTokenPrivileges](https://learn.microsoft.com/zh-cn/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)、[GetCurrentProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)、[CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | 要求调用进程令牌已包含 `SeDebugPrivilege`（对于提升的管理员令牌是标准的）。该函数启用它；无法添加它。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 配套权限启用 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| 提权检查 | [is_running_as_admin](is_running_as_admin.md) |
| UAC 提权请求 | [request_uac_elevation](request_uac_elevation.md) |
| 进程句柄打开（受益于此权限） | [get_process_handle](get_process_handle.md) |
| 线程句柄打开（受益于此权限） | [get_thread_handle](get_thread_handle.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
