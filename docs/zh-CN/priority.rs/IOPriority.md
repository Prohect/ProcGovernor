# IOPriority 枚举 (priority.rs)

Windows NT IO 优先级级别的类型安全表示。在人类可读的字符串名称与用于 `NtSetInformationProcess` / `NtQueryInformationProcess` 的 `ProcessIoPriority` 信息类的未记录 `u32` IO 优先级值之间进行映射。

## 语法

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
```

## 成员

| 变体 | 字符串名称 | Win32 常量 | 描述 |
|---------|-------------|----------------|-------------|
| `None` | `"none"` | *(无)* | 未配置 IO 优先级。`as_win_const()` 返回 `None`。 |
| `VeryLow` | `"very low"` | `0` | 最低 IO 优先级。后台级别的 IO 调度。 |
| `Low` | `"low"` | `1` | 低 IO 优先级。降低的 IO 带宽分配。 |
| `Normal` | `"normal"` | `2` | 大多数进程的默认 IO 优先级。 |
| `High` | `"high"` | `3` | 最高 IO 优先级。需要 `SeIncreaseBasePriorityPrivilege` 和管理员权限。 |

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回此 IO 优先级变体的人类可读字符串名称。如果变体未在内部表中找到，则返回 `"unknown"`（对于有效变体不可达）。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<u32>
```

返回用于 `NtSetInformationProcess` 的 NT IO 优先级值，对于 `IOPriority::None` 变体返回 `None`。当此方法返回 `None` 时，调用方应跳过 API 调用。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为 `IOPriority` 变体。如果字符串不匹配任何已知 IO 优先级名称，则返回 `IOPriority::None`。输入在比较之前先转换为小写。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `s` | `&str` | 要解析的字符串（例如 `"very low"`、`"Normal"`、`"HIGH"`）。 |

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

将 NT IO 优先级值转换回其人类可读的字符串名称。如果值不匹配任何已知常量，则返回 `"unknown"`。用于在比较当前 IO 优先级与期望值时记录日志。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `val` | `u32` | NT IO 优先级值（0–3）。 |

## 备注

### TABLE 驱动的转换

与本模块中的所有优先级枚举一样，`IOPriority` 使用单一的 `const TABLE` 数组驱动所有四个转换方法。每个条目是一个 `(Self, &'static str, Option<u32>)` 元组，确保字符串名称和数值常量仅在唯一一处定义。

### 未记录的 API

与进程优先级（使用已记录的 `SetPriorityClass` API）不同，IO 优先级通过 NT 本机 `NtSetInformationProcess` 使用 `ProcessIoPriority`（信息类 33）进行管理。`u32` 值 0–3 虽未经 Microsoft 正式记录，但通过逆向工程和社区文档已得到广泛确认。

### 高 IO 优先级

设置 `IOPriority::High`（值为 `3`）要求调用进程持有 `SeIncreaseBasePriorityPrivilege` 并以管理员权限运行。否则 `NtSetInformationProcess` 调用将失败并返回 `STATUS_PRIVILEGE_NOT_HELD`。该服务在启动时通过 [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) 获取此权限。

### 关于 from_str 的说明

此 `from_str` 是固有方法，并非 `std::str::FromStr` trait 实现。它不返回 `Result`——无法识别的字符串静默映射到 `IOPriority::None`。

## 需求

| | |
|---|---|
| **模块** | [`src/priority.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **使用方** | [`apply_io_priority`](../apply.rs/apply_io_priority.md)、配置解析器（[`read_config`](../config.rs/read_config.md)） |
| **Win32 API** | `NtQueryInformationProcess` / `NtSetInformationProcess` 配合 `ProcessIoPriority` |
| **权限** | `High` 变体需要 `SeIncreaseBasePriorityPrivilege` |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 进程优先级枚举 | [ProcessPriority](ProcessPriority.md) |
| 内存优先级枚举 | [MemoryPriority](MemoryPriority.md) |
| 线程优先级枚举 | [ThreadPriority](ThreadPriority.md) |
| IO 优先级应用 | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| 模块概述 | [priority.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
