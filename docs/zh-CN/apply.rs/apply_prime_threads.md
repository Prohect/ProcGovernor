# apply_prime_threads 函数 (apply.rs)

通过识别 CPU 密集型线程并将其固定到指定的"主"CPU 上，为进程编排主线程调度，以提高缓存局部性和性能。这是主线程子系统的顶层入口点，协调选择、提升和降级阶段。

## 语法

```ProcGovernor/src/apply.rs#L708-718
pub fn apply_prime_threads<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

### `pid`

目标进程的进程 ID。

### `config`

包含主线程调度设置的 [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) 的引用，包括 `prime_threads_cpus`、`prime_threads_prefixes`、`track_top_x_threads` 和 `ideal_processor_rules`。

### `dry_run`

如果为 `true`，函数将在 `apply_config_result` 中记录预定的变更，而不进行任何 Windows API 调用。仅记录初始的主 CPU 描述。

### `current_mask`

当前进程亲和性掩码的可变引用。传递给 [`apply_prime_threads_promote`](apply_prime_threads_promote.md)，后者使用它根据进程亲和性来过滤主 CPU 集合。

### `process`

目标进程的 [`ProcessEntry`](../process.rs/ProcessEntry.md) 的引用。用于获取线程总数以确定候选池大小。

### `threads`

一个返回 `HashMap<u32, SYSTEM_THREAD_INFORMATION>` 引用的闭包，映射线程 ID 到其系统线程信息。传递给降级阶段以枚举活动线程。

### `prime_core_scheduler`

[`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，该调度器在迭代之间维护每线程状态（周期计数、固定 CPU 集合 ID、活动连续次数、句柄）。

### `apply_config_result`

[`ApplyConfigResult`](ApplyConfigResult.md) 的可变引用，用于累加变更和错误消息。

## 返回值

此函数不返回值。结果累加在 `apply_config_result` 中。

## 备注

### 算法

主线程算法分四个阶段进行：

1. **候选者构建** — 收集具有非零缓存周期计数的线程，并按总 CPU 时间增量（内核 + 用户时间）降序排序。候选池大小设为 `max(prime_count × 4, cpu_count)`，上限为线程总数。已退出顶级候选者池的之前被固定的线程会被重新加入，以确保它们能被正确降级。

2. **选择** — [`apply_prime_threads_select`](apply_prime_threads_select.md) 使用基于迟滞的选择，通过 [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md) 来确定哪些线程符合主线程资格。这防止了主线程和非主线程状态之间的快速翻转。

3. **提升** — [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 通过 `SetThreadSelectedCpuSets` 将新选中的主线程固定到指定的 CPU，并可选择性地提升其线程优先级。

4. **降级** — [`apply_prime_threads_demote`](apply_prime_threads_demote.md) 移除不再符合条件的线程的 CPU 集合固定，并恢复其原始线程优先级。

### 前置条件

在调用此函数之前，必须先调用 [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) 来填充缓存的周期计数并打开线程句柄。周期数据从 `PrimeThreadScheduler` 的每线程统计中获取。

### 跟踪模式

当 `track_top_x_threads` 非零时，函数启用跟踪模式，通过 `last_system_thread_info` 存储每个线程的 `SYSTEM_THREAD_INFORMATION` 快照。`track_top_x_threads` 为负值时禁用主调度阶段，但仍允许跟踪。

### 提前退出条件

在以下情况下函数立即返回：
- `prime_threads_cpus` 和 `prime_threads_prefixes` 均为空 **且** `track_top_x_threads` 为零。
- 在 `dry_run` 模式下，仅记录一条描述主 CPU 的变更消息。

### 候选池大小

候选池有意相对于主槽位数量进行过度采样（`prime_count × 4`），以便为迟滞算法提供足够的线程活动级别上下文。这确保降级候选者始终可见。

## 需求

| 需求 | 值 |
|---|---|
| 模块 | `apply` |
| 配置类型 | [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| 调用者 | 服务主循环（每进程强制执行） |
| 被调函数 | [`apply_prime_threads_select`](apply_prime_threads_select.md)、[`apply_prime_threads_promote`](apply_prime_threads_promote.md)、[`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| 前置条件 | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| Windows API | 无直接调用（委托给子函数） |
| 权限 | `THREAD_SET_INFORMATION`、`THREAD_QUERY_INFORMATION`（通过子函数） |

## 另请参阅

| 主题 | 链接 |
|---|---|
| 主线程选择 | [apply_prime_threads_select](apply_prime_threads_select.md) |
| 主线程提升 | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| 主线程降级 | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| 周期时间预取 | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 迟滞调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 线程级配置 | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| 结果累加器 | [ApplyConfigResult](ApplyConfigResult.md) |
| 模块概览 | [apply.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
