# reset_thread_ideal_processors 函数 (apply.rs)

在亲和性或 CPU 集合更改后重置进程中所有线程的理想处理器分配。通过按 CPU 时间（降序）排序线程并以轮询顺序分配理想处理器（带有随机偏移以避免确定性聚集）来在线程之间分配新的 CPU 集合。

## 语法

```ProcGovernor/src/apply.rs#L219-226
pub fn reset_thread_ideal_processors<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

匹配进程的 [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)。用于日志消息中的 `name` 字段以及打开线程句柄。

`dry_run: bool`

当为 `true` 时，将要更改的内容记录到 `apply_config_result` 中，而不调用任何 Windows API。

`cpus: &[u32]`

用于跨线程分配理想处理器的 CPU 索引集合。调用者在亲和性更改后传递 `&config.affinity_cpus`，或在 CPU 集合更改后（当 `cpu_set_reset_ideal` 设置时）传递 `&config.cpu_set_cpus`。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

返回进程的线程 ID 到 `SYSTEM_THREAD_INFORMATION` 映射的惰性访问器。

`apply_config_result: &mut ApplyConfigResult`

记录变更和错误的累加器。参见 [ApplyConfigResult](ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果累加在 `apply_config_result` 中。

## 备注

当 Windows 更改进程的亲和性掩码时，线程理想处理器分配可能变得过时 — 线程可能保留指向不再属于亲和性集合的 CPU 的理想处理器提示。此函数通过在新的 CPU 集合上重新分配理想处理器来纠正这一问题。

### 算法

1. **提前退出** — 如果 `cpus` 为空，立即返回。在试运行模式下，记录一条摘要变更消息并返回。
2. **收集线程时间** — 遍历所有线程并将每个 TID 与其总 CPU 时间（`KernelTime + UserTime`）配对。
3. **降序排序** — 按总 CPU 时间降序排序线程，使最繁忙的线程首先被分配。
4. **随机偏移** — 生成一个随机 `u8` 偏移值以随机化 CPU 列表中的起始位置。这防止了连续调用中相同的 CPU 始终接收最高活动量的线程。
5. **轮询分配** — 对于每个线程（按排序顺序）：
   - 通过 [`get_thread_handle`](../winapi.rs/get_thread_handle.md) 打开线程句柄。
   - 选择写句柄，优先使用 `w_handle` 而非 `w_limited_handle`。
   - 计算目标 CPU 为 `cpus[(success_count + random_shift) % cpu_count]`。
   - 使用处理器组 0 和目标 CPU 编号调用 `SetThreadIdealProcessorEx`。
   - 失败时，通过 [`log_error_if_new`](log_error_if_new.md) 记录。成功时，增加成功计数器。
6. **摘要** — 记录一条变更条目：`"reset ideal processor for {N} threads"`。

### 平台说明

- `SetThreadIdealProcessorEx` 是对 Windows 调度器的提示，而非硬约束。操作系统仍可能在其它 CPU 上调度该线程。
- 所有分配使用处理器组 0。具有超过 64 个逻辑处理器且跨多个组的系统未被完全处理。
- 此函数打开的线程句柄在使用后立即丢弃（不缓存在 `PrimeThreadScheduler` 中）。

### 边界情况

- 如果没有线程可以被打开（例如，所有句柄失败），摘要消息报告 `"reset ideal processor for 0 threads"`。
- 随机偏移通过模运算进行回绕，因此对任何 `random::<u8>()` 值都是安全的。

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **被调用方** | [apply_affinity](apply_affinity.md)（在成功 `SetProcessAffinityMask` 之后）、[apply_process_default_cpuset](apply_process_default_cpuset.md)（当 `cpu_set_reset_ideal` 为 `true` 时） |
| **调用** | [`get_thread_handle`](../winapi.rs/get_thread_handle.md)、[`set_thread_ideal_processor_ex`](../winapi.rs/set_thread_ideal_processor_ex.md)、[`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **权限** | 每个线程上的 `THREAD_SET_INFORMATION`（或 `THREAD_SET_LIMITED_INFORMATION`） |

## 另请参阅

| | |
|---|---|
| [apply_affinity](apply_affinity.md) | 设置进程亲和性掩码；成功时调用此函数 |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | 设置进程默认 CPU 集合；可选调用此函数 |
| [apply_ideal_processors](apply_ideal_processors.md) | 用于线程级配置的基于规则的理想处理器分配 |
| [ApplyConfigResult](ApplyConfigResult.md) | 变更和错误的累加器 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 进程级配置结构体 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
