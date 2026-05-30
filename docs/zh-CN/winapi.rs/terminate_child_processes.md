# terminate_child_processes 函数 (winapi.rs)

终止当前进程生成的所有子进程。在服务启动期间调用，以清理孤立的子进程——特别是 [request_uac_elevation](request_uac_elevation.md) 中的 UAC 提权流程留下的 PowerShell 控制台宿主进程。

## 语法

```rust
pub fn terminate_child_processes()
```

## 参数

此函数不接受任何参数。它操作当前进程（通过 `GetCurrentProcessId` 标识）。

## 返回值

此函数不返回值。快照创建、进程枚举或终止期间的失败被记录但不会作为错误传播。

## 备注

该函数使用 Windows Tool Help 库枚举系统上的所有进程，并识别那些 `th32ParentProcessID` 与当前进程 PID 匹配的进程。对于找到的每个子进程，它以 `PROCESS_TERMINATE` 访问权限打开该进程，并以退出码 `0` 调用 `TerminateProcess`。

### 算法

1. **GetCurrentProcessId** — 获取当前（服务）进程的 PID。
2. **CreateToolhelp32Snapshot** — 创建所有运行进程的快照（`TH32CS_SNAPPROCESS`）。如果快照失败，函数静默返回。
3. **Process32FirstW / Process32NextW** — 遍历快照中的每个进程。
4. 对于每个 `th32ParentProcessID == current_pid` 的进程条目：
   - 从以 null 结尾的 `szExeFile` 字段提取子进程名称。
   - 通过 `OpenProcess` 以 `PROCESS_TERMINATE` 访问权限打开子进程。
   - 以退出码 `0` 调用 `TerminateProcess`。
   - 通过 `CloseHandle` 关闭进程句柄。
   - 记录每个步骤的成功或失败。
5. **CloseHandle** — 在迭代完成后关闭快照句柄。

### 日志记录

每次终止尝试产生一条日志消息：

| 结果 | 日志消息格式 |
|------|------|
| 成功 | `"terminate_child_processes: terminated '{name}' (PID {pid})"` |
| TerminateProcess 失败 | `"terminate_child_processes: failed to terminate '{name}' (PID {pid})"` |
| OpenProcess 失败 | `"terminate_child_processes: failed to open '{name}' (PID {pid})"` |

### UAC 提权清理

当服务以无管理员权限启动时，[request_uac_elevation](request_uac_elevation.md) 通过 `powershell.exe Start-Process -Verb RunAs` 生成一个新的提权实例，然后退出。PowerShell 子进程可能作为孤立进程持续存在。在下一次启动时（现在已提权），`terminate_child_processes` 在主轮询循环开始之前清理任何此类残留子进程。

### 安全考虑

- 该函数无差别地终止**所有**子进程。它不按进程名称或目的过滤。
- `TerminateProcess` 是立即的、非优雅的终止——子进程不会收到关闭通知或运行清理处理程序。
- 如果没有子进程存在，该函数静默遍历快照并返回，不采取任何操作。
- 快照句柄始终在函数返回前关闭，即使迭代在中途失败也是如此。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | `src/main.rs` 中的主启动逻辑 |
| **被调用者** | 无（直接使用 Win32 API 的叶子函数） |
| **Win32 API** | [GetCurrentProcessId](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)、[CreateToolhelp32Snapshot](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)、[Process32FirstW](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)、[Process32NextW](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw)、[OpenProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)（`PROCESS_TERMINATE`）、[TerminateProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess)、[CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | 对子进程的 `PROCESS_TERMINATE` 访问权限；对某些子进程可能需要提权 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 创建子进程的 UAC 提权 | [request_uac_elevation](request_uac_elevation.md) |
| 触发提权的管理员检查 | [is_running_as_admin](is_running_as_admin.md) |
| 模块枚举（使用类似的快照模式） | [enumerate_process_modules](enumerate_process_modules.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
