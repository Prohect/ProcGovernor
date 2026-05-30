# apply_prime_threads_promote 函数 (apply.rs)

通过 CPU 集合固定将选为主线程的线程提升到专用的高性能 CPU，并可选择性地提升其线程优先级。此函数是主线程调度算法的"奖励"阶段 — 表现出持续高 CPU 使用率的线程被授予对特定处理器核心的优先访问权，以提高缓存局部性并减少争用。

## 语法

```ProcGovernor/src/apply.rs#L811-819
pub fn apply_prime_threads_promote(
    pid: u32,
    config: &ThreadLevelConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ThreadLevelConfig`

线程级配置，包含：
- `prime_threads_cpus` — 要将主线程固定到的默认 CPU 索引集合。
- `prime_threads_prefixes` — 一个 [`PrimePrefix`](../config.rs/PrimePrefix.md) 规则列表，根据线程的启动地址模块名称覆盖默认 CPU 集合和线程优先级。
- `name` — 用于日志记录的配置规则名称。

`current_mask: &mut usize`

进程当前的亲和性掩码，用于过滤主 CPU 索引。如果掩码非零，则只有掩码中存在的 CPU 才会被用于固定（通过 `filter_indices_by_mask`）。这防止将线程分配到进程允许的亲和性范围之外的 CPU。

`tid_with_delta_cycles: &[(u32, u64, bool)]`

由 [`apply_prime_threads_select`](apply_prime_threads_select.md) 生成的 `(thread_id, delta_cycles, is_prime)` 元组的切片。只有 `is_prime == true` 的条目才会被此函数处理。

`prime_core_scheduler: &mut PrimeThreadScheduler`

跟踪每线程状态（包括缓存的句柄、启动地址、固定的 CPU 集合 ID 和原始线程优先级）的 [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) 实例。

`apply_config_result: &mut ApplyConfigResult`

变更消息和错误的累加器。参见 [`ApplyConfigResult`](ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果累加在 `apply_config_result` 中。

## 备注

### 每个线程的提升流程

对于每个标记为主线程（`is_prime == true`）的线程：

1. **如果已固定则跳过** — 如果 `thread_stats.pinned_cpu_set_ids` 非空，该线程已被提升，不执行任何操作。

2. **解析写句柄** — 从缓存的 `ThreadHandle` 获取可写线程句柄，优先使用完整访问句柄而非受限句柄。如果没有有效的句柄可用，则跳过该线程并通过 [`log_error_if_new`](log_error_if_new.md) 记录错误。

3. **解析启动模块** — 使用线程缓存的启动地址调用 `resolve_address_to_module` 来确定线程入口点属于哪个模块（DLL/EXE）。

4. **匹配前缀规则** — 遍历 `config.prime_threads_prefixes`，并对启动模块名称执行不区分大小写的前缀匹配。第一个匹配的 [`PrimePrefix`](../config.rs/PrimePrefix.md) 规则可以覆盖：
   - 要固定到的 CPU 集合（通过 `prefix.cpus`）。
   - 要设置的线程优先级（通过 `prefix.thread_priority`）。

   如果配置了前缀但无一匹配，则该线程**完全跳过**（不进行提升）。如果未配置前缀，则使用默认的 `config.prime_threads_cpus`。

5. **按亲和性掩码过滤 CPU** — 如果 `current_mask` 非零，目标 CPU 索引会被过滤为仅包含进程亲和性掩码允许的 CPU。

6. **通过 `SetThreadSelectedCpuSets` 固定** — 将目标 CPU 索引转换为 CPU 集合 ID，并调用 Windows API 固定线程。成功时，在 `thread_stats.pinned_cpu_set_ids` 中记录固定的 CPU 集合 ID，并记录一条包含线程 ID、提升的 CPU、周期计数和启动模块的变更消息。

7. **提升线程优先级** — 固定后，通过 `GetThreadPriority` 读取当前线程优先级并保存在 `thread_stats.original_priority` 中，以便稍后在降级时恢复。新优先级的确定方式如下：
   - 如果匹配的前缀指定了 `thread_priority`，则直接使用该值（记录为"priority set"）。
   - 否则，当前优先级通过 `ThreadPriority::boost_one()` 提升一级（记录为"priority boosted"）。

   仅当新值与当前值不同时才更改优先级。优先级提升无论 CPU 固定是否成功都会应用。

### 前缀匹配细节

- 匹配**不区分大小写** — 模块名称和前缀在比较前都被转换为小写。
- 仅使用**第一个**匹配的前缀；不评估后续前缀。
- 当配置了前缀但线程的模块不匹配任何前缀时，该线程**被排除在提升之外**。这允许有针对性地提升特定子系统（例如渲染线程、音频线程）。

### 错误处理

所有 Windows API 错误通过 [`log_error_if_new`](log_error_if_new.md) 报告并进行去重，防止持久性错误的日志刷屏。跟踪的操作包括：
- `Operation::OpenThread` — 无效的线程句柄。
- `Operation::SetThreadSelectedCpuSets` — CPU 集合固定失败。
- `Operation::SetThreadPriority` — 优先级提升失败。

### 变更消息

成功提升时，可能记录两条变更消息：
- `"Thread {tid} -> (promoted, [{cpus}], cycles={delta}, start={module})"` — CPU 集合固定。
- `"Thread {tid} -> (priority boosted: {old} -> {new})"` 或 `"Thread {tid} -> (priority set: {old} -> {new})"` — 优先级更改。

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **被调用方** | [`apply_prime_threads`](apply_prime_threads.md) |
| **调用** | [`log_error_if_new`](log_error_if_new.md)、`resolve_address_to_module`、`filter_indices_by_mask`、`cpusetids_from_indices`、`indices_from_cpusetids`、`format_cpu_indices` |
| **Win32 API** | [`SetThreadSelectedCpuSets`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets)、[`GetThreadPriority`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority)、[`SetThreadPriority`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority)、[`GetLastError`](https://learn.microsoft.com/zh-cn/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| **权限** | 需要对目标线程的 `THREAD_SET_INFORMATION` 和 `THREAD_QUERY_INFORMATION`（或受限变体）。 |

## 另请参阅

| | |
|---|---|
| [`apply_prime_threads`](apply_prime_threads.md) | 调用 select → promote → demote 的编排器。 |
| [`apply_prime_threads_select`](apply_prime_threads_select.md) | 基于迟滞选择要提升的线程。 |
| [`apply_prime_threads_demote`](apply_prime_threads_demote.md) | 撤销提升：取消固定 CPU 并恢复优先级。 |
| [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) | 管理每线程调度状态和迟滞逻辑。 |
| [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) | 线程级设置（包括主前缀）的配置。 |
| [`PrimePrefix`](../config.rs/PrimePrefix.md) | 带有可选 CPU 集合和线程优先级覆盖的每模块前缀规则。 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
