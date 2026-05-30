# PrimePrefix 结构体 (config.rs)

将模块名前缀与 CPU 集和可选的线程优先级提升关联，用于主线程匹配。当主线程调度器将线程识别为"主线程"（高活跃度）时，它会根据存储的 `PrimePrefix` 条目检查线程的启动模块，以确定该线程应被固定到哪些 CPU 上，以及是否需要提升优先级。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<List<[u32; CONSUMER_CPUS]>>,
    pub thread_priority: ThreadPriority,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `prefix` | `String` | 要与线程启动模块名匹配的模块名前缀（例如 `"engine.dll"`）。空字符串匹配所有线程，不论其启动模块为何。比较时不区分大小写。 |
| `cpus` | `Option<List<[u32; CONSUMER_CPUS]>>` | 匹配的主线程应固定到的 CPU 索引列表。当值为 `Some` 时，对于匹配此前缀的线程，将覆盖父级 [ThreadLevelConfig](ThreadLevelConfig.md) 的 `prime_threads_cpus`。当值为 `None` 时，使用父级的 `prime_threads_cpus` 作为回退。 |
| `thread_priority` | [ThreadPriority](../priority.rs/ThreadPriority.md) | 当线程晋升为主线程状态时应用的可选优先级提升。`ThreadPriority::None` 表示不更改优先级（自动提升行为）。在配置中通过 `!priority` 后缀语法指定（例如 `engine.dll!above normal`）。 |

## 备注

### 配置语法

`PrimePrefix` 条目从规则行的 prime 字段（字段 4）解析而来。该格式支持按前缀的 CPU 覆盖和优先级提升：

```
process.exe:normal:0:0:*pcore@engine.dll;helper.dll!above normal:none:none:0:1
```

在此示例中：
- `*pcore` 引用一个 CPU 别名，定义了要分配的 CPU。
- `@engine.dll;helper.dll!above normal` 定义了两个前缀：`engine.dll`（无优先级提升）和 `helper.dll`（提升到 above normal）。

### 匹配行为

- 当 `prefix` 为空（`""`）时，该条目充当通配规则，匹配任何线程而无论其启动模块为何。
- 一个进程规则可以存在多个 `PrimePrefix` 条目。调度器按顺序评估它们，线程匹配其字符串是线程启动模块名前缀的第一个前缀。
- `cpus` 字段允许在同一进程规则中将不同模块的线程定向到不同的 CPU 核心。例如，渲染线程可以分配到 P 核心，而音频线程分配到 E 核心。

### 默认构造

当 prime 字段中未使用 `@prefix` 语法时，会创建一个单个 `PrimePrefix`，其 `prefix` 为空、`cpus` 设为 `None`、`thread_priority` 设为 `ThreadPriority::None`。这意味着该进程的所有主线程使用父级配置的 CPU 集，且无优先级提升。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **消费者** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) |
| **依赖项** | [ThreadPriority](../priority.rs/ThreadPriority.md)、[List](../collections.rs/README.md) |
| **所需权限** | 无（仅为数据结构） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 父级配置结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 主线程晋升 | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| 线程优先级枚举 | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| 规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 模块概览 | [config.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
