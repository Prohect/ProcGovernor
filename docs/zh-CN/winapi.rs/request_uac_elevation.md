# request_uac_elevation 函数 (winapi.rs)

通过生成一条 PowerShell `Start-Process -Verb RunAs` 命令，以管理员权限重新启动当前进程，这会触发 Windows UAC 同意提示。提权的子进程启动后，当前（未提权）进程退出。

## 语法

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `console` | `bool` | 服务是否以 `console` CLI 标志启动。当为 `true` 时，记录一条警告，说明提权进程的输出将出现在新的控制台窗口中，而非当前窗口。 |

## 返回值

`io::Result<()>` — 成功时，此函数**不返回**，因为当前进程在生成提权的子进程后调用 `std::process::exit(0)`。失败时（例如，无法启动 PowerShell），返回描述生成失败的 `io::Error`。

## 备注

### 提权机制

该函数构造以下形式的 PowerShell 命令：

```
Start-Process -FilePath '<current_exe_path>' -Verb RunAs -ArgumentList '<original_args> -skip_log_before_elevation'
```

- 当前可执行文件路径通过 `std::env::current_exe()` 获取。
- 所有原始命令行参数（跳过 `argv[0]`）被转发到提权实例。
- 标志 `-skip_log_before_elevation` 被追加，以防止提权进程重复已在未提权实例中执行的日志初始化。

### 控制台模式警告

当 `console` 为 `true` 时，该函数记录一条警告，说明提权进程将在单独的控制台窗口中运行。这是 `runas` 动词固有的特性——Windows 为提权进程创建一个新控制台，当前控制台会话不再接收服务的输出。

### 进程退出

在 `Command::spawn()` 成功后，当前进程立即调用 `exit(0)`。这确保只有一个服务实例正在运行（新提权的那个）。退出是无条件的——调用者中此函数调用之后的清理代码将不会执行。

### 错误处理

如果 `Command::spawn()` 失败（例如，找不到 PowerShell，或在进程启动前用户拒绝了 UAC 提示），错误将被记录并作为 `io::Result<Err>` 传播。调用者随后可以决定是继续以无提权状态运行还是中止。

### UAC 提示行为

实际的 UAC 同意对话框在提权的 PowerShell 实例执行 `Start-Process -Verb RunAs` 命令时由 Windows 呈现。如果用户点击**否**或对话框超时，从生成（未提权）进程的角度来看，`Start-Process` cmdlet 静默失败——`spawn()` 调用本身成功，因为它只启动 PowerShell，而非提权目标。此时当前进程将已经退出。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | `src/main.rs` 中的主启动逻辑，由 `!is_running_as_admin() && !cli.no_uac` 保护 |
| **被调用者** | `std::env::current_exe`、`std::env::args`、`std::process::Command::spawn`、`std::process::exit` |
| **Win32 API** | 无直接调用；依赖 PowerShell 的 `Start-Process -Verb RunAs`，后者内部调用 [ShellExecuteW](https://learn.microsoft.com/zh-cn/windows/win32/api/shellapi/nf-shellapi-shellexecutew) |
| **权限** | 调用不需要权限；UAC 提示将提权授予子进程 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 控制此调用的管理员检查 | [is_running_as_admin](is_running_as_admin.md) |
| 提权后启用的调试权限 | [enable_debug_privilege](enable_debug_privilege.md) |
| 提权后启用的基础优先级权限 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| 启动时的子进程清理 | [terminate_child_processes](terminate_child_processes.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
