# main 函数 (main.rs)

ProcGovernor 的应用程序入口点。解析命令行参数，分派到相应的操作模式，并 — 对于默认服务模式 — 运行主轮询循环，该循环在运行的 Windows 进程上强制执行进程和线程配置。

## 语法

```rust
fn main() -> windows::core::Result<()>
```

## 返回值

`windows::core::Result<()>` — 在正常关闭时返回 `Ok(())`。从快照创建或 CLI 解析中传播 Windows 错误。

## 备注

该函数实现了一个多阶段启动，随后是一个连续的强制执行循环。

### 阶段 1 — CLI 分派

1. **解析参数** — 调用 [`parse_args`](../cli.rs/parse_args.md) 填充 [`CliArgs`](../cli.rs/CliArgs.md) 结构体。
2. **模式分派** — 按顺序检查提前退出模式：
   - `-help` → [`print_help`](../cli.rs/print_help.md) 并返回。
   - `-helpAll` → [`print_help_all`](../cli.rs/print_help_all.md) 并返回。
   - `-convert` → [`convert`](../config.rs/convert.md) 并返回。
   - `-autogroup` → [`sort_and_group_config`](../config.rs/sort_and_group_config.md) 并返回。
   - `-validate` → 读取配置，打印验证报告，并返回。
   - `-processLogs` → [`process_logs`](process_logs.md) 并返回。

### 阶段 2 — 配置和权限设置

3. **读取配置** — 调用 [`read_config`](../config.rs/read_config.md) 将 INI 格式的配置文件解析为 [`ConfigResult`](../config.rs/ConfigResult.md)。打印配置报告。如果发现错误，退出。
4. **读取黑名单** — 可选地读取要忽略的进程名称黑名单文件。
5. **空检查** — 如果配置和黑名单都为空且未启用查找模式，退出。
6. **启用权限** — 为当前进程令牌调用 [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md) 和 [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md)。
7. **计时器分辨率** — 调用 [`set_timer_resolution`](../winapi.rs/set_timer_resolution.md) 将系统计时器设置为配置的分辨率。
8. **UAC 提升** — 如果未以管理员身份运行且未设置 `-noUAC`，调用 [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md) 以提升权限重新启动进程。
9. **终止子进程** — 调用 [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) 清理先前的非提升实例。

### 阶段 3 — ETW 监视器

10. **启动 ETW** — 除非设置了 `-noETW`，启动一个 [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md)，通过 `mpsc` 通道传递进程启动/停止事件。如果 ETW 失败，回退到纯轮询模式。

### 阶段 4 — 主循环

循环重复直到关闭（试运行在完成一次迭代后停止、`-loop` 限制迭代次数、或 ETW 通道断开）：

11. **拍摄快照** — 调用 [`ProcessSnapshot::take`](../process.rs/ProcessSnapshot.md) 枚举所有运行中的进程。
12. **匹配规则** — 遍历分级的配置规则。对于每个等级和进程名称匹配，调用 [`apply_config`](apply_config.md)，它处理进程级和线程级应用。
    - **ETW 挂起列表** — 从 ETW 事件接收的进程通过 `process_level_pending` 被积极应用，在保持循环中通过匹配快照数据来排空。
    - **完整匹配 vs. 分级** — 第一次循环迭代（以及配置重载后）对所有进程进行完整匹配。后续迭代仅以配置的等级间隔匹配进程。
    - **连续应用** — 当设置了 `-continuousProcessLevelApply` 时，进程级配置每次迭代都被重新应用。否则，`process_level_applied` 跟踪已配置的 PID。
13. **线程级遍历** — 在组合遍历之后，为已由 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 初始化的进程运行专用的仅线程级遍历，在每个等级间隔应用基于周期时间的调度。
14. **清理** — 从 `prime_core_scheduler`、`process_level_applied` 和失败映射中移除已终止的 PID。
15. **查找模式** — 调用 [`process_find`](process_find.md) 记录未管理的进程。
16. **刷新日志** — 刷新主日志记录器和查找日志记录器。

### 阶段 5 — 休眠和热重载

