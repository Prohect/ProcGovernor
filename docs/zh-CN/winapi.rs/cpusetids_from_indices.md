# cpusetids_from_indices 函数 (winapi.rs)

将逻辑 CPU 索引（从 0 开始的处理器编号）数组转换为其对应的 Windows CPU 集合 ID。由于 Windows CPU 集合 API 操作的是系统分配的不透明 ID，而不是用户友好的逻辑处理器索引，因此需要进行此转换。

## 语法

```rust
pub fn cpusetids_from_indices(cpu_indices: &[u32]) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpu_indices` | `&[u32]` | 要转换的从零开始的逻辑处理器索引切片。每个值表示在任务管理器或 `PROCESSOR_NUMBER::Number` 中看到的逻辑处理器编号。 |

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 与输入索引对应的 CPU 集合 ID 的栈分配列表。如果某些索引未匹配缓存 CPU 集合拓扑中的任何条目，列表可能比输入短。如果 `cpu_indices` 为空，则返回空列表。

## 备注

- 该函数获取 [CPU_SET_INFORMATION](README.md) 静态缓存的锁，并遍历所有缓存的 [CpuSetData](CpuSetData.md) 条目。对于每个其 `logical_processor_index` 出现在输入切片中的条目，该条目的 `id` 被追加到结果列表中。
- CPU 集合拓扑在进程启动时通过 `GetSystemCpuSetInformation` 查询一次，并在服务的整个生存期内缓存。此函数不直接调用任何 Windows API——它只读取缓存。
- 输出顺序遵循缓存 CPU 集合数据的遍历顺序，该顺序与系统枚举顺序匹配（通常按逻辑处理器索引升序）。
- 结果使用 `List<[u32; CONSUMER_CPUS]>`，这是一个来自 `crate::collections` 的栈分配固定容量列表，对于最多具有 `CONSUMER_CPUS` 个逻辑处理器的系统避免了堆分配。

### CPU 集合 ID 与逻辑索引

| 概念 | 示例 | 使用方 |
|------|------|------|
| 逻辑处理器索引 | `0`, `1`, `2`, ... | 配置文件、亲和性掩码、`PROCESSOR_NUMBER::Number` |
| CPU 集合 ID | `0x100`, `0x101`, ... | `SetProcessDefaultCpuSets`、`SetThreadSelectedCpuSets` 和其他 CPU 集合 API |

索引和 ID 之间的映射是系统特定的，在启动时确定。同一个物理核心在重新启动后可能具有不同的 CPU 集合 ID。

### 与其他转换函数的关系

| 函数 | 方向 |
|------|------|
| **cpusetids_from_indices** | 逻辑索引 → CPU 集合 ID |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 亲和性掩码 → CPU 集合 ID |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU 集合 ID → 逻辑索引 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | CPU 集合 ID → 亲和性掩码 |
| [filter_indices_by_mask](filter_indices_by_mask.md) | 按亲和性掩码过滤的逻辑索引 |

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **被调用者** | [get_cpu_set_information](README.md)（读取缓存的 CPU 集合拓扑） |
| **Win32 API** | 无直接调用；依赖来自 [GetSystemCpuSetInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) 的缓存数据 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| CPU 集合拓扑缓存 | [CpuSetData](CpuSetData.md) |
| 反向转换 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| 基于掩码的转换 | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合应用 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
