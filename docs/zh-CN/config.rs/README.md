# config 模块 (ProcGovernor)

`config` 模块负责处理 ProcGovernor 配置文件的解析、验证和管理。它定义了进程级和线程级调度策略的规则结构体，并实现了一个多段类 INI 格式的配置解析器，支持 CPU 别名（`*name = spec`）、命名组（`name { members }`）、调优常量（`@NAME = value`）以及复杂的 CPU 规格说明（范围、十六进制掩码、分号分隔的索引）。该模块还提供配置文件和黑名单文件的热重载能力、从 Process Lasso 格式的转换器，以及一个将共享相同规则的进程合并为组的自动分组工具。

## 结构体

| 名称 | 描述 |
|------|------|
| [PrimePrefix](PrimePrefix.md) | 将模块名前缀与 CPU 集和可选的线程优先级提升关联，用于主线程匹配。 |
| [IdealProcessorRule](IdealProcessorRule.md) | 将线程启动模块前缀映射到理想 CPU 分配。 |
| [ProcessLevelConfig](ProcessLevelConfig.md) | 每进程规则，应用一次：优先级、亲和性、CPU 集、IO 优先级、内存优先级。 |
| [ThreadLevelConfig](ThreadLevelConfig.md) | 每进程线程级规则，每次轮询迭代中应用：主线程、理想处理器、跟踪。 |
| [ConfigConstants](ConfigConstants.md) | 主线程选择滞后算法的调优常量（连续次数、阈值）。 |
| [ConfigResult](ConfigResult.md) | 配置解析的聚合结果：按等级组织的规则映射、计数器、错误和警告。 |

## 函数

| 名称 | 描述 |
|------|------|
| [parse_cpu_spec](parse_cpu_spec.md) | 将 CPU 规格说明字符串（范围、十六进制掩码、分号分隔）解析为排序的 CPU 索引列表。 |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | 将 64 位位掩码转换为排序的 CPU 索引列表。 |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | 将 CPU 索引切片转换为 `usize` 位掩码。 |
| [format_cpu_indices](format_cpu_indices.md) | 将 CPU 索引切片格式化为紧凑的范围字符串，如 `"0-7,12-19"`。 |
| [parse_mask](parse_mask.md) | 便捷包装函数：将 CPU 规格说明字符串直接解析为位掩码。 |
| [resolve_cpu_spec](resolve_cpu_spec.md) | 解析可能是 `*别名` 引用或字面量规格的 CPU 规格说明。 |
| [collect_members](collect_members.md) | 将冒号分隔的进程名称拆分为成员列表。 |
| [parse_constant](parse_constant.md) | 解析 `@NAME = value` 常量定义（MIN_ACTIVE_STREAK、KEEP_THRESHOLD、ENTRY_THRESHOLD）。 |
| [parse_alias](parse_alias.md) | 解析 `*name = cpu_spec` 别名定义。 |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | 解析理想处理器规格说明，如 `*alias@prefix1;prefix2`。 |
| [collect_group_block](collect_group_block.md) | 收集 `{` 和 `}` 之间的多行组块成员。 |
| [parse_and_insert_rules](parse_and_insert_rules.md) | 主要规则解析器：拆分字段、验证、创建配置、按等级插入到 [ConfigResult](ConfigResult.md) 中。 |
| [read_config](read_config.md) | 主配置文件读取器。处理常量、别名、组和单行规则。 |
| [read_bleack_list](read_bleack_list.md) | 读取黑名单文件（每行一个进程名称，`#` 表示注释）。 |
| [read_utf16le_file](read_utf16le_file.md) | 读取 UTF-16LE 编码文件，返回 Rust `String`。 |
| [convert](convert.md) | 将 Process Lasso 配置格式转换为 ProcGovernor 格式。 |
| [sort_and_group_config](sort_and_group_config.md) | 将具有相同设置的规则自动分组为命名组块。 |
| [hotreload_blacklist](hotreload_blacklist.md) | 如果磁盘上的黑名单文件已被修改，则热重载它。 |
| [hotreload_config](hotreload_config.md) | 如果配置文件已被修改，则热重载它，成功时重置调度器状态。 |

## 配置文件格式

配置文件使用面向行的格式，包含以下几种段落类型：

```
# 以 # 开头的注释

# 常量（调优参数）
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1

# CPU 别名
*pcore = 0-7
*ecore = 8-19

# 单行规则
process.exe:priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade

# 命名组规则
group_name { proc1.exe: proc2.exe: proc3.exe }:normal:*ecore:0:0:low:none:0:1

# 多行组
group_name {
    proc1.exe: proc2.exe
    proc3.exe
}:normal:*ecore:0:0:low:none:0:1
```

## 另请参阅

| 主题 | 链接 |
|------|------|
| 执行引擎 | [apply.rs](../apply.rs/README.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md)、[IOPriority](../priority.rs/IOPriority.md)、[MemoryPriority](../priority.rs/MemoryPriority.md)、[ThreadPriority](../priority.rs/ThreadPriority.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |
| 集合类型 | [List / HashMap](../collections.rs/README.md) |
| 主服务循环 | [main.rs](../main.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
