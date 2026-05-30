# filter_indices_by_mask 函数 (winapi.rs)

将逻辑 CPU 索引列表过滤为仅那些在给定亲和性掩码中对应位被设置的索引。用于将配置的 CPU 索引列表与实际进程亲和性取交集，确保主线程绑定和 CPU 集合操作仅针对进程被允许使用的处理器。

## 语法

```rust
pub fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpu_indices` | `&[u32]` | 要过滤的逻辑 CPU 索引（从 0 开始）切片。每个值表示一个逻辑处理器编号（例如，`0`、`1`、`5`、`12`）。 |
| `affinity_mask` | `usize` | 一个位掩码，其中位 *N* 被设置表示逻辑处理器 *N* 是被允许的。通常从 `GetProcessAffinityMask` 获取或从配置构建。 |

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 仅包含 `cpu_indices` 中那些在 `affinity_mask` 中对应位被设置的索引的栈分配列表。元素顺序匹配输入顺序。如果没有索引通过过滤，则返回空列表。

## 备注

每个索引的过滤条件是：

```
idx < 64 && ((1usize << idx) & affinity_mask) != 0
```

- **64 位限制：** 索引 ≥ 64 被静默排除，因为 64 位 Windows 上的 `usize` 只能通过位位置表示处理器 0–63。具有超过 64 个逻辑处理器的系统使用处理器组，这些由 CPU 集合 API 单独处理。
- **不去重：** 如果输入 `cpu_indices` 包含重复值，则输出也会为通过过滤的任何重复值包含重复项。
- **不排序：** 输出保留原始输入排序。如果需要排序输出，调用者必须单独对结果排序。
- 该函数使用迭代器链（`filter` + `copied` + `collect`）并且不进行堆分配——结果被收集到栈支持的 `List<[u32; CONSUMER_CPUS]>` 中。

### 典型用法

此函数由 [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) 调用，以确保主线程 CPU 索引仅限于进程当前亲和性掩码内的那些处理器。例如，如果配置指定主 CPU 为 `[4, 5, 6, 7]`，但进程具有 `0x3F`（CPU 0–5）的亲和性掩码，则该函数返回 `[4, 5]`。

### 示例

| `cpu_indices` | `affinity_mask` | 结果 |
|-------------|---------------|------|
| `[0, 2, 4]` | `0x15`（位 0, 2, 4） | `[0, 2, 4]` |
| `[0, 2, 4]` | `0x05`（位 0, 2） | `[0, 2]` |
| `[0, 1, 2]` | `0x00` | `[]`（空） |
| `[64, 65]` | `0xFFFFFFFFFFFFFFFF` | `[]`（索引 ≥ 64 被排除） |

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)，其他需要将配置索引与进程亲和性取交集的应用函数 |
| **被调用者** | 无（纯计算） |
| **Win32 API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 索引 → CPU 集合 ID 转换 | [cpusetids_from_indices](cpusetids_from_indices.md) |
| 掩码 → CPU 集合 ID 转换 | [cpusetids_from_mask](cpusetids_from_mask.md) |
| CPU 集合 ID → 索引转换 | [indices_from_cpusetids](indices_from_cpusetids.md) |
| CPU 集合 ID → 掩码转换 | [mask_from_cpusetids](mask_from_cpusetids.md) |
| 主线程提升 | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
