# log_process_find 函数 (logging.rs)

将发现的进程名称记录到查找日志文件，经过去重处理，使每个进程名称在每个会话中最多记录一次。

## 语法

```rust
#[inline]
pub fn log_process_find(process_name: &str)
```

## 参数

`process_name: &str`

发现的进程的可执行文件名称（例如 `"notepad.exe"`）。

## 返回值

此函数不返回值。

## 备注

此函数是查找模式进程日志记录的公共入口点。它获取全局 `FINDS_SET` 的锁并尝试插入 `process_name`。如果插入成功（名称之前不存在），该函数委托给 [log_to_find](log_to_find.md)，使用格式化消息 `"find {process_name}"`。如果名称已被记录，则该调用为无操作。

### 去重范围

`FINDS_SET` 静态变量在程序启动时初始化一次，在进程生命周期内从不被清除。这意味着去重跨越整个服务会话 —— 在第 1 分钟发现的进程名称在第 60 分钟不会被再次记录，即使该进程在此期间被重新启动。该集合仅在服务本身重新启动时重置，这也同时按日期轮换日志文件。

### 输出格式

写入查找日志的消息格式为：

```
[HH:MM:SS]find notepad.exe
```

时间戳前缀由 [log_to_find](log_to_find.md) 添加，而非由此函数添加。

### 线程安全性

该函数获取 `FINDS_SET` 的 `Mutex` 锁。锁仅在 `HashSet::insert` 调用期间保持；随后的 [log_to_find](log_to_find.md) 调用在锁释放后发生（在 `if` 条件的末尾隐式 drop）。

### 控制台模式

当 `USE_CONSOLE` 为 `true` 时，查找日志输出通过 [log_to_find](log_to_find.md) 重定向到 stdout。这通常是在使用 `-find` CLI 标志交互式运行该工具时的情况。

## 需求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用者** | 主轮询循环（查找模式路径）、[scheduler](../scheduler.rs/README.md) |
| **被调函数** | [log_to_find](log_to_find.md) |
| **静态变量** | `FINDS_SET`（`Lazy<Mutex<HashSet<String>>>`） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [logging.rs](README.md) |
| 查找日志写入器 | [log_to_find](log_to_find.md) |
| 通用日志函数 | [log_message](log_message.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
