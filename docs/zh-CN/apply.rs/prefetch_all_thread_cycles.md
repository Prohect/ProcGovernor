# prefetch_all_thread_cycles 函数 (apply.rs)

为进程的顶级 CPU 消耗线程预取线程周期计数，为基于迟滞的主线程提升/降级算法建立基线测量。

此函数为按内核+用户时间排序的线程打开句柄，通过 `QueryThreadCycleTime` 查询其周期计数器，根据缓存值计算周期增量，并使用活动连续次数信息更新 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)。它还会解析线程启动地址以供后续模块匹配使用。

## 语法

```ProcGovernor/src/apply.rs#L585-591
pub fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

正在测量其线程的进程 ID。

`config: &ThreadLevelConfig`

进程的线程级配置。`name` 字段用于错误日志记录。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

返回线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 结构体（来自 `NtQuerySystemInformation`）映射的惰性访问器。

`prime_scheduler: &mut PrimeThreadScheduler`

存储每线程周期/时间缓存、活动连续次数和线程句柄的调度器的可变引用。使用新的测量值就地更新。

`apply_config_result: &mut ApplyConfigResult`

周期时间查询期间遇到的错误的累加器。

## 返回值

此函数不返回值。结果作为副作用存储在 `prime_scheduler` 中：

- `cached_cycles` — 每线程的当前原始周期计数。
- `cached_total_time` — 每线程的当前内核+用户时间。
- 活动连续次数通过 [`PrimeThreadScheduler::update_active_streaks`](../scheduler.rs/PrimeThreadScheduler.md) 更新。

## 备注

### 算法

1. **计算时间增量** — 对于进程中的每个线程，计算 `cached_total_time`（内核 + 用户）以及来自 `last_total_time` 的增量。将结果存储在按增量时间降序排序的列表中。

2. **清理已终止的线程** — 从 `pid_to_process_stats` 中移除不再存在的线程条目。丢弃已终止线程的缓存线程句柄以避免句柄泄漏。

3. **限制候选者数量** — 仅处理顶级线程，上限为 `min(cpu_count * 2, thread_count)`。这限制了具有数百或数千个线程的进程的开销。

4. **打开线程句柄** — 对于每个候选线程，通过 [`get_thread_handle`](../winapi.rs/get_thread_handle.md) 打开线程句柄（如果尚未缓存在 `thread_stats.handle` 中）。句柄被缓存以供跨迭代重用。

5. **解析启动地址** — 如果 `thread_stats.start_address` 为零，通过 [`get_thread_start_address`](../winapi.rs/get_thread_start_address.md) 查询线程启动地址。这在后续主线程提升期间用于模块前缀匹配。

6. **查询周期时间** — 调用 `QueryThreadCycleTime` 读取线程的 CPU 周期计数器。该值存储在 `cached_cycles` 中。

7. **计算周期增量** — 在所有查询之后，为每个具有非零缓存周期的线程计算 `cached_cycles - last_cycles`。缓存周期为零的线程将其 `active_streak` 重置为 0。

8. **更新活动连续次数** — 使用周期增量列表调用 `PrimeThreadScheduler::update_active_streaks`。周期超过保持阈值的线程其连续次数增加；其他线程被重置。

### 线程句柄缓存

线程句柄打开一次并存储在 `thread_stats.handle` 中。这避免了在每个轮询间隔重复调用 `OpenThread`。当线程从统计映射中被清理时，句柄会自动丢弃。

### 候选池大小

候选池大小设为 `cpu_count * 2`（基于 CPU 集合信息），确保跟踪足够多的线程以处理变动而不产生过多开销。池始终至少包含线程数减一。

### 平台说明

- `QueryThreadCycleTime` 返回 CPU 周期（而非墙上时钟时间），提供高分辨率、与调度无关的线程活动度量。
- 该函数倾向于使用完整访问读句柄（`r_handle`）而非受限句柄（`r_limited_handle`）。

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **被调用方** | [`apply_prime_threads`](apply_prime_threads.md)、主轮询循环 |
| **调用** | [`get_thread_handle`](../winapi.rs/get_thread_handle.md)、[`get_thread_start_address`](../winapi.rs/get_thread_start_address.md)、[`PrimeThreadScheduler::update_active_streaks`](../scheduler.rs/PrimeThreadScheduler.md)、[`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | [`QueryThreadCycleTime`](https://learn.microsoft.com/zh-cn/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) |
| **权限** | 目标线程上的 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION` |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | 在主线程选择之前调用此函数的编排器 |
| [apply_prime_threads_select](apply_prime_threads_select.md) | 使用此处计算的周期增量进行基于迟滞的选择 |
| [update_thread_stats](update_thread_stats.md) | 在应用周期完成后将缓存值复制到 `last_*` 字段 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 存储所有每线程统计信息的调度器结构体 |
| [ApplyConfigResult](ApplyConfigResult.md) | 变更和错误的累加器 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
