# log_apply_results 函数 (main.rs)

格式化并记录在单个进程的配置应用遍历之后收集在 [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) 中的变更和错误。错误被路由到查找日志；变更以右对齐、多行格式写入主日志。

## 语法

```rust
fn log_apply_results(pid: &u32, name: &String, result: ApplyConfigResult)
```

## 参数

`pid: &u32`

目标进程的进程标识符。在格式化输出中右填充至 5 个字符。

`name: &String`

来自匹配此进程的配置规则的进程名称（例如 `"game.exe"`）。用作日志行前缀的中间段。

`result: ApplyConfigResult`

包含当前应用周期中变更和错误的 [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) 累加器。被此函数消费。

## 返回值

此函数不返回值。

## 备注

如果 `result.is_empty()` 返回 `true`，函数立即退出，不对不需要变更且未遇到错误的进程产生日志输出。

### 错误日志记录

`result.errors` 中的所有条目通过 `log_to_find` 写入查找日志。这将错误输出与正常的变更跟踪分离，允许错误被独立审查（例如，在 `-process_logs` 后处理期间）。

### 变更日志记录

变更以对齐的多行布局格式化：

1. **第一条**变更以单行格式记录：

   `{pid:>5}::{name}::{change}`

   例如：`" 1234::game.exe::Priority: normal -> high"`

2. **后续**变更以计算出的填充前缀记录，使其直接与第一条变更的文本对齐。填充考虑了日志基础设施前置的 `[HH:MM:SS]` 时间前缀（10 个字符）。

这种对齐确保单个进程的所有变更在日志文件中显示为视觉上分组的块，当同时应用许多设置时提高了可读性。

### 日志基础设施

- `log_message` 以时间戳前缀写入主日志文件。
- `log_pure_message` 写入主日志文件而不添加自己的时间戳，依赖调用者提供的填充进行对齐。
- `log_to_find` 写入查找模式后处理使用的 `.find.log` 文件。

## 需求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用者** | [apply_config](apply_config.md)、[main](main.md) 中的仅线程级应用路径 |
| **被调函数** | `log_to_find`、`log_message`、`log_pure_message` |
| **Win32 API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 结果累加器 | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| 组合应用入口点 | [apply_config](apply_config.md) |
| 日志基础设施 | [logging.rs](../logging.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
