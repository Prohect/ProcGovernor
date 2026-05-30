# ProcessSnapshot 结构体 (process.rs)

RAII 守卫，使用 `NtQuerySystemInformation(SystemProcessInformation)` 捕获所有正在运行的进程及其线程的时间点快照。该结构体持有指向共享缓冲区和进程映射表的可变引用，两者在快照被 drop 时自动清除。

## 语法

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `buffer` | `&'a mut Vec<u8>` | 指向原始字节缓冲区的可变引用，该缓冲区保存 `NtQuerySystemInformation` 返回的 `SYSTEM_PROCESS_INFORMATION` 链表。通过 [SNAPSHOT_BUFFER](README.md) 跨迭代重用。 |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | 指向进程映射表的可变引用，在 `take()` 期间填充。键为 PID，值为 [ProcessEntry](ProcessEntry.md) 结构体。公开以便调用者可以遍历和查询进程。 |

## 方法

### take

```rust
pub fn take(
    buffer: &'a mut Vec<u8>,
    pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
) -> Result<Self, i32>
```

通过调用 `NtQuerySystemInformation(SystemProcessInformation)` 捕获系统上所有进程和线程的快照。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `buffer` | `&'a mut Vec<u8>` | 指向用于接收系统信息的字节缓冲区的可变引用。如果缓冲区太小，则动态增长。通常来自 `SNAPSHOT_BUFFER`。 |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | 指向将被进程条目填充的 HashMap 的可变引用。每次调用开始时清除。通常来自 `PID_TO_PROCESS_MAP`。 |

**返回值**

`Result<ProcessSnapshot<'a>, i32>` —— 成功时返回 RAII 快照守卫。失败时返回作为 `i32` 的 NTSTATUS 错误码。

**备注**

- 该函数在重试循环中调用 `NtQuerySystemInformation`。如果调用返回 `STATUS_INFO_LENGTH_MISMATCH`（`0xC0000004`），缓冲区将重新分配到 `ReturnLength` 输出参数指示的大小（向上舍入到 8 字节边界），如果 `ReturnLength` 为零则加倍。
- 成功调用后，原始缓冲区作为 `SYSTEM_PROCESS_INFORMATION` 结构的链表遍历。每个条目的 `NextEntryOffset` 字段指向下一个条目；偏移量为零表示最后一个条目。
- 对于每个进程，通过 `ProcessEntry::new()` 构造一个 [ProcessEntry](ProcessEntry.md)，该方法提取小写的进程名称并存储指向线程数组的基指针。
- 缓冲区和映射表在 **drop 时清除**，因此从 `pid_to_process` 派生的所有 [ProcessEntry](ProcessEntry.md) 引用在快照被 drop 后变为无效。

### Drop

```rust
impl<'a> Drop for ProcessSnapshot<'a> {
    fn drop(&mut self);
}
```

当快照超出作用域时，清除 `pid_to_process` 和 `buffer`。这确保 [ProcessEntry](ProcessEntry.md) 中存储的陈旧原始指针（`threads_base_ptr` 字段）在底层缓冲区被重用后永远不会被解引用。

## 备注

`ProcessSnapshot` 遵循 RAII 模式，将解析的进程数据的生命周期绑定到原始缓冲区的生命周期。因为 `SYSTEM_PROCESS_INFORMATION.Threads` 是附加到每个结构的可变长度数组，线程数据仅在原始缓冲区存活时有效。`Drop` 实现强制执行此不变性。

### 典型用法

```rust
let mut buffer = SNAPSHOT_BUFFER.lock().unwrap();
let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buffer, &mut map)?;
// 使用 snapshot.pid_to_process 遍历进程
// snapshot 在此处被 drop，清除 buffer 和 map
```

### 缓冲区增长策略

| 条件 | 操作 |
|------|------|
| `ReturnLength > 0` | 分配 `((ReturnLength / 8) + 1) * 8` 字节（8 字节对齐上限） |
| `ReturnLength == 0` | 将当前缓冲区容量加倍 |

初始缓冲区大小为 32 字节（来自 `SNAPSHOT_BUFFER`），这将在第一次调用时始终触发至少一次调整大小。后续调用重用上一次成功调用的容量，因此在第一次迭代后调整大小变得罕见。

## 需求

| | |
|---|---|
| **模块** | `src/process.rs` |
| **调用者** | `src/main.rs` 中的主轮询循环 |
| **被调函数** | `NtQuerySystemInformation` (ntdll)、[ProcessEntry::new](ProcessEntry.md) |
| **NT API** | [NtQuerySystemInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation)，使用 `SystemProcessInformation`（类 5） |
| **权限** | 基本枚举不需要权限；[SeDebugPrivilege](../winapi.rs/enable_debug_privilege.md) 将可见性扩展到受保护的进程 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [process.rs](README.md) |
| 进程数据容器 | [ProcessEntry](ProcessEntry.md) |
| 线程句柄管理 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