17. **ETW 响应式休眠** — 当没有活动的线程级跟踪且 ETW 可用时，循环以 `(interval_ms + 16) / 2` 毫秒的超时时间在 ETW 通道上阻塞。进程启动事件被排队到 `process_level_pending`；停止事件从跟踪中移除 PID。当挂起项累积了足够长的时间（大约 `interval_ms`）后循环中断。
18. **轮询休眠** — 当 ETW 休眠不适用时（活动线程级跟踪、ETW 禁用或 `-continuousProcessLevelApply`），回退到简单的 `thread::sleep` 休眠 `interval_ms`。
19. **热重载** — 调用 [`hotreload_config`](../config.rs/hotreload_config.md) 和 [`hotreload_blacklist`](../config.rs/hotreload_blacklist.md) 检测文件修改并在更改时重载。配置重载时，重置 `process_level_applied` 并在下一次迭代触发完整匹配。

### 阶段 6 — 关闭

20. **停止 ETW** — 调用 `EtwProcessMonitor::stop()` 拆除 ETW 跟踪会话。

### 关键状态变量

| 变量 | 类型 | 用途 |
|----------|------|---------|
| `process_level_applied` | `SmallVec<[u32; PIDS]>` | 已接收进程级配置的 PID。防止冗余的重新应用。 |
| `thread_level_applied` | `SmallVec<[u32; PENDING]>` | 在当前迭代中已接收线程级配置的 PID。每次循环清除。 |
| `process_level_pending` | `SmallVec<[u32; PENDING]>` | 从 ETW 进程启动事件接收的等待应用的 PID。 |
| `full_process_level_match` | `bool` | 当为 `true` 时，所有进程无论等级都被匹配。在第一次循环和配置重载时设置。 |
| `current_loop` | `u32` | 单调递增的循环计数器。用于基于等级的取模调度。 |

### ETW 休眠算法

ETW 响应式休眠在只有进程级配置处于活动状态时避免了固定间隔轮询。循环在 ETW 通道上等待，而不是休眠 `interval_ms`：

- 在**超时**且挂起列表为空时，继续等待。
- 在**超时**且挂起列表非空时，中断以处理挂起项。
- 在**进程启动事件**时，将 PID 添加到挂起列表；当经过足够的墙上时钟时间后中断。
- 在**进程停止事件**时，从所有跟踪结构中移除 PID。
- 在**通道断开**时，设置 `should_continue = false`（另一个实例可能已接管 ETW 会话）。

这导致空闲期间 CPU 使用率更低，同时保持对新进程启动的快速响应。

## 需求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用者** | Rust 运行时（`fn main`） |
| **被调函数** | [`parse_args`](../cli.rs/parse_args.md)、[`read_config`](../config.rs/read_config.md)、[`apply_config`](apply_config.md)、[`apply_thread_level`](apply_thread_level.md)、[`process_find`](process_find.md)、[`process_logs`](process_logs.md)、[`log_apply_results`](log_apply_results.md)、[`EtwProcessMonitor::start`](../event_trace.rs/EtwProcessMonitor.md)、[`ProcessSnapshot::take`](../process.rs/ProcessSnapshot.md)、[`hotreload_config`](../config.rs/hotreload_config.md)、[`hotreload_blacklist`](../config.rs/hotreload_blacklist.md)、[`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md)、[`request_uac_elevation`](../winapi.rs/request_uac_elevation.md)、[`set_timer_resolution`](../winapi.rs/set_timer_resolution.md)、[`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) |
| **Win32 API** | [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/zh-cn/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)（通过 `ProcessSnapshot`）、ETW 通过 [`EVENT_TRACE_PROPERTIES`](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/) |
| **权限** | `SeDebugPrivilege`、`SeIncreaseBasePriorityPrivilege`、管理员（通过 UAC） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概览 | [main.rs](README.md) |
| CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |
| 配置结果 | [ConfigResult](../config.rs/ConfigResult.md) |
| 应用引擎 | [apply.rs](../apply.rs/README.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| ETW 进程监视器 | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
