# apply_prime_threads_select 函数 (apply.rs)

使用基于迟滞的提升逻辑选择顶级线程获得主线程状态。此函数是主线程调度管道的决策层 — 它基于 CPU 周期增量和活动连续次数确定*哪些*线程符合主 CPU 固定资格，而不执行任何实际的系统调用。

## 语法

```ProcGovernor/src/apply.rs#L794-800
pub fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

## 参数

`pid: u32`

正在评估其线程的进程 ID。

`prime_count: usize`

可提升为主线程状态的最大线程数。通常等于配置的主 CPU 数量（`config.prime_threads_cpus.len()`）。

`tid_with_delta_cycles: &mut [(u32, u64, bool)]`

一个 `(thread_id, delta_cycles, is_prime)` 元组的可变切片。入口时，所有条目的 `is_prime` 字段为 `false`。出口时，被选中为主线程的线程的 `is_prime` 设置为 `true`。该切片应已填充当前调度间隔中的候选线程及其周期增量。

`prime_core_scheduler: &mut PrimeThreadScheduler`

拥有线程统计信息、迟滞常数和 `select_top_threads_with_hysteresis` 算法的 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 实例。

## 返回值

此函数不返回值。结果通过 `tid_with_delta_cycles` 中每个元组的 `is_prime` 字段原位写入。

## 备注

此函数完全委托给 [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md)，传递一个谓词，该谓词在 `pinned_cpu_set_ids` 非空时将线程视为"当前已分配"。迟滞算法应用两个阈值：

- **保持阈值** — 已是主线程的线程，如果其周期增量达到或超过所有候选者中最大周期增量的此百分比，则保持主线程状态。
- **进入阈值** — 当前不是主线程的线程必须超过此（更高的）最大比例百分比，*并且*在被提升前保持至少 `min_active_streak` 个间隔的活动连续次数。

这种双阈值方法可防止当多个线程具有相似的 CPU 利用率时在主线程和非主线程状态之间快速翻转（颠簸）。

此函数有意与 `apply_prime_threads_promote` 和 `apply_prime_threads_demote` 分离，以在 [`apply_prime_threads`](apply_prime_threads.md) 内维护清晰的 **选择 → 提升 → 降级** 管道。

### 谓词：`is_currently_assigned`

传递给迟滞选择器的闭包为：

```ProcGovernor/src/apply.rs#L801-803
|thread_stats| {
    !thread_stats.pinned_cpu_set_ids.is_empty()
}
```

如果一个线程在之前的提升过程中已被固定到一个或多个 CPU 集合 ID，则该线程被视为当前已分配（并有资格获得较低的保持阈值）。

## 需求

| 需求 | 值 |
|---|---|
| **模块** | `src/apply.rs` |
| **被调用方** | [`apply_prime_threads`](apply_prime_threads.md) |
| **调用** | [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md) |
| **Win32 API** | 无 |
| **权限** | 无（无系统调用） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | 调用 select → promote → demote 的编排器 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 将选中的主线程固定到 CPU 并提升优先级 |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | 取消固定并恢复失去主线程资格的线程 |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | 收集被选择过程消耗的周期数据 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 拥有迟滞逻辑和线程统计信息的调度器 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
