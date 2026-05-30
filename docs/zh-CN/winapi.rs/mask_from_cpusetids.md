# mask_from_cpusetids 函数 (winapi.rs)

将 CPU 集合 ID 切片转换回处理器亲和性位掩码。这是 [cpusetids_from_mask](cpusetids_from_mask.md) 的反向操作，将不透明的 Windows CPU 集合标识符映射到 `usize` 掩码中的位置位。

## 语法

```rust
pub fn mask_from_cpusetids(cpuids: &[u32]) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpuids` | `&[u32]` | 要转换的 CPU 集合 ID 切片。每个 ID 必须对应系统 CPU 集合拓扑中从 [get_cpu_set_information](../winapi.rs/README.md) 获取的条目。 |

## 返回值

`usize` — 一个位掩码，如果提供的任何 CPU 集合 ID 映射到逻辑处理器索引 *N*，则位 *N* 被设置。如果输入切片为空，则返回 `0`。

### 示例

| 输入 CPU 集合 ID | 逻辑处理器索引 | 输出掩码 |
|-----------------|--------------|--------|
| `[]` | *(无)* | `0x0` |
| `[256]` | `[0]` | `0x1` |
| `[256, 258, 260]` | `[0, 2, 4]` | `0x15` |

*(实际的 CPU 集合 ID 值是系统特定的；以上仅为说明性示例。)*

## 备注

- 该函数获取 `CPU_SET_INFORMATION` mutex 锁并遍历所有缓存的 [CpuSetData](CpuSetData.md) 条目。对于每个其 `id` 在输入切片中找到的条目，相应的位（`1 << logical_processor_index`）被设置在结果掩码中。
- 逻辑处理器索引 ≥ 64 将被静默跳过，因为在 64 位系统上它们无法在单个 `usize` 位掩码中表示。这与每个处理器组 64 个处理器的 Windows 亲和性掩码限制一致。
- 该函数执行 O(C × N) 查找，其中 C 是 CPU 集合条目的数量，N 是输入切片的长度，对每个缓存条目使用 `slice::contains`。这对于消费级系统典型的较小输入大小（≤ 64 个 CPU）是高效的。
- 输入中未匹配缓存拓扑中任何条目的 CPU 集合 ID 将被静默忽略——对于无法识别的 ID 不设置任何位。
- 此函数目前标记为 `#[allow(dead_code)]`，但可供任何需要将 CPU 集合 ID 转换回亲和性掩码表示的组件使用。

### 与其他转换函数的关系

| 函数 | 方向 |
|------|------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | CPU 索引 → CPU 集合 ID |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 亲和性掩码 → CPU 集合 ID |
| [indices_from_cpusetids](indices_from_cpusetids.md) | CPU 集合 ID → CPU 索引 |
| **mask_from_cpusetids** | **CPU 集合 ID → 亲和性掩码** |
| [filter_indices_by_mask](filter_indices_by_mask.md) | CPU 索引 × 掩码 → 过滤后的索引 |

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | 可供一般使用；当前为 `#[allow(dead_code)]` |
| **被调用者** | [get_cpu_set_information](../winapi.rs/README.md)（获取缓存的 CPU 集合拓扑） |
| **Win32 API** | 无直接调用；依赖来自 [GetSystemCpuSetInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/systeminformationapi/nf-systeminformationapi-getsystemcpusetinformation) 的缓存数据 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 反向：掩码到 CPU 集合 ID | [cpusetids_from_mask](cpusetids_from_mask.md) |
| 索引到 CPU 集合 ID | [cpusetids_from_indices](cpusetids_from_indices.md) |
| CPU 集合 ID 到索引 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合拓扑缓存 | [CpuSetData](CpuSetData.md) |
| 亲和性检查工具 | [is_affinity_unset](is_affinity_unset.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
