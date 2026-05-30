# print_cli_help 函数 (cli.rs)

将详细的 CLI 帮助信息打印到控制台或日志，涵盖所有命令行选项，包括基本参数、操作模式以及调试/测试选项。

## 语法

```rust
pub fn print_cli_help()
```

## 参数

此函数没有参数。

## 返回值

此函数不返回值。

## 备注

与 [print_help](print_help.md) 仅显示最常用选项不同，`print_cli_help` 输出每个受支持的命令行标志和参数的完整参考。输出分为三个部分：

1. **基本参数** —— 通用标志，例如 `-help`、`-console`、`-noUAC`、`-config`、`-find`、`-blacklist`、`-interval` 和 `-resolution`。
2. **操作模式** —— 特定任务模式，包括 `-validate`、`-processlogs`、`-dryrun`、`-convert`、`-autogroup`，以及它们使用的 `-in` / `-out` 文件参数。
3. **调试和测试选项** —— 用于开发和故障排除的标志：`-loop`、`-logloop`、`-noDebugPriv`、`-noIncBasePriority`、`-no_etw` 和 `-continuous_process_level_apply`。

输出还包括一个**调试**部分，其中包含适用于非管理员（控制台）和管理员（日志文件）测试场景的即用型示例命令行，以及一条说明 —— 当 UAC 提权生成新会话时，`-console` 输出会丢失。

### 控制台副作用

此函数本身**不**设置控制台输出标志。调用者有责任在调用此函数之前确保控制台输出已启用。实际上，[print_help_all](print_help_all.md) 在委托给 `print_cli_help` 之前设置了控制台标志。

### 输出机制

所有输出都通过项目的 `log!` 宏写入，该宏根据当前的 `use_console` 全局状态路由到控制台或日志文件。

## 需求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用者** | [print_help_all](print_help_all.md) |
| **被调函数** | `log!` 宏 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [cli.rs](README.md) |
| 基本帮助 | [print_help](print_help.md) |
| 配置格式帮助 | [print_config_help](print_config_help.md) |
| 合并帮助 | [print_help_all](print_help_all.md) |
| 参数解析器 | [parse_args](parse_args.md) |
| CLI 状态结构体 | [CliArgs](CliArgs.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
