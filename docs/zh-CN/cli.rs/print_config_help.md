# print_config_help 函数 (cli.rs)

将配置文件格式文档打印到活动输出（控制台或日志文件）。此函数遍历 [get_config_help_lines](get_config_help_lines.md) 返回的行，并使用项目的 `log!` 宏写入每一行。

## 语法

```rust
pub fn print_config_help()
```

## 参数

此函数没有参数。

## 返回值

此函数不返回值。

## 备注

`print_config_help` 是 [get_config_help_lines](get_config_help_lines.md) 的一个薄包装器。它作为一个单独的函数存在，以便帮助文本可以独立打印（例如，从未来的 `--help-config` 标志），或者像 [print_help_all](print_help_all.md) 那样与其他帮助部分组合。

与 [print_help](print_help.md) 和 [print_help_all](print_help_all.md) 不同，此函数本身**不**设置控制台输出标志（`get_use_console!()`）。当作为 `print_help_all` 的一部分调用时，该标志已由调用者设置；当独立调用时，调用者负责确保输出目标已配置。

配置帮助文本涵盖：

- **术语** —— Intel 混合 CPU 的 P-core / E-core 命名约定。
- **配置格式** —— 冒号分隔规则语法的逐字段分解。
- **CPU 规格格式** —— 范围 (`0-7`)、单独 CPU (`0;4;8`)、十六进制位掩码 (`0xFF`) 和别名引用 (`*pcore`)。
- **优先级级别** —— 进程优先级、IO 优先级和内存优先级的有效值。
- **理想处理器语法** —— 支持多段的模块前缀匹配规则。
- **进程组** —— 命名和匿名 `{ }` 组块。

## 需求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用者** | [print_help_all](print_help_all.md) |
| **被调函数** | [get_config_help_lines](get_config_help_lines.md)、`log!` 宏 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [cli.rs](README.md) |
| 帮助文本源 | [get_config_help_lines](get_config_help_lines.md) |
| 完整帮助打印函数 | [print_help_all](print_help_all.md) |
| CLI 帮助打印函数 | [print_cli_help](print_cli_help.md) |
| 配置解析器 | [read_config](../config.rs/read_config.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
