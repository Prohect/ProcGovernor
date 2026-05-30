# format_cpu_indices 函数 (config.rs)

将 CPU 索引切片格式化为紧凑、可读的范围字符串。连续的索引会被折叠为破折号分隔的范围，非连续的索引用逗号分隔。

## 语法

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## 参数

`cpus: &[u32]`

要格式化的 CPU 索引切片。切片无需预先排序；函数在内部对副本进行排序。重复值会保留在排序中，但不影响输出范围。

## 返回值

`String`——CPU 索引的紧凑表示。如果输入切片为空则返回 `"0"`。否则，返回以逗号分隔的字符串，其中连续段被折叠为范围。

**输出示例：**

| 输入 | 输出 |
|------|------|
| `[]` | `"0"` |
| `[0, 1, 2, 3]` | `"0-3"` |
| `[0, 2, 4]` | `"0,2,4"` |
| `[0, 1, 2, 8, 9, 10, 16]` | `"0-2,8-10,16"` |
| `[5]` | `"5"` |

## 备注

函数将输入复制到栈分配的 `List<[u32; CONSUMER_CPUS]>`（一个 `SmallVec`）中，并在格式化前排序。这确保无论输入顺序如何，都能正确检测范围。

### 范围折叠算法

函数遍历排序列表。对于每个起始索引，只要下一个值等于当前值加一就扩展连续段。当段结束时：

- 如果段的起始和结束相等，则追加单个值。
- 如果起始和结束不同，则追加 `"起始-结束"` 范围字符串。

范围和单个值用逗号连接。

### 空返回零约定

空输入返回字符串 `"0"`，因为在 ProcGovernor 配置格式中 `0` 表示"不更改"或"未配置"。此约定允许 `format_cpu_indices` 的输出直接插入到配置文件行中，无需对空 CPU 集做特殊处理。

### 与 parse_cpu_spec 的关系

`format_cpu_indices` 是 [parse_cpu_spec](parse_cpu_spec.md) 在范围和单个值格式上的逆向操作。即，`parse_cpu_spec(format_cpu_indices(cpus))` 生成原始输入的排序去重版本。然而，`format_cpu_indices` 从不输出十六进制掩码格式，即使输入最初是从十六进制字符串解析的。

### 用法

此函数在整个代码库中用于：

- 生成显示 CPU 分配的人类可读日志消息（亲和性更改、CPU 集更改、主线程固定）。
- 在 [convert](convert.md) 和 [sort_and_group_config](sort_and_group_config.md) 工具中生成配置文件输出。
- 在诊断消息中调试显示 CPU 集。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用者** | `apply_affinity`、`apply_process_default_cpuset`、`apply_prime_threads_promote`、`apply_prime_threads_demote`、`reset_thread_ideal_processors`、`apply_ideal_processors`、日志函数 |
| **被调用者** | 无 |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 逆向解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 位掩码到索引 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| 索引到位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 集合类型 | [List / CONSUMER_CPUS](../collections.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
