# PrimeThreadScheduler 结构体 (scheduler.rs)

主线程调度引擎。维护每个进程、每个线程的统计数据，并实现基于迟滞的线程选择，以识别 CPU 活动最频繁的线程（"主线程"）并将其提升到专用处理器核心。迟滞机制通过要求线程在提升前保持高于进入阈值的活动，并允许它们只要保持在较低的保持阈值之上就能保持提升状态，从而防止颠簸。

## 语法

```rust
#[derive(Debug)]
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `pid_to_process_stats` | `HashMap<u32, ProcessStats>` | 从进程 ID 到 [ProcessStats](ProcessStats.md) 的映射，跟踪每个进程的线程统计信息、存活状态和调试配置。条目在首次访问时创建，并在进程通过 [drop_process_by_pid](#drop_process_by_pid) 退出时移除。 |
| `constants` | `ConfigConstants` | 来自服务配置的迟滞调优常量：`entry_threshold`、`keep_threshold` 和 `min_active_streak`。参见 [ConfigConstants](../config.rs/ConfigConstants.md)。 |

## 方法

### new

```rust
pub fn new(constants: ConfigConstants) -> Self
```

使用给定的迟滞常量创建一个新的 `PrimeThreadScheduler`，并附带一个空的进程统计映射。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `constants` | `ConfigConstants` | 控制线程提升/降级行为的迟滞调优参数。 |

**返回值**

`PrimeThreadScheduler` — 一个可供使用的新调度器实例。

---

### reset_alive

```rust
pub fn reset_alive(&mut self)
```

通过将所有 [ProcessStats](ProcessStats.md) 条目的 `alive` 设为 `false`，将所有跟踪的进程标记为已死亡。在每个轮询循环迭代开始时调用。仍在运行的进程将在快照处理过程中通过 [set_alive](#set_alive) 重新标记为存活。在迭代结束后仍为死亡状态的进程将通过 [drop_process_by_pid](#drop_process_by_pid) 被清理。

---

### set_alive

```rust
pub fn set_alive(&mut self, pid: u32)
```

将进程标记为当前迭代存活。如果给定 PID 不存在 [ProcessStats](ProcessStats.md) 条目，则用默认值创建一个。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 要标记为存活的进程标识符。 |

---

### set_tracking_info

```rust
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

为进程设置调试跟踪信息。当进程退出时，如果 `track_top_x_threads` 非零，则前 N 个按 CPU 周期排序的线程将被记录在诊断报告中。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程标识符。 |
| `track_top_x_threads` | `i32` | 在进程退出时要报告的前几个线程数。`0` 表示禁用跟踪。使用绝对值作为计数。 |
| `process_name` | `String` | 用于日志输出的进程显示名称。 |

---

### get_thread_stats

```rust
#[inline]
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

返回给定进程和线程的 [ThreadStats](ThreadStats.md) 的可变引用。如果进程或线程条目不存在，则用默认值创建。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 进程标识符。 |
| `tid` | `u32` | 线程标识符。 |

**返回值**

`&mut ThreadStats` — 线程统计条目的可变引用。

---

### update_active_streaks

```rust
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

根据每个线程的增量 CPU 周期（相对于所有线程的最大值）更新其 `active_streak` 计数器。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 拥有这些线程的进程标识符。 |
| `tid_with_delta_cycles` | `&[(u32, u64)]` | `(线程ID, 增量周期)` 对的切片，表示自上次测量以来的 CPU 周期增量。 |

**备注**

连续次数更新算法如下：

1. 计算 `max_cycles` — 切片中所有线程的最高增量。
2. 如果 `max_cycles == 0`，将所有连续次数重置为零并返回。
3. 对于每个线程：
   - **如果该线程已有连续次数 > 0：**
     - 如果 `delta < keep_threshold × max_cycles`，将连续次数重置为 0（线程降到保持阈值以下）。
     - 否则，连续次数递增 1，上限为 254。
   - **如果该线程没有连续次数：**
     - 如果 `delta >= entry_threshold × max_cycles`，将连续次数设为 1（线程进入活跃区域）。

