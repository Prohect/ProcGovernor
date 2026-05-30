# apply_prime_threads_demote 函数 (apply.rs)

对不再符合主线程资格的线程进行降级，移除其 CPU 集合固定并恢复其原始线程优先级。

## 语法

```ProcGovernor/src/apply.rs#L953-961
pub fn apply_prime_threads_demote<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

其线程可能被降级的目标进程的进程标识符。

`config: &ThreadLevelConfig`

包含用于日志记录的进程名称的线程级配置。参见 [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md)。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

返回进程所有活动线程 ID 及其系统线程信息的惰性访问器。用于枚举可能需要降级的线程。

`tid_with_delta_cycles: &[(u32, u64, bool)]`

由先前管道阶段生成的候选线程列表。每个元组为 `(thread_id, delta_cycles, is_prime)`。`is_prime` 标志指示应**保持**主线程的线程；所有其它具有非空 `pinned_cpu_set_ids` 的线程被降级。

`prime_core_scheduler: &mut PrimeThreadScheduler`

持有每线程状态（包括缓存的句柄、固定的 CPU 集合 ID 和原始线程优先级）的主线程调度器。参见 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)。

`apply_config_result: &mut ApplyConfigResult`

变更和错误消息的累加器。参见 [ApplyConfigResult](ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果记录在 `apply_config_result` 中。

## 备注

此函数是主线程调度管道的第三阶段也是最后阶段，由 [apply_prime_threads](apply_prime_threads.md) 在 [apply_prime_threads_select](apply_prime_threads_select.md) 和 [apply_prime_threads_promote](apply_prime_threads_promote.md) 之后调用。

### 降级逻辑

1. **识别主线程集合** — 构建 `tid_with_delta_cycles` 中当前标记为 `is_prime == true` 的线程 ID 的 `HashSet`。
2. **枚举活动线程** — 从 `threads()` 访问器收集所有活动线程 ID。
3. **过滤候选者** — 对于每个活动线程，跳过仍是主线程的或具有空 `pinned_cpu_set_ids`（从未被提升）的线程。
4. **移除 CPU 集合固定** — 调用 `SetThreadSelectedCpuSets` 并传入空切片来清除任何 CPU 集合分配，允许线程在任何处理器上运行。
5. **清除固定状态** — 无论 `SetThreadSelectedCpuSets` 调用成功还是失败，始终清除 `pinned_cpu_set_ids`。这可以防止产生无限重试循环而刷屏错误日志。
6. **恢复线程优先级** — 如果在提升期间保存了 `original_priority`，则通过 `SetThreadPriority` 将线程恢复到其先前的优先级。

### 错误弹性

该函数有意在 `SetThreadSelectedCpuSets` 失败时也清除 `pinned_cpu_set_ids`。这种设计选择优先考虑避免日志刷屏而非保证清理。如果 API 调用失败（例如，由于线程句柄已失效），线程将在退出或被重新创建时自然失去其固定。

错误通过 [log_error_if_new](log_error_if_new.md) 去重，因此每个唯一的 `(pid, tid, operation, error_code)` 组合仅被报告一次。

### 线程句柄选择

写句柄优先于受限写句柄。如果两者均无效，则跳过该线程并记录错误。

### 优先级恢复

该函数使用 `thread_stats.original_priority.take()` 来消耗存储的优先级。这确保优先级仅恢复一次，防止在后续周期中重复恢复。

### 示例变更消息

成功降级时：
- `Thread 1234 -> (demoted, start=ntdll.dll)`

优先级恢复失败时，通过 `log_error_if_new` 记录错误。

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **被调用方** | [apply_prime_threads](apply_prime_threads.md) |
| **调用** | `SetThreadSelectedCpuSets`、`SetThreadPriority`、`resolve_address_to_module`、[log_error_if_new](log_error_if_new.md) |
| **Win32 API** | [SetThreadSelectedCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets)、[SetThreadPriority](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| **权限** | 目标线程上的 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION` |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [apply_prime_threads](apply_prime_threads.md) | 主线程调度管道的编排函数 |
| [apply_prime_threads_select](apply_prime_threads_select.md) | 选择哪些线程符合主线程资格 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 通过 CPU 固定和优先级提升来提升选中的线程 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 管理每线程调度状态和迟滞 |
| [ApplyConfigResult](ApplyConfigResult.md) | 变更和错误报告的累加器 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
