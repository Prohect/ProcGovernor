# IdealProcessorRule 结构体 (config.rs)

将一组 CPU 索引映射到一个线程启动模块前缀列表，形成一条理想处理器分配规则。当服务遍历一个进程的线程时，任何启动模块与 `prefixes` 中某一项匹配的线程都会从 `cpus` 中获得一个理想处理器提示。如果 `prefixes` 为空，则该规则无条件应用于所有线程。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: List<[u32; CONSUMER_CPUS]>,
    pub prefixes: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `cpus` | `List<[u32; CONSUMER_CPUS]>` | 分配理想处理器提示时使用的逻辑处理器索引排序列表。在解析时从 CPU 别名解析而来。必须非空才能使规则生效。 |
| `prefixes` | `Vec<String>` | 用于筛选线程的小写模块名前缀。如果线程的启动模块以列表中的任意一项开头，则该线程匹配。空向量表示该规则匹配进程中的每个线程。 |

## 备注

`IdealProcessorRule` 由 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 从配置规则行的理想处理器字段（字段 7）生成。规格格式使用 `*alias@prefix1;prefix2` 段，其中每个段成为一个 `IdealProcessorRule` 实例。可以链接多个段，为同一进程中不同模块的线程分配不同的 CPU 集。

### 理想处理器 vs. 亲和性

理想处理器提示是一种*软偏好*——Windows 调度器会偏好提示的核心，但在负载下可能会将线程调度到其他位置。这与硬亲和性形成对比，后者将线程限制在严格的 CPU 集合内。因此，理想处理器规则适用于引导线程放置而不会引入资源饥饿的风险。

### 线程匹配

线程到规则的匹配由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 在每次轮询迭代中执行。对于每个线程，服务通过来自 Windows 事件跟踪（ETW）子系统的缓存数据查询线程的起始地址模块。第一个其 `prefixes` 列表包含匹配前缀（或列表为空）的规则决定理想 CPU。

### CPU 分布

当多个线程匹配同一规则时，服务会在 `cpus` 列表中轮转分配理想处理器，以均匀分散负载。

### 配置语法示例

```
*pcore = 0-7
*ecore = 8-19
game.exe:normal:0:0:0:none:none:*pcore@engine.dll;render.dll*ecore@audio.dll
```

这将产生两个 `IdealProcessorRule` 条目：

1. `cpus: [0..7], prefixes: ["engine.dll", "render.dll"]`
2. `cpus: [8..19], prefixes: ["audio.dll"]`

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **生产者** | [parse_ideal_processor_spec](parse_ideal_processor_spec.md)、[parse_and_insert_rules](parse_and_insert_rules.md) |
| **存储于** | [ThreadLevelConfig](ThreadLevelConfig.md)`.ideal_processor_rules` |
| **消费者** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **集合类型** | [List](../collections.rs/README.md) (`SmallVec<[u32; CONSUMER_CPUS]>`) |
| **所需权限** | 无（仅为数据结构） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 父级配置结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 规格解析器 | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| 运行时应用 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| CPU 别名解析 | [parse_alias](parse_alias.md) |
| 主线程前缀（相关概念） | [PrimePrefix](PrimePrefix.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
