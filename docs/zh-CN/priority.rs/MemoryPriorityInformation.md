# MemoryPriorityInformation 结构体 (priority.rs)

围绕 `u32` 值的 `#[repr(C)]` 包装器，用于与 Win32 `MEMORY_PRIORITY_INFORMATION` 结构体布局直接互操作。在通过 `ProcessMemoryPriority` 信息类调用 `SetProcessInformation` / `GetProcessInformation` 时使用，这些 API 期望传入指向单个 `ULONG` 字段的指针。

## 语法

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `0`（元组字段） | `u32` | 原始内存优先级值。对应于 Windows 定义的 `MEMORY_PRIORITY_*` 常量之一（例如从 `MEMORY_PRIORITY_VERY_LOW` 到 `MEMORY_PRIORITY_NORMAL`）。 |

## 备注

此结构体仅用于为设置内存优先级的 `NtSetInformationProcess` / `SetProcessInformation` 调用提供大小和内存布局正确的 FFI 类型。`#[repr(C)]` 属性确保内存布局与 Win32 API 期望的匹配——单个 `ULONG`（4 字节，自然对齐）。

该结构体派生 `PartialEq` 和 `Eq` 用于比较（例如在进行更改之前检查当前内存优先级是否与期望值匹配），并派生 `Clone` / `Copy` 以实现便捷的传值语义。

### 与 MemoryPriority 枚举的关系

[`MemoryPriority`](MemoryPriority.md) 是配置和日志中使用的类型安全、人类可读的枚举。当 Win32 互操作需要原始 `u32` 值时，`MemoryPriority::as_win_const()` 返回一个 `Option<MEMORY_PRIORITY>`，可将其包装在 `MemoryPriorityInformation` 中用于 FFI 调用。两种类型互补：

| 类型 | 用途 |
|------|---------|
| [`MemoryPriority`](MemoryPriority.md) | 配置解析、字符串显示、变体匹配 |
| `MemoryPriorityInformation` | 用于 Win32 `SetProcessInformation` 调用的 FFI 安全布局 |

### Win32 对应关系

Windows SDK 定义 `MEMORY_PRIORITY_INFORMATION` 如下：

```c
typedef struct _MEMORY_PRIORITY_INFORMATION {
    ULONG MemoryPriority;
} MEMORY_PRIORITY_INFORMATION;
```

`MemoryPriorityInformation(pub u32)` 是与此具有相同布局的 Rust 新型类型，使其可以安全地作为指针传递给 `SetProcessInformation` 的 `lpProcessInformation` 参数。

## 需求

| | |
|---|---|
| **模块** | [`src/priority.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **调用方** | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| **依赖项** | 无（纯新型类型包装器） |
| **Win32 API** | [`SetProcessInformation`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) 配合 `ProcessMemoryPriority` |
| **权限** | `PROCESS_SET_INFORMATION`（调用方在写入时需要） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 类型安全的内存优先级枚举 | [`MemoryPriority`](MemoryPriority.md) |
| 内存优先级应用 | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| 模块概述 | [priority.rs](README.md) |
| 进程优先级枚举 | [`ProcessPriority`](ProcessPriority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