`entry_threshold` 有意高于 `keep_threshold`，创建一个迟滞带，防止快速提升/降级循环。`active_streak` 计数器必须达到 `min_active_streak`，线程才能在 [select_top_threads_with_hysteresis](#select_top_threads_with_hysteresis) 中获得提升资格。

---

### select_top_threads_with_hysteresis

```rust
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
)
```

使用两阶段迟滞算法选择哪些线程应获得主线程状态。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 拥有这些线程的进程标识符。 |
| `tid_with_delta_cycles` | `&mut [(u32, u64, bool)]` | **\[输入/输出\]** `(线程ID, 增量周期, 是否为主线程)` 元组的可变切片。`is_prime` 字段对选为主线程的线程设为 `true`。`is_prime` 的输入值被忽略（被覆盖）。 |
| `slot_count` | `usize` | 可以提升到主线程状态的最大线程数（通常是专用 CPU 核心数）。 |
| `is_currently_assigned` | `fn(&ThreadStats) -> bool` | 回调函数，如果线程当前已分配到主资源（例如，具有非空的 `pinned_cpu_set_ids`），则返回 `true`。由第一阶段用来识别现任线程。 |

**返回值**

此函数不返回值。结果通过 `tid_with_delta_cycles` 中每个元组的 `is_prime` 字段传达。

**备注**

选择过程在两个阶段中对输入进行（首先按 `delta_cycles` 降序排序）：

| 阶段 | 目的 | 条件 |
|------|------|------|
| **第一（保留）** | 保留仍然合格且当前已分配的线程 | `is_currently_assigned(stats) == true` 且 `delta >= keep_threshold × max_cycles` |
| **第二（提升）** | 用新候选线程填充剩余槽位 | `delta >= entry_threshold × max_cycles` 且 `active_streak >= min_active_streak` 且线程尚未被选中 |

这种两阶段设计确保现任的主线程不会因微小的周期波动而被降级，而新线程必须在被提升之前展示持续的高活动性。`entry_threshold` 和 `keep_threshold` 之间的差距就是迟滞带。

---

### drop_process_by_pid

```rust
pub fn drop_process_by_pid(&mut self, pid: &u32)
```

从调度器中移除一个进程及其所有线程统计数据。关闭所有缓存的线程句柄，删除该进程的模块地址缓存，并可选择性地发出前几个线程的诊断报告。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `&u32` | 要移除的进程标识符的引用。 |

**备注**

- 如果 [ProcessStats](ProcessStats.md) 条目上的 `track_top_x_threads != 0`，则记录前 N 个线程（按 `last_cycles`）的报告。报告包含每个线程的详细信息：TID、周期计数、解析为模块名称的起始地址、内核时间、用户时间、创建时间、等待时间、优先级、基础优先级、上下文切换、线程状态和等待原因。
- 存储在 [ThreadStats](ThreadStats.md) 中的所有 [ThreadHandle](../winapi.rs/ThreadHandle.md) 实例被释放，这将关闭其底层的 Windows 句柄。
- 调用 [drop_module_cache](../winapi.rs/drop_module_cache.md) 来释放此进程的缓存模块枚举。
- 如果在映射中未找到 PID，函数立即返回，无任何效果。

## 备注

### 迟滞模型

调度器使用经典的迟滞（施密特触发器）模式来防止主线程和非主线程状态之间的颠簸：

```
                    ┌─────────────────────────────────┐
   entry_threshold  │  ← 线程必须超过此值才能          │
                    │    开始累计连续次数               │
                    ├─────────────────────────────────┤
   keep_threshold   │  ← 线程必须保持在此值之上         │
                    │    才能保持主线程状态             │
                    └─────────────────────────────────┘
```

`entry_threshold` 高于 `keep_threshold`。线程必须：
1. 超过 `entry_threshold × max_cycles` 以开始累积活动连续次数。
2. 保持其连续次数达到 `min_active_streak` 次迭代以获得提升资格。
3. 一旦提升，只有当其周期降到 `keep_threshold × max_cycles` 以下时才会失去主线程状态。

### 生命周期

1. **reset_alive** — 在每个循环迭代开始时调用。
2. **set_alive** / **set_tracking_info** — 在快照中找到每个进程时调用。
3. **update_active_streaks** — 在计算周期增量后调用。
4. **select_top_threads_with_hysteresis** — 调用来确定主线程分配。
5. **drop_process_by_pid** — 对未被标记为存活的（已退出）进程调用。

## 需求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用者** | `src/main.rs` 中的主轮询循环、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) |
| **被调用者** | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md)、[drop_module_cache](../winapi.rs/drop_module_cache.md)、[log_message](../logging.rs/log_message.md) |
| **依赖** | [ConfigConstants](../config.rs/ConfigConstants.md)、[ProcessStats](ProcessStats.md)、[ThreadStats](ThreadStats.md)、[ThreadHandle](../winapi.rs/ThreadHandle.md) |
| **权限** | 无直接要求；此调度器使用的线程句柄需要之前已获取的权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [scheduler.rs](README.md) |
| 每个进程的统计数据 | [ProcessStats](ProcessStats.md) |
| 每个线程的统计数据 | [ThreadStats](ThreadStats.md) |
| 理想处理器状态 | [IdealProcessorState](IdealProcessorState.md) |
| 线程句柄 RAII 包装器 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| 主线程应用 | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| 配置常量 | [ConfigConstants](../config.rs/ConfigConstants.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
