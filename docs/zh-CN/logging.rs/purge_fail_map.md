# purge_fail_map 函数 (logging.rs)

从应用失败跟踪映射中移除陈旧条目，以防止内存无限增长。由主轮询循环在枚举运行进程后周期性调用。

## 语法

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, &str)])
```

## 参数

`pids_and_names: &[(u32, &str)]`

一个 `(pid, process_name)` 元组的切片，表示当前正在运行的进程。只有 PID **和**进程名称与此切片中的元组匹配的条目才能在清除后存活。

## 返回值

此函数不返回值。它直接修改全局 `PID_MAP_FAIL_ENTRY_SET`。

## 备注

### 算法

清除遵循标记—清除模式：

1. **将所有条目标记为已死：** 遍历全局 `PID_MAP_FAIL_ENTRY_SET` 映射中的每个条目，并将每个 `alive` 标志设置为 `false`。
2. **重新标记存活的：** 对于 `pids_and_names` 中的每个 `(pid, name)`，如果该 PID 存在于映射中且至少有一个条目的 `process_name` 与 `name` 匹配，则将该第一个条目的 `alive` 标志设置回 `true`。
3. **清除：** 对外层映射调用 `retain`，移除内部映射中不包含 `alive == true` 条目的任何 PID。

### PID 重用感知

因为 Windows PID 可以回收，重新标记步骤除了检查 PID 外，还会检查 `process_name` 字段。如果 PID 被另一个进程重用，其条目都不会匹配新名称，陈旧的条目将被清除。这防止了错误去重错误地抑制恰好获得回收 PID 的新进程的错误。

### 调用频率

此函数在每个主循环迭代中调用一次，在完整的进程列表被枚举之后。它在操作期间获取 `PID_MAP_FAIL_ENTRY_SET` 互斥锁，因此不应在已经持有此锁的上下文中调用。

### 与 is_new_error 的关系

虽然 [is_new_error](is_new_error.md) 添加条目并处理每个 PID 的进程名称更改（当检测到名称不匹配时清除内部映射），`purge_fail_map` 处理移除已完全退出的进程条目的互补情况。它们共同确保映射始终以当前正在运行的受监控进程数量为界。

## 需求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用者** | 主轮询循环（进程枚举之后） |
| **被调函数** | `get_pid_map_fail_entry_set!()` 宏 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [logging.rs](README.md) |
| 错误去重检查 | [is_new_error](is_new_error.md) |
| 失败条目键 | [ApplyFailEntry](ApplyFailEntry.md) |
| 操作枚举 | [Operation](Operation.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
