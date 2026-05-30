# print_help_all 函数 (cli.rs)

打印 ProcGovernor 的完整帮助输出，将命令行用法和配置文件格式文档合并为一个参考。这是 `-helpall` / `--helpall` 命令行标志的处理函数。

## 语法

```rust
pub fn print_help_all()
```

## 参数

此函数没有参数。

## 返回值

此函数不返回值。

## 备注

`print_help_all` 是 `-helpall` / `--helpall` 命令行标志的入口点。它依次执行三个操作：

1. **强制控制台输出** —— 通过 `get_use_console!()` 将全局 `use_console` 标志设置为 `true`，确保所有后续输出发送到交互式控制台而不是日志文件。
2. **打印 CLI 帮助** —— 调用 [print_cli_help](print_cli_help.md) 显示完整的命令行参数参考，包括基本参数、操作模式、调试/测试选项和示例调试命令。
3. **打印配置帮助** —— 通过 `log!("")` 发出一个空白分隔行，然后调用 [print_config_help](print_config_help.md) 显示配置文件格式文档，涵盖术语、规则语法、CPU 规格格式、优先级级别、理想处理器语法和进程组语法。

### 控制台标志的副作用

因为 `print_help_all` 强制 `use_console = true`，所以同一进程调用中的任何后续日志记录也将发送到控制台。这是有意为之 —— 用户是交互式运行的，期望所有输出显示在屏幕上。

### 与其他帮助函数的关系

| 函数 | 范围 | 设置控制台标志？ |
|------|------|-----------------|
| [print_help](print_help.md) | 简洁用法摘要 | 是 |
| [print_cli_help](print_cli_help.md) | 仅完整 CLI 参考 | 否 |
| [print_config_help](print_config_help.md) | 仅配置格式参考 | 否 |
| **print_help_all** | 完整 CLI + 配置格式 | 是 |

`print_help_all` 是唯一将 `print_cli_help` 和 `print_config_help` 组合在一起的函数。它也是仅有的两个设置控制台输出标志的帮助函数之一（与 `print_help` 一起），因为另外两个被设计为供组合函数调用的构建块。

## 需求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用者** | `main`（当 [CliArgs](CliArgs.md)`.help_all_mode` 为 `true` 时） |
| **被调函数** | `get_use_console!` 宏、`log!` 宏、[print_cli_help](print_cli_help.md)、[print_config_help](print_config_help.md) |
| **Win32 API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [cli.rs](README.md) |
| 基本帮助输出 | [print_help](print_help.md) |
| CLI 帮助（内部调用） | [print_cli_help](print_cli_help.md) |
| 配置帮助（内部调用） | [print_config_help](print_config_help.md) |
| 配置帮助行源 | [get_config_help_lines](get_config_help_lines.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
