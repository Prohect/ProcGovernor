# parse_and_insert_rules 函数 (config.rs)

主要规则字段解析和插入函数。接收一个进程名称数组（单个名称或组块的成员）和一个冒号拆分的规则字段数组，验证每个字段，构造 [ProcessLevelConfig](ProcessLevelConfig.md) 和/或 [ThreadLevelConfig](ThreadLevelConfig.md) 实例，并将其插入 [ConfigResult](ConfigResult.md) 中相应等级索引的映射中。

## 语法

```rust
fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## 参数

`members: &[String]`

应将解析的规则应用于其上的小写进程名称切片。对于单行规则，包含一个元素（例如 `["game.exe"]`）。对于组规则，包含从 `{ }` 块中提取的所有成员名称。

`rule_parts: &[&str]`

从配置行的规则部分拆分的字段字符串切片。期望的字段位置：

| 索引 | 字段 | 示例 | 描述 |
|------|------|------|------|
| 0 | priority | `"normal"` | 进程优先级类别 |
| 1 | affinity | `"*pcore"` 或 `"0-7"` | CPU 亲和性规格说明 |
| 2 | cpuset | `"@*ecore"` 或 `"0"` | CPU 集规格说明；`@` 前缀启用理想处理器重置 |
| 3 | prime | `"?8x*p@engine.dll"` | 主线程规格（CPU、前缀、跟踪计数） |
| 4 | io_priority | `"low"` | IO 优先级级别 |
| 5 | memory_priority | `"none"` | 内存优先级级别 |
| 6 | ideal / grade | `"*p@render.dll"` 或 `"2"` | 理想处理器规格，如果为数字则为等级 |
| 7 | grade | `"1"` | 应用频率层级（仅当字段 6 为理想处理器规格时） |

至少需要 2 个字段（优先级和亲和性）；所有后续字段可选，默认为"不更改"值。

`line_number: usize`

此规则在配置文件中来源的基于 1 的行号。用于所有错误和警告消息，供用户诊断。

`cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>`

已定义 CPU 别名的映射，由先前的 [parse_alias](parse_alias.md) 调用填充。传递给 [resolve_cpu_spec](resolve_cpu_spec.md) 以进行别名感知的 CPU 字段解析。

`result: &mut ConfigResult`

**\[入参, 出参\]** 累积的解析结果。此函数读取和写入 `errors`、`warnings`、`process_level_configs`、`thread_level_configs`、`process_rules_count`、`redundant_rules_count` 和 `thread_level_configs_count`。

## 返回值

此函数不返回值。所有输出通过对 `result` 的变更传达。

## 备注

### 字段解析流水线

每个字段按顺序独立解析：

1. **优先级（字段 0）：** 通过 `ProcessPriority::from_str` 解析。未知值产生警告并默认为 `ProcessPriority::None`。

2. **亲和性（字段 1）：** 通过 [resolve_cpu_spec](resolve_cpu_spec.md) 解析，处理 `*别名` 引用和字面量 CPU 规格。

3. **CPU 集（字段 2）：** 如果字段以 `@` 开头，剥离 `@` 并将 `cpu_set_reset_ideal` 设为 `true`；剩余部分作为 CPU 规格解析。此标志导致在应用 CPU 集后重新分布线程理想处理器。

4. **主线程规格（字段 3）：** 最复杂的字段。支持多种子格式：
   - `"0"`——无主线程调度。
   - `"*alias"`——将主线程固定到别名的 CPU，匹配所有线程。
   - `"?Nx*alias"`——跟踪前 N 个线程（按 CPU 周期）并将其固定到别名 CPU。带数字的 `?` 前缀设置正向 `track_top_x_threads`。
   - `"??N"`——跟踪前 N 个线程，但无主线程固定。`??` 前缀设置负向 `track_top_x_threads`（仅跟踪模式）。
   - `"*alias@prefix1;prefix2!priority"`——按前缀的 CPU 路由，带可选的线程优先级提升。每个 `*alias@` 段生成 [PrimePrefix](PrimePrefix.md) 条目。前缀内的 `!` 分隔符设置 [ThreadPriority](../priority.rs/ThreadPriority.md) 覆盖。

5. **IO 优先级（字段 4）：** 通过 `IOPriority::from_str` 解析。未知值产生警告。

6. **内存优先级（字段 5）：** 通过 `MemoryPriority::from_str` 解析。未知值产生警告。

7. **理想处理器 / 等级（字段 6）：** 歧义字段——如果以 `*` 开头或为 `"0"`，则通过 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 解析为理想处理器规格。如果解析为纯整数，则视为等级。否则，尝试作为理想处理器规格，等级默认为 1。

8. **等级（字段 7）：** 如果字段 6 是理想处理器规格且字段 7 存在，则将其解析为等级。等级必须 ≥ 1；值为 0 产生警告并默认为 1。

### 配置插入逻辑

对于每个成员名称，函数执行两次独立的有效性检查：

- **进程级有效：** 至少 `priority`、`affinity_cpus`、`cpu_set_cpus`、`io_priority` 或 `memory_priority` 之一为非默认值。如果有效，创建 [ProcessLevelConfig](ProcessLevelConfig.md) 并插入 `result.process_level_configs` 中的相应等级下。

- **线程级有效：** 至少 `prime_threads_cpus`（非空）、`track_top_x_threads`（非零）或 `ideal_processor_rules`（非空）之一为活跃。如果有效，创建 [ThreadLevelConfig](ThreadLevelConfig.md) 并插入 `result.thread_level_configs`。

如果两项检查均未通过，则对该进程发出警告，表明不存在有效规则。

### 冗余检测

插入前，函数检查所有现有等级映射中是否存在同名进程条目。如果找到，`redundant_rules_count` 递增并发出警告。新条目覆盖旧条目——最后定义生效。

### 主前缀构造

当主线程规格包含 `@` 段时，解析器构建一个包含每段 CPU 列表和可选线程优先级覆盖的 `Vec<PrimePrefix>`。`prime_threads_cpus` 字段设为所有段 CPU 集的并集。当不存在 `@` 段时，创建单个默认 [PrimePrefix](PrimePrefix.md)，其空前缀（匹配所有线程）、`None` cpus（从 `prime_threads_cpus` 继承）和 `ThreadPriority::None`。

### 无前缀规格的默认 PrimePrefix

即使主线程规格是简单 `*alias` 而没有任何 `@` 前缀筛选，函数仍会创建一个包含一个默认条目的 `Vec<PrimePrefix>`。这确保下游的 [apply_prime_threads](../apply.rs/apply_prime_threads.md) 函数始终至少有一个前缀条目可遍历。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有（`fn`） |
| **调用者** | [read_config](read_config.md)（用于单行规则和组规则） |
| **被调用者** | [resolve_cpu_spec](resolve_cpu_spec.md)、[parse_ideal_processor_spec](parse_ideal_processor_spec.md)、`ProcessPriority::from_str`、`IOPriority::from_str`、`MemoryPriority::from_str`、`ThreadPriority::from_str` |
| **写入** | [ConfigResult](ConfigResult.md)（`.process_level_configs`、`.thread_level_configs`、`.errors`、`.warnings`、计数器） |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 进程级配置结构体 | [ProcessLevelConfig](ProcessLevelConfig.md) |
| 线程级配置结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 主前缀结构体 | [PrimePrefix](PrimePrefix.md) |
| 理想处理器规则结构体 | [IdealProcessorRule](IdealProcessorRule.md) |
| CPU 别名解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 理想处理器规格解析器 | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 配置结果容器 | [ConfigResult](ConfigResult.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md)、[IOPriority](../priority.rs/IOPriority.md)、[MemoryPriority](../priority.rs/MemoryPriority.md)、[ThreadPriority](../priority.rs/ThreadPriority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
