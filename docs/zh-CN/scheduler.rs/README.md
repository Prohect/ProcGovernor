# scheduler 模块 (ProcGovernor)

`scheduler` 模块实现了主线程调度引擎——核心算法，用于识别进程中 CPU 活动最频繁的线程，并跟踪其活动以做出 CPU 绑定决策。它使用基于迟滞的选择来防止提升/降级颠簸：线程必须在进入阈值之上保持活动达到最低连续次数才能被提升，并且必须降到单独的（更低的）保持阈值之下才会被降级。

该模块在多次轮询迭代中维护每个进程和每个线程的统计数据，包括周期计数器、连续次数计数器、线程句柄、理想处理器分配以及缓存的优先级信息。

## 结构体

| 名称 | 描述 |
|------|------|
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | 顶层调度器结构体，拥有每个进程的统计数据和迟滞常量。提供存活跟踪、连续次数更新和基于迟滞的线程选择方法。 |
| [ProcessStats](ProcessStats.md) | 每个进程的状态容器，持有线程统计映射、存活标志、跟踪配置和进程元数据。 |
| [IdealProcessorState](IdealProcessorState.md) | 每个线程的理想处理器分配状态，跟踪当前和先前的组/编号分配。 |
| [ThreadStats](ThreadStats.md) | 每个线程的状态容器，包含周期/时间计数器、句柄缓存、CPU 集合绑定、活动连续次数、起始地址、优先级和理想处理器状态。 |

## 函数

| 名称 | 描述 |
|------|------|
| [format_100ns](format_100ns.md) | 将 100 纳秒的时间值格式化为人类可读的 `"秒.毫秒 s"` 字符串。 |
| [format_filetime](format_filetime.md) | 将 Windows FILETIME 值（自 1601-01-01 起的 100ns 单位）转换为本地日期时间字符串。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 阈值的配置常量 | [ConfigConstants](../config.rs/ConfigConstants.md) |
| 主线程应用逻辑 | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| 线程选择阶段 | [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) |
| 线程句柄管理 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| 进程快照数据 | [ProcessEntry](../process.rs/ProcessEntry.md) |
| 模块地址解析 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
