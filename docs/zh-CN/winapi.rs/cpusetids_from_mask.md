# cpusetids_from_mask 函数 (winapi.rs)

将 CPU 亲和性位掩码转换为 Windows CPU 集合 ID 列表。掩码中的每个设置位对应一个逻辑处理器索引，该索引通过缓存的系统 CPU 集合拓扑映射到其不透明的 CPU 集合 ID。

## 语法

```rust
pub fn cpusetids_from_mask(mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `mask` | `usize` | 一个位掩码，其中位 *N* 表示逻辑处理器 *N*。例如，`0x15`（二进制 `10101`）表示处理器 0、2 和 4。只有低 64 位有意义；索引 ≥ 64 的处理器将被静默跳过。 |

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 与 `mask` 中设置位所指示的逻辑处理器对应的 CPU 集合 ID 的栈分配列表。如果 `mask` 为 `0`，则返回空列表。

## 备注

- 该函数获取 [CPU_SET_INFORMATION](README.md) 缓存的锁，并遍历所有 [CpuSetData](CpuSetData.md) 条目。对于每个其 `logical_processor_index` 对应 `mask` 中设置位的条目，该条目的 `id` 被追加到结果中。
- 位测试执行为 `(1usize << logical_processor_index) & mask != 0`，这将函数限制在单个处理器组内的处理器 0–63。`logical_processor_index >= 64` 的条目通过显式边界检查被排除。
- 此函数是 [cpusetids_from_indices](cpusetids_from_indices.md) 的基于掩码的对应函数，后者接受显式的处理器索引列表而不是位掩码。
- 目前标记为 `#[allow(dead_code)]`——可供使用但在当前代码库中未被调用。

### 与亲和性掩码的关系

Windows 亲和性掩码（由 `GetProcessAffinityMask` 返回）使用此函数接受的相同的每位一个处理器的编码。这使得 `cpusetids_from_mask` 成为传统亲和性 API 与较新的 CPU 集合 API 之间的自然桥梁。

### 性能

该函数对缓存的 CPU 集合数据执行单次遍历（每个逻辑处理器一个条目），使其时间复杂度为 O(n)，其中 n 是系统上逻辑处理器的数量。`Mutex` 锁在遍历期间被持有。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | 当前未使用（`#[allow(dead_code)]`） |
| **被调用者** | [get_cpu_set_information](README.md) |
| **Win32 API** | 无直接调用；消费来自 [GetSystemCpuSetInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/systemcpusetinformation/nf-systemcpusetinformation-getsystemcpusetinformation) 的缓存数据 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 基于索引的转换 | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 反向：CPU 集合 ID → 索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| 反向：CPU 集合 ID → 掩码 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| 按掩码过滤索引 | [filter_indices_by_mask](filter_indices_by_mask.md) |
| CPU 集合拓扑缓存 | [CpuSetData](CpuSetData.md) |
| 模块概述 | [winapi.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
