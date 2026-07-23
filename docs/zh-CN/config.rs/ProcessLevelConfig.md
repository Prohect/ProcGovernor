# ProcessLevelConfig 结构体 (config.rs)

表示单个 Windows 进程的完整进程级调优参数集。每个实例捕获所需的优先级类别、CPU 亲和性掩码、Job Object 亲和性、CPU 集分配、IO 优先级和内存优先级。当服务首次发现匹配的进程时，这些设置会被应用一次，并且在后续轮询迭代中不会重新应用，除非配置被热重载。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct ProcessLevelConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub job_object_affinity_spec: String,
    pub job_object_affinity_cpus: List<[u32; CONSUMER_CPUS]>,
    pub affinity_cpus: List<[u32; CONSUMER_CPUS]>,
    pub cpu_set_cpus: List<[u32; CONSUMER_CPUS]>,
    pub cpu_set_reset_ideal: bool,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|-------------|
| `name` | `String` | 小写进程名称（例如 `"game.exe"`），用作配置哈希映射中的查找键。匹配时对运行中的进程名称不区分大小写。 |
| `priority` | [ProcessPriority](../priority.rs/ProcessPriority.md) | 目标 Windows 进程优先级类别。当设为 `ProcessPriority::None` 时，服务不修改进程的优先级。有效值包括 `Idle`、`BelowNormal`、`Normal`、`AboveNormal`、`High` 和 `RealTime`。 |
| `job_object_affinity_spec` | `String` | 原始 CPU 规格字符串（例如 `*ecore`、`0-7`），用于命名和标识 Job Object。这是配置文件中 `job_affinity` 字段的未解析文本，用作 Job Object 缓存键的一部分。 |
| `job_object_affinity_cpus` | `List<[u32; CONSUMER_CPUS]>` | 内核强制的 Job Object 亲和性的逻辑处理器索引排序列表。空列表表示不创建 Job Object。与 `affinity_cpus` 不同，此限制在内核级别强制执行，无法绕过。 |
| `affinity_cpus` | `List<[u32; CONSUMER_CPUS]>` | 定义硬 CPU 亲和性掩码的逻辑处理器索引排序列表。空列表表示不应用亲和性更改。设置后，服务会调用 `SetProcessAffinityMask` 并立即将线程理想处理器重新分布到新的 CPU 集上。 |
| `cpu_set_cpus` | `List<[u32; CONSUMER_CPUS]>` | 进程默认 CPU 集（软亲和性）的逻辑处理器索引排序列表。空列表表示不应用 CPU 集更改。使用 Windows CPU Sets API（`SetProcessDefaultCpuSets`），提供比硬亲和性更软的调度提示。 |
| `cpu_set_reset_ideal` | `bool` | 当为 `true` 时，在应用 CPU 集后重置所有线程的理想处理器分配。在配置文件中通过在 CPU 集规格前加 `@` 前缀触发（例如 `@*ecore`）。在将 CPU 集与理想处理器规则结合使用时很有用，可确保线程在 CPU 集更改后重新分布。 |
| `io_priority` | [IOPriority](../priority.rs/IOPriority.md) | 进程的目标 IO 优先级。通过 `NtSetInformationProcess` 配合 `ProcessIoPriority` 设置。当设为 `IOPriority::None` 时，不应用 IO 优先级更改。有效值包括 `VeryLow`、`Low`、`Normal` 和 `High`。 |
| `memory_priority` | [MemoryPriority](../priority.rs/MemoryPriority.md) | 进程的目标内存优先级。通过 `SetProcessInformation` 配合 `ProcessMemoryPriority` 设置。当设为 `MemoryPriority::None` 时，不应用内存优先级更改。有效值包括 `VeryLow`、`Low`、`Medium`、`BelowNormal` 和 `Normal`。 |

## 备注

### 配置文件格式

`ProcessLevelConfig` 从具有以下位置字段的配置规则行构造而来：

```
process.exe:priority:job_affinity:affinity:cpuset:prime:io_priority:memory_priority:ideal:grade
```

只有进程名称后的前三个字段（`priority`、`job_affinity` 和 `affinity`）是必需的。省略的字段默认为其"不更改"值（`None` / 空列表 / `false`）。

### 解析期间的验证

[parse_and_insert_rules](parse_and_insert_rules.md) 函数仅在至少有一个进程级字段具有非默认值时才创建 `ProcessLevelConfig`。如果所有进程级字段均为默认值（例如，规则仅指定了主 CPU 等线程级设置），则不会插入 `ProcessLevelConfig`，仅创建 [ThreadLevelConfig](ThreadLevelConfig.md)。

### 亲和性类型对比

`job_object_affinity_cpus`、`affinity_cpus` 和 `cpu_set_cpus` 都控制进程可以使用哪些处理器，但在执行方式上有所不同：

| 特性 | Job Object (`job_object_affinity_cpus`) | 亲和性 (`affinity_cpus`) | CPU 集 (`cpu_set_cpus`) |
|------|---|---|---|
| **API** | `CreateJobObjectW` + `SetInformationJobObject` + `AssignProcessToJobObject` | `SetProcessAffinityMask` | `SetProcessDefaultCpuSets` |
| **执行方式** | 内核级——进程及其子进程无法绕过 | 硬性——线程无法在排除的 CPU 上运行 | 软性——调度器偏好列出的 CPU，但可能溢出 |
| **范围** | 限制进程及所有子进程 | 限制整个进程 | 设置默认提示；单个线程可以覆盖 |
| **最大 CPU 数** | 64（单个处理器组） | 64（单个处理器组） | 支持跨处理器组的超过 64 个 CPU |

### 基于等级的组织

`ProcessLevelConfig` 实例存储在 [ConfigResult](ConfigResult.md) 中的两级 `HashMap<u32, HashMap<String, ProcessLevelConfig>>` 内。外层键是*等级*（应用频率），内层键是进程名称。等级 1 的规则每次轮询迭代都会检查；更高等级以更低频率检查，降低低优先级进程的开销。

### 冗余检测

如果多条规则为同一进程名称定义了 `ProcessLevelConfig`，解析器会发出警告，且后定义覆盖先定义。[ConfigResult](ConfigResult.md) 中的 `redundant_rules_count` 计数器跟踪发生了多少次此类覆盖。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **创建者** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **消费者** | [apply_job_object_affinity](../apply.rs/apply_job_object_affinity.md)、[apply_priority](../apply.rs/apply_priority.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| **依赖项** | [ProcessPriority](../priority.rs/ProcessPriority.md)、[IOPriority](../priority.rs/IOPriority.md)、[MemoryPriority](../priority.rs/MemoryPriority.md)、[List](../collections.rs/README.md) |
| **所需权限** | `PROCESS_SET_INFORMATION`（写入）、`PROCESS_QUERY_LIMITED_INFORMATION`（读取） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 线程级对应结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 聚合解析结果 | [ConfigResult](ConfigResult.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU 别名解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md)、[IOPriority](../priority.rs/IOPriority.md)、[MemoryPriority](../priority.rs/MemoryPriority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*