# parse_mask 函数 (config.rs)

将 CPU 规格说明字符串解析为对应位掩码的便捷函数。将 [parse_cpu_spec](parse_cpu_spec.md) 和 [cpu_indices_to_mask](cpu_indices_to_mask.md) 组合为一次调用。

## 语法

```rust
pub fn parse_mask(s: &str) -> usize
```

## 参数

`s: &str`

任意 [parse_cpu_spec](parse_cpu_spec.md) 接受的格式的 CPU 规格说明字符串：范围（`"0-7"`）、分号分隔的索引（`"0;4;8"`）、十六进制掩码（`"0xFF"`）或哨兵值 `"0"`（无 CPU）。

## 返回值

`usize`——一个位掩码，如果 CPU 索引 *N* 出现在解析的规格中，则位 *N* 被置位。当输入为空、`"0"` 或无法解析时返回 `0`。

## 备注

此函数是一个两步流水线：

1. 输入字符串 `s` 传递给 [parse_cpu_spec](parse_cpu_spec.md)，生成排序的 `List<[u32; CONSUMER_CPUS]>`（CPU 索引列表）。
2. 结果索引列表传递给 [cpu_indices_to_mask](cpu_indices_to_mask.md)，生成 `usize` 位掩码。

### 转换示例

| 输入 | 中间索引 | 输出掩码 |
|------|----------|----------|
| `"0-3"` | `[0, 1, 2, 3]` | `0xF` |
| `"0;2;4"` | `[0, 2, 4]` | `0x15` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0x0` |

### 64 核限制

由于 [cpu_indices_to_mask](cpu_indices_to_mask.md) 静默丢弃 ≥ 64 的索引，解析规格中任何超过处理器 63 的 CPU 索引都不会在返回的位掩码中表示。对于超过 64 个逻辑处理器的系统，建议直接使用 [parse_cpu_spec](parse_cpu_spec.md) 返回的 CPU 索引列表。

### 允许死代码

该函数标注了 `#[allow(dead_code)]`，表示它在所有构建配置中可能没有活跃的调用者，但作为公共工具函数被保留。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 公开（`pub fn`） |
| **调用者** | 工具 / 外部消费者 |
| **被调用者** | [parse_cpu_spec](parse_cpu_spec.md)、[cpu_indices_to_mask](cpu_indices_to_mask.md) |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 索引到掩码转换 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 掩码到索引转换 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| 显示格式化 | [format_cpu_indices](format_cpu_indices.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
