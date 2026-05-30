# ThreadLevelConfig 结构体 (config.rs)

每进程线程级配置，控制主线程调度和理想处理器分配。与首次发现进程时应用一次的 [ProcessLevelConfig](ProcessLevelConfig.md) 不同，`ThreadLevelConfig` 规则在每次轮询迭代中评估，以跟踪线程活动并动态重新分配 CPU 资源。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct ThreadLevelConfig {
    pub name: String,
    pub prime_threads_cpus: List<[u32; CONSUMER_CPUS]>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `name` | `String` | 小写进程名称（例如 `"game.exe"`），用作线程级配置映射中的查找键。 |
| `prime_threads_cpus` | `List<[u32; CONSUMER_CPUS]>` | 所有可用于主线程固定的 CPU 索引的并集。这是所有 [PrimePrefix](PrimePrefix.md) 条目的合并集。当线程晋升为主线程时，其 CPU 集被限制为此列表中的索引（如果前缀匹配激活，则限制为特定前缀子集）。 |
| `prime_threads_prefixes` | `Vec<PrimePrefix>` | [PrimePrefix](PrimePrefix.md) 规则列表，控制哪些线程有资格进行主调度以及每个前缀组接收哪个 CPU 子集。空前缀字符串匹配所有线程。多个条目允许将不同模块的线程路由到不同的 CPU 集。 |
| `track_top_x_threads` | `i32` | 控制要跟踪的前 N 个线程（按 CPU 周期消耗）。正值启用前 N 个线程的主线程调度。负值启用仅跟踪模式（收集指标但不执行 CPU 固定）。零值完全禁用线程跟踪。从 prime 字段中的 `?N`（正值）或 `??N`（负值）前缀解析而来。 |
| `ideal_processor_rules` | `Vec<IdealProcessorRule>` | [IdealProcessorRule](IdealProcessorRule.md) 条目列表，根据线程的启动模块前缀为线程分配理想处理器提示。独立于主线程调度进行评估。 |

## 备注

### 与 ProcessLevelConfig 的关系

一条配置规则行可以同时为同一进程生成一个 [ProcessLevelConfig](ProcessLevelConfig.md) 和一个 `ThreadLevelConfig`。[parse_and_insert_rules](parse_and_insert_rules.md) 函数判断线程级字段是否活跃（非零主 CPU、非零跟踪计数或非空理想处理器规则），并仅在至少一项线程级功能正在使用时创建 `ThreadLevelConfig`。

### 主线程调度

主线程系统识别进程中 CPU 密集型最高的线程，并将其固定到高性能核心。选择过程使用由 [ConfigConstants](ConfigConstants.md) 控制的滞后机制，以避免频繁切换：

1. 每次迭代，线程按 CPU 周期增量排名。
2. 超过 `entry_threshold` 相对份额的线程开始累积活跃连续次数。
3. 一旦线程的连续次数达到 `min_active_streak`，它将被晋升为主线程状态并固定到 `prime_threads_cpus`。
4. 主线程仅在其份额降至 `keep_threshold` 以下时才会被降级。

`track_top_x_threads` 字段限制了参与此排名的线程数量。在拥有大量线程的系统上，这避免了测量每个线程的周期。

### 仅跟踪模式

当 `track_top_x_threads` 为负值时（从 `??N` 语法解析），调度器收集线程周期统计信息并记录日志，但不执行任何 CPU 集更改。这有助于在提交主配置之前对线程行为进行性能分析。

### 基于前缀的 CPU 路由

`prime_threads_prefixes` 列表允许根据线程的启动模块将不同线程路由到不同 CPU 子集。例如，游戏的渲染线程（从 `d3d11.dll` 启动）可以固定到 P 核心，而音频线程（从 `xaudio2.dll` 启动）分配到 E 核心。每个 [PrimePrefix](PrimePrefix.md) 还可以携带可选的 [ThreadPriority](../priority.rs/ThreadPriority.md) 提升。

### 理想处理器分配

`ideal_processor_rules` 字段独立于主调度运行。它在匹配特定模块前缀的线程上设置理想处理器提示，Windows 调度器将其作为偏好（而非硬约束）。这是比主线程固定更轻量级的替代方案。

### 配置字段格式

线程级设置从配置规则行的字段 4（prime）和字段 7（ideal processor）解析而来：

```
process.exe:priority:affinity:cpuset:prime_spec:io:memory:ideal_spec:grade
                                      ^field4                ^field7
```

prime 规格支持多种形式：
- `*alias`——将主线程固定到别名 CPU
- `?8x*alias`——跟踪前 8 个，固定到别名
- `??16`——跟踪前 16 个，不固定
- `*p@engine.dll;render.dll*e@audio.dll`——基于前缀的路由，各前缀有各自的 CPU 集

### 存储结构

`ThreadLevelConfig` 实例存储在 `ConfigResult.thread_level_configs` 中，这是一个 `HashMap<u32, HashMap<String, ThreadLevelConfig>>`，其中外层键是等级（轮询频率层级），内层键是小写进程名称。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **构造者** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **消费者** | [apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、主轮询循环 |
| **依赖项** | [PrimePrefix](PrimePrefix.md)、[IdealProcessorRule](IdealProcessorRule.md)、[List](../collections.rs/README.md) |
| **所需权限** | 无（仅为数据结构） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 进程级对应结构体 | [ProcessLevelConfig](ProcessLevelConfig.md) |
| 主前缀规则 | [PrimePrefix](PrimePrefix.md) |
| 理想处理器规则 | [IdealProcessorRule](IdealProcessorRule.md) |
| 滞后常量 | [ConfigConstants](ConfigConstants.md) |
| 主线程调度器状态 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 配置文件读取器 | [read_config](read_config.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
