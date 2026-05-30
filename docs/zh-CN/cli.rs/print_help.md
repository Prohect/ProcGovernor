# print_help 函数 (cli.rs)

将 ProcGovernor 的基本使用帮助信息打印到控制台。这是当用户在命令行上传递 `-help`、`--help`、`-?`、`/?` 或 `?` 时显示的默认帮助输出。

## 语法

```rust
pub fn print_help()
```

## 参数

此函数没有参数。

## 返回值

此函数不返回值。

## 备注

该函数在打印之前无条件地将全局 `use_console` 标志设置为 `true`，确保帮助输出定向到控制台而不是日志文件。这是必要的，因为服务通常记录到文件，但帮助文本必须对交互式用户可见。

帮助文本通过 `log!` 宏以单个多行原始字符串字面量发出。

### 显示的部分

| 部分 | 内容 |
|------|------|
| **标题** | 一行描述服务用途的文本 |
| **常用选项** | `-help`、`-helpall`、`-console`、`-config`、`-find`、`-interval`、`-noUAC`、`-resolution` |
| **模式** | `-validate`、`-processlogs`、`-dryrun`、`-convert`、`-autogroup` |

### 与其他帮助函数的关系

- `print_help` 显示适合快速参考的选项的精简子集。
- [print_cli_help](print_cli_help.md) 显示完整的 CLI 参考，包括调试和测试选项。
- [print_config_help](print_config_help.md) 显示配置文件格式文档。
- [print_help_all](print_help_all.md) 将 `print_cli_help` 和 `print_config_help` 合并为一个输出。

### 控制台标志的副作用

因为 `print_help` 强制 `use_console = true`，所以同一进程调用中的任何后续日志记录也将发送到控制台。这是有意为之 —— 当用户请求帮助时，他们是交互式运行的，不希望输出到日志文件。

## 需求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用者** | `main`（当 [CliArgs](CliArgs.md)`.help_mode` 为 `true` 时） |
| **被调函数** | `log!` 宏、`get_use_console!` 宏 |
| **Win32 API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CLI 参数解析器 | [parse_args](parse_args.md) |
| 详细 CLI 帮助 | [print_cli_help](print_cli_help.md) |
| 配置格式帮助 | [print_config_help](print_config_help.md) |
| 合并帮助 | [print_help_all](print_help_all.md) |
| CLI 参数结构体 | [CliArgs](CliArgs.md) |
| 模块概述 | [cli.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
