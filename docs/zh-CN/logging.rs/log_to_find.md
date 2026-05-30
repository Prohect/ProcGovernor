# log_to_find 函数 (logging.rs)

将带时间戳的消息写入查找模式日志文件，或在启用控制台输出时写入控制台。

## 语法

```rust
pub fn log_to_find(msg: &str)
```

## 参数

`msg: &str`

要记录的消息字符串。时间戳前缀 `[HH:MM:SS]` 会自动添加到输出之前。

## 返回值

此函数不返回值。

## 备注

此函数是查找模式输出的专用日志记录接收器。它写入单独的日志文件（`YYYYMMDD.find.log`）而非主应用程序日志，使查找模式发现保持隔离以便于查看。

### 输出路由

| 条件 | 目标 |
|------|------|
| `USE_CONSOLE` 为 `true` | 通过 `stdout` 输出到标准输出 |
| `USE_CONSOLE` 为 `false` | 通过 `get_logger_find!()` 获取的查找日志文件 |

### 与 log_message 的区别

- **无 dust-bin 检查：** 与 [log_message](log_message.md) 不同，此函数**不**检查 `DUST_BIN_MODE` 标志。查找模式日志记录永远不会被 dust-bin 机制抑制。
- **单独的日志文件：** 输出发送到查找日志文件（`YYYYMMDD.find.log`）而非主日志文件（`YYYYMMDD.log`）。

### 时间戳格式

时间戳使用缓存的 `LOCAL_TIME_BUFFER` 格式化为 `[HH:MM:SS]`。最终输出行的格式为：

```
[14:32:07]find chrome.exe
```

### 错误处理

写入失败被静默忽略。该函数使用 `let _ = writeln!(...)` 丢弃任何 I/O 错误。

## 需求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用者** | [log_process_find](log_process_find.md) |
| **被调函数** | `get_local_time!()`、`get_use_console!()`、`get_logger_find!()` |
| **访问的静态变量** | `LOCAL_TIME_BUFFER`、`USE_CONSOLE`、`FIND_LOG_FILE` |
| **权限** | 无（文件必须已经打开） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [logging.rs](README.md) |
| 查找模式去重包装器 | [log_process_find](log_process_find.md) |
| 主日志函数 | [log_message](log_message.md) |
| 日志文件路径构建 | [get_log_path](get_log_path.md) |
| 无时间戳的原始日志记录 | [log_pure_message](log_pure_message.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
