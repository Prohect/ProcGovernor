# apply_config 函数 (main.rs)

为单个匹配的进程同时应用进程级和线程级配置的组合入口点。创建一个共享的线程缓存，使每个进程每次迭代最多枚举一次线程，然后委托给 [apply_process_level](apply_process_level.md) 和 [apply_thread_level](apply_thread_level.md)。跟踪哪些 PID 已在每个级别被处理，并记录合并后的结果。

## 语法

```rust
fn apply_config(
    cli: &CliArgs,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    job_manager: &mut JobObjectManager,
    process_level_applied: &mut SmallVec<[u32; PIDS]>,
    thread_level_applied: &mut SmallVec<[u32; PENDING]>,
    grade: &u32,
    pid: &u32,
    name: &&str,
    process_level_config: &ProcessLevelConfig,
    process: &ProcessEntry,
)
```

## 参数

`cli: &CliArgs`

解析后的 [CLI 参数](../cli.rs/CliArgs.md)。用于读取 `cli.dry_run`，该值被转发给两个应用函数。

`configs: &ConfigResult`

完整的 [ConfigResult](../config.rs/ConfigResult.md)。用于通过 `grade` 和 `name` 从 `configs.thread_level_configs` 中查找匹配的 [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md)。

`prime_core_scheduler: &mut PrimeThreadScheduler`

[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 实例，传递给 [apply_thread_level](apply_thread_level.md) 以进行主线程跟踪和调度。

`job_manager: &mut JobObjectManager`

缓存和管理用于内核强制 CPU 亲和性的命名 Windows Job Objects 的管理器。传递给 [`apply_process_level`](apply_process_level.md)。参见 [`JobObjectManager`](../job_object.rs/JobObjectManager.md)。

`process_level_applied: &mut SmallVec<[u32; PIDS]>`

已应用进程级设置的 PID 的运行列表。在 [apply_process_level](apply_process_level.md) 完成后，当前 `pid` 被无条件追加。在后续迭代中（除非设置了 `-continuous_process_level_apply`），此列表中的 PID 会被跳过进程级工作。

`thread_level_applied: &mut SmallVec<[u32; PENDING]>`

在*当前*迭代中已应用线程级设置的 PID 的运行列表。仅当 [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) 存在且调用了 [apply_thread_level](apply_thread_level.md) 时才追加当前 `pid`。这防止同一进程在单个循环迭代中被处理两次（这会被调度器的基于增量的周期时间跟踪所破坏）。

`grade: &u32`

匹配规则所在的配置等级（轮询频率乘数）。用于在 `configs.thread_level_configs` 中查找对应的线程级配置。

`pid: &u32`

目标进程的进程标识符。

`name: &&str`

目标进程的小写可执行文件名称（例如 `"chrome.exe"`）。用作查找线程级配置的键。

`process_level_config: &ProcessLevelConfig`

匹配此进程的 [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)。转发给 [apply_process_level](apply_process_level.md)。

`process: &ProcessEntry`

当前快照中的 [ProcessEntry](../process.rs/ProcessEntry.md)，用于通过 `process.get_threads()` 枚举线程。

## 返回值

此函数不返回值。所有结果通过 `process_level_applied`、`thread_level_applied` 以及 [log_apply_results](log_apply_results.md) 产生的日志输出进行传递。

## 备注

### 线程缓存

该函数创建一个 `OnceCell<HashMap<u32, SYSTEM_THREAD_INFORMATION>>`，惰性评估 `process.get_threads()`。此单元格通过闭包引用在 [apply_process_level](apply_process_level.md) 和 [apply_thread_level](apply_thread_level.md) 之间共享，确保无论有多少应用函数需要线程数据，每次调用最多进行一次线程枚举（涉及 `NtQuerySystemInformation`）。

### 两级应用流程

1. 创建一个新的 [ApplyConfigResult](../apply.rs/ApplyConfigResult.md)。
2. 使用进程级配置无条件调用 [apply_process_level](apply_process_level.md)。
3. 然后函数在 `configs.thread_level_configs[grade][name]` 中查找线程级配置。如果存在，调用 [apply_thread_level](apply_thread_level.md) 并将 PID 添加到 `thread_level_applied`。
4. PID 始终被添加到 `process_level_applied`。
5. 使用累积的结果调用 [log_apply_results](log_apply_results.md)。

### 等级不变量

该函数（按约定）断言用于查找进程级配置的等级与用于线程级配置查找的等级相同。这由主循环中的调用者保证，调用者按等级迭代配置。

## 需求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用者** | [main](main.md) 循环 — ETW 挂起路径和完整匹配路径 |
| **被调函数** | [apply_process_level](apply_process_level.md)、[apply_thread_level](apply_thread_level.md)、[log_apply_results](log_apply_results.md)、`ProcessEntry::get_threads` |
| **Win32 API** | 无直接调用（委托给被调函数） |
| **权限** | 无直接依赖（委托给被调函数） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 进程级应用包装器 | [apply_process_level](apply_process_level.md) |
| 线程级应用包装器 | [apply_thread_level](apply_thread_level.md) |
| 结果日志记录 | [log_apply_results](log_apply_results.md) |
| 结果累加器 | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| 配置类型 | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)、[ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| 模块概览 | [main.rs](README.md) |

*文档记录于提交：[e8d16f2](https://github.com/Prohect/ProcGovernor/tree/e8d16f2bb3258b3aa6d761002188fe68b71ca85f)*
