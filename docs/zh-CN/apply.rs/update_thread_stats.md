# update_thread_stats 函数 (apply.rs)

将当前周期时间和总时间作为"上次"值缓存，以供下一次迭代的增量计算使用。此函数必须在每个应用周期结束时调用，以确保下一个周期计算准确的线程 CPU 使用增量。

## 语法

```ProcGovernor/src/apply.rs#L1312-1315
pub fn update_thread_stats(
    pid: u32,
    prime_scheduler: &mut PrimeThreadScheduler,
)
```

## 参数

`pid: u32`

要更新其线程统计信息的进程 ID。

`prime_scheduler: &mut PrimeThreadScheduler`

拥有每线程统计信息缓存的 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用。调度器的 `pid_to_process_stats` 映射针对给定的 `pid` 查询。

## 返回值

此函数不返回值。

## 备注

`update_thread_stats` 为给定进程下跟踪的每个线程执行两次转移：

1. **周期快照** — `cached_cycles` 被复制到 `last_cycles`，然后 `cached_cycles` 被清零。这为下一次调用 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 准备了基线，该函数计算 `delta_cycles = cached_cycles - last_cycles`。

2. **总时间快照** — `cached_total_time` 被复制到 `last_total_time`，然后 `cached_total_time` 被清零。总时间（内核 + 用户）用于按 CPU 消耗对线程排序以及计算基于时间的增量。

只有缓存值大于零的条目才会被更新。这避免覆盖在当前周期中未被测量的线程（例如，在 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 中处于候选池之外的线程）的有效 `last_*` 值。

如果 `pid` 不在 `pid_to_process_stats` 中，函数静默返回，无任何效果。

### 调用顺序

在典型的应用周期中，调用顺序为：

1. [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) — 查询当前周期/时间值并将其存储在 `cached_*` 字段中。
2. [apply_prime_threads](apply_prime_threads.md) / [apply_ideal_processors](apply_ideal_processors.md) — 消耗缓存的增量以进行线程选择和调度决策。
3. **`update_thread_stats`** — 将缓存值快照到 `last_*` 字段并清除缓存。

省略此调用将导致增量在多个周期中累积，从而在主线程和理想处理器算法中产生错误的线程排名。

## 需求

| | |
|---|---|
| **模块** | [apply.rs](README.md) |
| **调用者** | 服务主循环（在所有每进程应用函数完成之后） |
| **被调函数** | 无（纯数据簿记） |
| **API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | 填充此函数快照的 `cached_cycles` 和 `cached_total_time` |
| [apply_prime_threads](apply_prime_threads.md) | 消耗周期增量以进行主线程选择 |
| [apply_ideal_processors](apply_ideal_processors.md) | 消耗周期增量以进行理想处理器分配 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 拥有每进程、每线程统计信息映射 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
