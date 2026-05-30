# ThreadStats 结构体 (scheduler.rs)

[PrimeThreadScheduler](PrimeThreadScheduler.md) 使用的每个线程的调度状态容器，用于在多次轮询迭代中跟踪 CPU 周期计数器、时间增量、句柄缓存、CPU 集合绑定、活动连续次数计数器、理想处理器分配和优先级记录。调度器观察到的每个线程在其父级 [ProcessStats](ProcessStats.md) 的 `tid_to_thread_stats` 映射中都有一个 `ThreadStats` 条目。

## 语法

```rust
pub struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: List<[u32; CONSUMER_CPUS]>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `last_total_time` | `i64` | 在最近完成的应用周期结束时记录的总 CPU 时间（内核 + 用户，以 100ns 为单位）。作为计算下一次迭代中时间增量的基线。 |
| `cached_total_time` | `i64` | 在当前周期中读取的总 CPU 时间，在由 [update_thread_stats](../apply.rs/update_thread_stats.md) 提交到 `last_total_time` 之前。 |
| `last_cycles` | `u64` | 线程在上一次迭代中的 CPU 周期计数快照，用作 [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) 中增量计算的基线。 |
| `cached_cycles` | `u64` | 在当前迭代中读取的线程 CPU 周期计数，在提交到 `last_cycles` 之前。 |
| `handle` | `Option<ThreadHandle>` | 此线程的缓存 [ThreadHandle](../winapi.rs/ThreadHandle.md)。如果尚未打开句柄则为 `None`。当存在时，`r_limited_handle` 保证有效；其他句柄应在使用前通过 `is_invalid()` 检查。当统计条目被移除时，句柄通过 `ThreadHandle::Drop` 自动关闭。 |
| `pinned_cpu_set_ids` | `List<[u32; CONSUMER_CPUS]>` | 通过 [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) 当前分配给此线程的 CPU 集合 ID。当线程未绑定到专用核心时为空。由降级路径用于知道要清除哪些 CPU 集合。 |
| `active_streak` | `u8` | 此线程的增量周期超过相对于最大值的进入阈值的连续迭代计数。由 [PrimeThreadScheduler::update_active_streaks](PrimeThreadScheduler.md) 递增，由 [PrimeThreadScheduler::select_top_threads_with_hysteresis](PrimeThreadScheduler.md) 使用。上限为 254 以防止溢出。当线程降到保持阈值以下时重置为 0。 |
| `start_address` | `usize` | 线程的 Win32 起始地址，通过 [get_thread_start_address](../winapi.rs/get_thread_start_address.md) 获取。用于基于模块前缀的理想处理器分配，并在自定义 `Debug` 实现和退出报告中解析为模块名称。 |
| `original_priority` | `Option<ThreadPriority>` | 服务修改之前线程的优先级。在第一次提升时捕获，以便 [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) 可以在线程失去主线程状态时恢复原始值。如果线程从未被提升则为 `None`。 |
| `last_system_thread_info` | `Option<SYSTEM_THREAD_INFORMATION>` | 来自进程快照的最新 `SYSTEM_THREAD_INFORMATION`，缓存用于 [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) 在进程退出时生成的诊断报告。包含内核时间、用户时间、上下文切换、等待原因、优先级和其他操作系统报告的线程状态。 |
| `ideal_processor` | `IdealProcessorState` | 跟踪此线程的当前和先前理想处理器组/编号分配。参见 [IdealProcessorState](IdealProcessorState.md)。 |
| `process_id` | `u32` | 拥有进程的 PID。由自定义 `Debug` 实现用于以正确的 PID 调用 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md)。 |

## 方法

### new

```rust
pub fn new(process_id: u32) -> Self
```

创建一个新的 `ThreadStats`，所有计数器清零、无句柄、空的 CPU 集合列表、连续次数为零，以及默认的 [IdealProcessorState](IdealProcessorState.md)。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `process_id` | `u32` | 拥有此线程的进程的 PID。存储以供 `Debug` 实现使用。 |

**返回值**

`ThreadStats` — 一个新实例，所有字段为其默认/零值。

### Default

```rust
impl Default for ThreadStats {
    fn default() -> Self;
}
```

委托给 `ThreadStats::new(0)`。

### Debug

```rust
impl fmt::Debug for ThreadStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
```

自定义 `Debug` 实现，通过 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) 将 `start_address` 解析为模块名称，而不是打印原始数字地址。这使得调试输出即时可用——例如显示 `start_address: "game.dll+0x1A40"` 而不是 `start_address: 0x7FF612341A40`。

**包含的字段：** `last_total_time`、`cached_total_time`、`last_cycles`、`cached_cycles`、`pinned_cpu_set_ids`、`active_streak`、`start_address`（已解析）、`original_priority`、`ideal_processor`。

**排除的字段：** `handle`、`last_system_thread_info`、`process_id`——省略以减少调试输出中的噪音。

## 备注

### 双缓冲模式

`last_*` / `cached_*` 字段对实现了时间和周期测量的双缓冲方案：

1. 在每个轮询迭代开始时，从操作系统读取当前值并存储在 `cached_total_time` / `cached_cycles` 中。
2. 增量计算为 `cached - last` 以确定自上一次迭代以来的活动。
3. 在迭代结束时（在 [update_thread_stats](../apply.rs/update_thread_stats.md) 中），缓存的值被复制到 `last_*`，建立新的基线。

这种分离确保增量计算和状态更新在轮询循环中发生在明确定义的点上，即使多个应用函数读取相同的线程统计数据也是如此。

### 句柄缓存

线程句柄在首次使用时延迟打开（通常由 [get_handles](../apply.rs/get_handles.md) 或 [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)），并存储在 `handle` 字段中以便在迭代中重用。这避免了在每个轮询周期上重复调用 `OpenThread` 的开销。句柄在以下情况下关闭：

- 线程的父进程退出（[drop_process_by_pid](PrimeThreadScheduler.md) 释放 `ThreadStats`）。
- 句柄被显式提取并释放。

### CPU 集合绑定

`pinned_cpu_set_ids` 字段使用栈分配的 `List<[u32; CONSUMER_CPUS]>`，以避免在消费级系统（≤64 个 CPU）的常见情况下进行堆分配。当线程被提升到主线程状态时，其专用 CPU 集合 ID 存储在此处。当降级时，此列表用于识别要清除哪些 CPU 集合，然后清空列表。

### 活动连续次数生命周期

| 连续次数值 | 含义 |
|---|---|
| `0` | 线程处于不活跃状态或最近降到保持阈值以下 |
| `1` | 线程在此次迭代中刚刚超过进入阈值 |
| `2..min_active_streak-1` | 线程正在累积连续次数，尚未有资格提升 |
| `≥ min_active_streak` | 线程有资格通过 `select_top_threads_with_hysteresis` 提升 |
| `254` | 最大连续次数值（硬上限，防止 `u8` 溢出） |

## 需求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用者** | [PrimeThreadScheduler](PrimeThreadScheduler.md)（所有方法）、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md)、[update_thread_stats](../apply.rs/update_thread_stats.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) |
| **依赖** | [ThreadHandle](../winapi.rs/ThreadHandle.md)、[IdealProcessorState](IdealProcessorState.md)、[ThreadPriority](../priority.rs/ThreadPriority.md)、`SYSTEM_THREAD_INFORMATION` (ntapi)、来自 `crate::collections` 的 `List` / `CONSUMER_CPUS` |
| **权限** | 无（仅数据结构；句柄获取需要适当的权限） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [scheduler.rs](README.md) |
| 父级容器 | [ProcessStats](ProcessStats.md) |
| 理想处理器状态 | [IdealProcessorState](IdealProcessorState.md) |
| 线程句柄 RAII 包装器 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| 周期预取 | [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) |
| 统计提交步骤 | [update_thread_stats](../apply.rs/update_thread_stats.md) |
| 模块地址解析 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| 迟滞选择 | [PrimeThreadScheduler](PrimeThreadScheduler.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
