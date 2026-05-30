# parse_constant 函数 (config.rs)

从配置文件中解析 `@NAME = value` 常量定义，并更新 [ConfigResult](ConfigResult.md) 中 [ConfigConstants](ConfigConstants.md) 的对应字段。识别的常量控制主线程调度的滞后行为。

## 语法

```rust
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
```

## 参数

`name: &str`

从配置行的 `@` 和 `=` 之间提取的大写常量名称。调用者负责在传递前修剪空白并转换为大写。识别的名称为：

| 名称 | 预期类型 | 描述 |
|------|----------|------|
| `MIN_ACTIVE_STREAK` | `u8` | 线程在晋升之前必须超过进入阈值的连续最小迭代次数。 |
| `KEEP_THRESHOLD` | `f64` | 主线程被降级的 CPU 周期份额分数。 |
| `ENTRY_THRESHOLD` | `f64` | 非主线程开始累积活跃连续次数所必须超过的 CPU 周期份额分数。 |

`value: &str`

`=` 号右侧的字符串值，由调用者修剪了空白。根据常量名称解析为适当类型（`MIN_ACTIVE_STREAK` 为 `u8`，阈值为 `f64`）。

`line_number: usize`

常量定义在配置文件中出现的基于 1 的行号。包含在错误和日志消息中以供诊断。

`result: &mut ConfigResult`

**\[入参, 出参\]** 解析结果累加器。成功时，`result.constants` 中的相应字段被更新，`result.constants_count` 递增。失败时，错误消息被推送到 `result.errors`。

## 返回值

此函数不返回值。结果通过变更 `result` 参数传达。

## 备注

### 解析行为

函数使用对 `name` 参数的 `match` 来确定正在设置哪个常量：

1. **`MIN_ACTIVE_STREAK`：** 值被解析为 `u8`。成功时，`result.constants.min_active_streak` 被设置，并发出日志消息。失败时，推送一条错误，指示无效的 `u8` 值。

2. **`KEEP_THRESHOLD` / `ENTRY_THRESHOLD`：** 值被解析为 `f64`。成功时，相应字段（`keep_threshold` 或 `entry_threshold`）被设置并记录日志。失败时，推送一条错误。

3. **未知名称：** 任何未识别的常量名称产生一条警告（而非错误），表明它将被忽略。这允许未来兼容新增常量而不会破坏现有配置。

### 日志记录

每个成功解析的常量通过 `log_message` 生成一条格式为 `"Config: NAME = value"` 的日志消息。这在启动和热重载期间提供即时反馈。

### 错误消息

错误消息遵循以下格式：类型不匹配时为 `"Line {N}: Invalid constant value '{value}' for '{name}' (expected type)"`，未识别名称时为 `"Line {N}: Unknown constant '{name}' - will be ignored"`。

### 与配置文件的关系

常量使用 `@` 前缀出现在配置文件顶部：

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1
```

[read_config](read_config.md) 函数检测以 `@` 开头的行，提取 `=` 号前后的名称和值，将名称转为大写，修剪值，并委托给 `parse_constant`。

### 幂等性

如果同一常量在配置文件中被定义多次，最后定义生效。每次定义递增 `constants_count`，因此如果存在重复，计数可能超过 3（已识别常量的数量）。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有（`fn`） |
| **调用者** | [read_config](read_config.md) |
| **被调用者** | `log_message`（日志记录） |
| **依赖项** | [ConfigResult](ConfigResult.md)、[ConfigConstants](ConfigConstants.md) |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 常量结构体 | [ConfigConstants](ConfigConstants.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 别名定义解析器 | [parse_alias](parse_alias.md) |
| 传播常量的热重载 | [hotreload_config](hotreload_config.md) |
| 解析结果容器 | [ConfigResult](ConfigResult.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
