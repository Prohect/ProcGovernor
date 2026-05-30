# enable_inc_base_priority_privilege 函数 (winapi.rs)

在当前进程令牌上启用 `SeIncreaseBasePriorityPrivilege` 权限，允许服务将线程和进程优先级类提升到 `Normal` 以上。没有此权限，尝试设置 `High` 或 `Realtime` 优先级类将失败，返回 `ERROR_PRIVILEGE_NOT_HELD`。

## 语法

```rust
pub fn enable_inc_base_priority_privilege(no_inc_base_priority: bool)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `no_inc_base_priority` | `bool` | 如果为 `true`，函数记录一条消息，指示该权限已被 `-noIncBasePriority` CLI 标志禁用，并立即返回，不修改进程令牌。如果为 `false`，函数继续执行权限启用。 |

## 返回值

此函数不返回值。成功或失败通过日志消息传达。

## 备注

该函数遵循与 [enable_debug_privilege](enable_debug_privilege.md) 相同的三步权限启用模式：

1. **OpenProcessToken** — 以 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限打开当前进程令牌。
2. **LookupPrivilegeValueW** — 将 `SE_INC_BASE_PRIORITY_NAME` 权限名称解析为 `LUID`。
3. **AdjustTokenPrivileges** — 通过构造一个带有 `SE_PRIVILEGE_ENABLED` 的 `TOKEN_PRIVILEGES` 结构体并将其传递给 API 来启用该权限。

令牌句柄在每次操作后通过 `CloseHandle` 关闭，包括在错误路径上。

### 提前退出

当 `no_inc_base_priority` 为 `true` 时，函数记录 `"SeIncreaseBasePriorityPrivilege disabled by -noIncBasePriority flag"` 并返回，不打开令牌。这允许用户在受限上下文中运行服务时选择退出优先级提升。

### 错误处理

每个步骤在失败时记录描述性消息并提前返回：

| 失败点 | 日志消息 |
|---|---|
| `OpenProcessToken` | `"enable_inc_base_priority_privilege: self OpenProcessToken failed"` |
| `LookupPrivilegeValueW` | `"enable_inc_base_priority_privilege: LookupPrivilegeValueW failed"` |
| `AdjustTokenPrivileges` | `"enable_inc_base_priority_privilege: AdjustTokenPrivileges failed"` |
| 成功 | `"enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded"` |

当 `LookupPrivilegeValueW` 失败时，上一步打开的令牌句柄在返回前被关闭。

### 何时需要此权限？

Windows 需要 `SeIncreaseBasePriorityPrivilege` 才能将进程优先级类设置为 `HIGH_PRIORITY_CLASS` 或 `REALTIME_PRIORITY_CLASS`，或在某些场景下将线程优先级提升到 `THREAD_PRIORITY_NORMAL` 以上。服务在启动期间调用此函数，以便后续 [apply_priority](../apply.rs/apply_priority.md) 调用可以按配置中定义的那样设置提升的优先级类。

### 与 SeDebugPrivilege 的关系

此权限独立于 [SeDebugPrivilege](enable_debug_privilege.md)。`SeDebugPrivilege` 控制打开其他用户或受保护进程拥有的进程句柄的能力，而 `SeIncreaseBasePriorityPrivilege` 控制提升调度优先级的能力。两者通常在启动时启用，但可以通过 CLI 标志独立禁用。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | `src/main.rs` 中的主启动逻辑 |
| **被调用者** | 无（叶子函数；直接调用 Win32 API） |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)、[LookupPrivilegeValueW](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew)、[AdjustTokenPrivileges](https://learn.microsoft.com/zh-cn/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)、[GetCurrentProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)、[CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | 要求进程在持有 `SeIncreaseBasePriorityPrivilege` 的账户下运行（通常是 Administrators）。该函数*启用*该权限；如果账户不拥有它，它无法*授予*它。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 调试权限启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| 提权检查 | [is_running_as_admin](is_running_as_admin.md) |
| UAC 提权请求 | [request_uac_elevation](request_uac_elevation.md) |
| 优先级应用 | [apply_priority](../apply.rs/apply_priority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
