# is_new_error 函数 (logging.rs)

确定给定进程的特定操作失败是否是首次出现，使调用者能够仅记录新错误并抑制重复的相同失败。

## 语法

```rust
pub fn is_new_error(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
) -> bool
```

## 参数

`pid: u32`

经历失败的进程标识符。用作全局 `PID_MAP_FAIL_ENTRY_SET` 映射中的顶级键。

`tid: u32`

与失败关联的线程标识符。对于不涉及特定线程的进程级操作，调用者通常传递 `0`。

`process_name: &str`

进程的可执行文件名称（例如 `"explorer.exe"`）。既用作去重键的一部分，也用于检测 PID 重用 —— 如果 PID 的存储条目具有不同的进程名称，则该 PID 的整个条目集在插入新条目之前被清除。

`operation: Operation`

标识哪个 Windows API 调用失败的 [Operation](Operation.md) 变体。

`error_code: u32`

失败 API 调用返回的 Win32 或 NTSTATUS 错误码。如果上下文中没有可用的错误码，调用者传递 `0` 或自定义区分符以区分不同的失败模式。

## 返回值

`bool` —— 如果这是此精确的 `(pid, tid, process_name, operation, error_code)` 组合第一次被记录，则返回 `true`，意味着调用者应该记录它。如果该失败已被跟踪，则返回 `false`，意味着调用者应该抑制日志。

## 备注

### 去重策略

该函数通过全局静态 `PID_MAP_FAIL_ENTRY_SET` 维护一个两级映射：

```
HashMap<u32, HashMap<ApplyFailEntry, bool>>
  ^pid         ^(tid, process_name, operation, error_code) -> 存活标志
```

每个 [ApplyFailEntry](ApplyFailEntry.md) 通过所有四个字段（`tid`、`process_name`、`operation`、`error_code`）进行相等性比较。`bool` 值是 [purge_fail_map](purge_fail_map.md) 在垃圾回收期间使用的"存活"标志。

### PID 重用检测

**不变性：** PID 的失败条目集中的所有条目预期共享相同的 `process_name`。当函数发现 PID 的现有条目具有与提供的名称不同的进程名称时，它在插入新条目之前清除该 PID 的整个条目集。这处理了 Windows PID 重用场景，其中已终止进程的 PID 被重新分配给新的、不相关的进程。

### 存活标志管理

当找到现有条目（重复）时，其存活标志设置为 `true`。这确保该条目在下一个 [purge_fail_map](purge_fail_map.md) 周期中存活，该周期将所有条目标记为已死，然后重新标记活跃的条目。

### 线程安全性

该函数通过 `get_pid_map_fail_entry_set!()` 宏获取全局 `PID_MAP_FAIL_ENTRY_SET` 互斥锁。锁在查找和潜在插入期间保持。

## 需求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用者** | [log_error_if_new](../apply.rs/log_error_if_new.md)、[apply.rs](../apply.rs/README.md) 中的应用函数 |
| **被调函数** | `get_pid_map_fail_entry_set!()` 宏 |
| **Win32 API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [logging.rs](README.md) |
| 失败条目键结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| 操作枚举 | [Operation](Operation.md) |
| 陈旧条目的垃圾回收 | [purge_fail_map](purge_fail_map.md) |
| 调用方错误日志记录辅助函数 | [log_error_if_new](../apply.rs/log_error_if_new.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
