# ConfigResult 结构体 (config.rs)

配置解析器的聚合输出。包含所有已解析的进程级和线程级规则（按等级即应用频率组织），以及解析统计信息、错误和警告。这是 [read_config](read_config.md) 的主要返回类型，也是主服务循环和 [hotreload_config](hotreload_config.md) 消费的结构体。

## 语法

```rust
#[derive(Debug, Default)]
pub struct ConfigResult {
    pub process_level_configs: HashMap<u32, HashMap<String, ProcessLevelConfig>>,
    pub thread_level_configs: HashMap<u32, HashMap<String, ThreadLevelConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub thread_level_configs_count: usize,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `process_level_configs` | `HashMap<u32, HashMap<String, ProcessLevelConfig>>` | 按等级（外层）和小写进程名称（内层）组织的进程级规则。等级 `1` 是默认值，表示规则在每次轮询迭代中应用；更高等级降低应用频率。 |
| `thread_level_configs` | `HashMap<u32, HashMap<String, ThreadLevelConfig>>` | 按等级（外层）和小写进程名称（内层）组织的线程级规则。这些规则在每次轮询迭代中评估，用于线程级调度（主线程、理想处理器）。 |
| `constants` | [ConfigConstants](ConfigConstants.md) | 主线程滞后的调优常量。从配置文件中的 `@CONSTANT = value` 行填充；未指定时使用 `Default` 值。 |
| `constants_count` | `usize` | 成功解析的 `@CONSTANT` 指令数量。 |
| `aliases_count` | `usize` | 成功解析的 `*alias = cpu_spec` 别名定义数量。 |
| `groups_count` | `usize` | 成功解析的 `{ }` 组块数量。 |
| `group_members_count` | `usize` | 所有已解析组块中包含的进程名称总数。 |
| `process_rules_count` | `usize` | 尝试为其创建规则的进程名称总数（包括单条和组成员）。 |
| `redundant_rules_count` | `usize` | 覆盖了先前为同一进程名称定义的规则的规则数量。每条冗余规则同时生成一条警告。 |
| `errors` | `Vec<String>` | 致命解析错误。任何非空的错误列表会导致 [is_valid](#is_valid) 返回 `false`，并阻止应用该配置。消息包含行号。 |
| `warnings` | `Vec<String>` | 非致命解析警告（例如未知的优先级名称被视为 `none`、冗余规则、空组）。存在警告时配置仍然可用。 |
| `thread_level_configs_count` | `usize` | 已插入的线程级配置条目的运行计数。每生成一个有效的线程级规则的进程名称递增一次。 |

## 方法

### is_valid

```rust
pub fn is_valid(&self) -> bool
```

如果 `errors` 向量为空则返回 `true`，表示配置解析成功且可以安全应用，无致命错误。

**返回值**

`bool`——未记录解析错误时为 `true`；否则为 `false`。

### total_rules

```rust
pub fn total_rules(&self) -> usize
```

返回所有等级中活跃规则的总数，结合进程级和线程级配置。这计算的是内层 `HashMap` 中的条目数，而非原始 `process_rules_count` 计数器。

**返回值**

`usize`——各等级中所有进程级配置条目和所有线程级配置条目的总和。

### print_report

```rust
pub fn print_report(&self)
```

记录解析结果的可读摘要。行为取决于有效性：

- **有效配置：** 记录组统计信息（如果存在任何组）、总进程规则数，以及以 `⚠` 为前缀的任何警告。
- **无效配置：** 记录所有以 `✗` 为前缀的错误、所有以 `⚠` 为前缀的警告，以及错误数量的最终总结。
- **存在冗余规则：** 即使配置有效也会打印警告，提醒用户注意重复定义。

## 备注

### 两级 HashMap 结构

外层 `HashMap<u32, ...>` 键是**等级**，一个 ≥ 1 的无符号整数，控制应用规则的频率。等级 1 的规则每个轮询周期运行一次；等级 *N* 的规则每 *N* 个周期运行一次。这允许用户为不需要持续监控的进程配置较低频率的执行。内层 `HashMap<String, ...>` 将小写进程名称映射到其各自配置。

### 进程级与线程级分离

一条配置行可以同时产生一个 [ProcessLevelConfig](ProcessLevelConfig.md) 和一个 [ThreadLevelConfig](ThreadLevelConfig.md)。[parse_and_insert_rules](parse_and_insert_rules.md) 中的解析器独立判定有效性：仅当优先级、亲和性、CPU 集、IO 优先级或内存优先级中至少有一项为非默认值时，才创建进程级条目；仅当指定了主 CPU、跟踪或理想处理器规则时，才创建线程级条目。如果两者均无效，则发出警告。

### 冗余检测

当为已在任何等级中存在的进程名称插入规则时，旧条目被覆盖，`redundant_rules_count` 递增。这是一个警告，而非错误——最后定义生效。

### Default 派生

该结构体派生 `Default`，将所有 `HashMap` 和 `Vec` 初始化为空，所有计数器设为零，`constants` 设为 `ConfigConstants::default()`。这被 [read_config](read_config.md) 用于在解析开始前创建初始累加器。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用者** | [read_config](read_config.md)、[hotreload_config](hotreload_config.md)、主轮询循环、[parse_and_insert_rules](parse_and_insert_rules.md)、[parse_constant](parse_constant.md)、[parse_alias](parse_alias.md) |
| **依赖项** | [ProcessLevelConfig](ProcessLevelConfig.md)、[ThreadLevelConfig](ThreadLevelConfig.md)、[ConfigConstants](ConfigConstants.md)、[HashMap](../collections.rs/README.md) |
| **所需权限** | 无（仅为数据结构） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 进程级规则结构体 | [ProcessLevelConfig](ProcessLevelConfig.md) |
| 线程级规则结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 调优常量 | [ConfigConstants](ConfigConstants.md) |
| 配置文件解析器 | [read_config](read_config.md) |
| 规则插入逻辑 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 带验证的热重载 | [hotreload_config](hotreload_config.md) |
| 应用引擎 | [apply.rs](../apply.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
