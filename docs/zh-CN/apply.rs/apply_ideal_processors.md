# apply_ideal_processors 函数 (apply.rs)

基于可配置的 [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md) 条目为线程分配理想处理器提示。每个规则指定一组 CPU 和可选的模块名前缀；函数通过启动地址模块来匹配线程，使用迟滞机制选择前 *N* 个线程（其中 *N* = 规则中的 CPU 数量），并以轮询方式将每个选中的线程分配到一个专用的理想 CPU。退出前 *N* 名的线程的理想处理器将恢复到分配前观察到的值。

## 语法

```ProcGovernor/src/apply.rs#L1048-1057
pub fn apply_ideal_processors<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 目标进程的进程 ID。 |
| `config` | `&ThreadLevelConfig` | 包含 `ideal_processor_rules` 的线程级配置。 |
| `dry_run` | `bool` | 当为 `true` 时，记录将要更改的内容而不调用任何 Windows API。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 返回进程线程映射的惰性访问器。 |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | 拥有每线程统计数据（周期缓存、理想处理器跟踪状态）的调度器。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 变更和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果记录在 `apply_config_result` 中。

## 备注

### 算法

对于 `config.ideal_processor_rules` 中的每个 [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md)：

1. **模块匹配** — 每个启动地址模块与规则中某一 `prefixes`（不区分大小写）匹配的线程被视为候选者。如果 `prefixes` 为空，所有线程均为候选者。

2. **带迟滞的选择** — 候选者被送入 [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md)，槽位数量等于 `rule.cpus.len()`。`is_currently_assigned` 谓词检查 `thread_stats.ideal_processor.is_assigned`，从而在轮询间隔之间稳定线程选择。

3. **保留现有分配** — 如果仍被选中，之前已分配理想 CPU 的线程保留其 CPU 槽位。其 CPU 被添加到 `claimed` 集合中以避免重复分配。

4. **新分配** — 已选中但尚未分配的新线程通过轮询方式从空闲池（不在 `claimed` 中的规则 CPU）中获得一个 CPU。调用 API `SetThreadIdealProcessorEx`，使用处理器组 0 和目标 CPU 编号。

5. **恢复** — 之前被分配但不再被选中的线程的理想处理器恢复到首次分配前捕获的值（`previous_group`、`previous_number`）。`is_assigned` 标志被清除。

### 理想处理器状态跟踪

每个线程的理想处理器状态跟踪在 `ThreadStats.ideal_processor` 中：

| 字段 | 用途 |
|-------|---------|
| `is_assigned` | 此函数当前是否拥有该线程的理想处理器。 |
| `previous_group` / `previous_number` | 首次分配前的理想处理器，用于恢复。 |
| `current_group` / `current_number` | 此函数最近设置的理想处理器。 |

第一次被选中时，调用 `GetThreadIdealProcessorEx` 来捕获基线。如果线程当前的理想处理器已在 `rule.cpus` 内，则保留而不进行冗余的 `Set` 调用。

### 试运行行为

当 `dry_run` 为 `true` 时，函数为每个规则记录一条摘要变更，指示 CPU 集合和前缀过滤器，然后返回而不打开任何线程句柄或调用 Windows API。

### 错误处理

来自 `GetThreadIdealProcessorEx` 和 `SetThreadIdealProcessorEx` 的错误通过 [`log_error_if_new`](log_error_if_new.md) 报告，按 `(pid, tid, operation, error_code)` 去重。无效的线程句柄被记录一次，且该线程被跳过。

### 平台说明

- 理想处理器是对 Windows 调度器的一个*提示*，而非硬性约束。操作系统仍可能在其它 CPU 上调度该线程。
- 仅支持的处理器组 0 的分配。基线理想处理器在组 0 内且在 `rule.cpus` 内的线程被声明而不调用 `Set`。

## 需求

| | |
|---|---|
| **模块** | `apply` (`src/apply.rs`) |
| **调用者** | 服务轮询循环（通过顶层应用编排器） |
| **被调函数** | [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md)、`resolve_address_to_module` (`winapi`)、`get_thread_ideal_processor_ex` / `set_thread_ideal_processor_ex` (`winapi`)、[`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | `SetThreadIdealProcessorEx`、`GetThreadIdealProcessorEx`（通过 `winapi` 包装器） |
| **权限** | `THREAD_SET_INFORMATION`（写句柄）、`THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION`（读句柄） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 结果累加器 | [ApplyConfigResult](ApplyConfigResult.md) |
| 线程级配置 | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| 理想处理器规则定义 | [IdealProcessorRule](../config.rs/IdealProcessorRule.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 周期预取（前置条件） | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 线程统计快照 | [update_thread_stats](update_thread_stats.md) |
| 模块概览 | [apply.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
