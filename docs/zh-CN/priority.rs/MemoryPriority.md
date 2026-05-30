# MemoryPriority 枚举 (priority.rs)

Windows 内存优先级级别的类型安全表示。在人类可读的字符串名称与 Win32 `MEMORY_PRIORITY` 常量之间进行映射，为配置解析和状态显示提供双向转换。

## 语法

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
```

## 成员

| 变体 | 字符串名称 | Win32 常量 | 描述 |
|---------|-------------|----------------|-------------|
| `None` | `"none"` | *(无)* | 未配置内存优先级。哨兵值——`as_win_const()` 返回 `None`。 |
| `VeryLow` | `"very low"` | `MEMORY_PRIORITY_VERY_LOW` | 最低内存优先级。页面是回收修剪的首选候选。 |
| `Low` | `"low"` | `MEMORY_PRIORITY_LOW` | 低内存优先级。 |
| `Medium` | `"medium"` | `MEMORY_PRIORITY_MEDIUM` | 中等内存优先级。 |
| `BelowNormal` | `"below normal"` | `MEMORY_PRIORITY_BELOW_NORMAL` | 低于正常的内存优先级。 |
| `Normal` | `"normal"` | `MEMORY_PRIORITY_NORMAL` | 默认内存优先级。页面在工作集中具有标准生命周期。 |

## 常量

### TABLE

```rust
const TABLE: &'static [(Self, &'static str, Option<MEMORY_PRIORITY>)]
```

私有查找表，包含所有 `(变体, 字符串名称, win32_常量)` 元组。所有转换方法都遍历此表，确保映射拥有唯一数据源。`MEMORY_PRIORITY` 值从 `windows::Win32::System::Threading` 导入。

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回此内存优先级级别的人类可读字符串表示（例如 `"very low"`、`"normal"`）。如果变体未在 `TABLE` 中找到，则返回 `"unknown"`（对于正常构造的值不应发生）。

**返回值：** 适用于配置显示和日志输出的 `&'static str`。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>
```

返回对应的 Win32 `MEMORY_PRIORITY` 常量，对于 `None` 变体返回 `None`。返回的值通过 [`MemoryPriorityInformation`](MemoryPriorityInformation.md) 包装结构体与 [`SetProcessInformation`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) 一起使用。

**返回值：** `Some(MEMORY_PRIORITY)` 用于已配置级别，对于 `MemoryPriority::None` 返回 `None`。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将字符串解析为 `MemoryPriority` 变体。比较不区分大小写（输入在匹配 `TABLE` 之前先转换为小写）。无法识别的字符串映射到 `MemoryPriority::None`。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `s` | `&str` | 要解析的字符串（例如 `"Very Low"`、`"below normal"`、`"Normal"`）。 |

**返回值：** 匹配的 `MemoryPriority` 变体，若无法识别则为 `None`。

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

将原始 `u32` 内存优先级值（由 [`GetProcessInformation`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) 返回）转换为人类可读的字符串。与 `TABLE` 中 `MEMORY_PRIORITY` 常量的 `.0` 字段进行匹配。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `val` | `u32` | 从进程读取的原始内存优先级值。 |

**返回值：** 诸如 `"very low"` 或 `"normal"` 的 `&'static str`，如果值不匹配任何已知常量则返回 `"unknown"`。

## 备注

- 内存优先级级别控制 Windows 内存管理器从进程工作集中回收修剪页面的积极程度。较低的优先级意味着在内存压力下页面会更早被修剪。
- 与映射到 `PROCESS_CREATION_FLAGS` 的 [`ProcessPriority`](ProcessPriority.md) 不同，内存优先级通过 `SetProcessInformation` / `NtSetInformationProcess` 的 `ProcessMemoryPriority` 信息类来设置。 [`MemoryPriorityInformation`](MemoryPriorityInformation.md) `repr(C)` 结构体提供了此调用所需的 FFI 布局。
- `from_str` 方法**未**实现标准的 `std::str::FromStr` trait。它是一个独立的关联函数，在解析失败时返回默认值（`None`）而非错误。
- 所有转换方法对 `TABLE` 进行线性扫描。由于只有 6 个条目，此开销可忽略不计。

## 需求

| | |
|---|---|
| **模块** | [`src/priority.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **调用方** | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md)，[`config.rs`](../config.rs/README.md) 中的配置解析 |
| **依赖项** | `windows::Win32::System::Threading::{MEMORY_PRIORITY, MEMORY_PRIORITY_VERY_LOW, MEMORY_PRIORITY_LOW, MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_BELOW_NORMAL, MEMORY_PRIORITY_NORMAL}` |
| **Win32 API** | [`SetProcessInformation`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation)、[`GetProcessInformation`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) |
| **权限** | 对于调用方具有 `PROCESS_SET_INFORMATION` 访问权限的进程，设置内存优先级无需额外权限 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| FFI 包装结构体 | [`MemoryPriorityInformation`](MemoryPriorityInformation.md) |
| 内存优先级应用函数 | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| 进程优先级枚举 | [`ProcessPriority`](ProcessPriority.md) |
| IO 优先级枚举 | [`IOPriority`](IOPriority.md) |
| 线程优先级枚举 | [`ThreadPriority`](ThreadPriority.md) |
| 模块概述 | [priority.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
