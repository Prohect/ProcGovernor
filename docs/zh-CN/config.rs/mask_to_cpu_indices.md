# mask_to_cpu_indices 函数 (config.rs)

将 64 位 CPU 位掩码转换为排序的单个 CPU 索引列表。掩码中的每个置位位对应输出列表中的一个逻辑处理器编号。

## 语法

```rust
fn mask_to_cpu_indices(mask: u64) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

`mask: u64`

一个 64 位位掩码，位 *N* 被置位表示逻辑处理器 *N* 应包含在输出列表中。例如，`0x15`（二进制 `10101`）表示处理器 0、2 和 4。

## 返回值

`List<[u32; CONSUMER_CPUS]>`——一个栈分配的 small vector，包含按升序排序的 CPU 索引。如果 `mask` 为 `0`，则返回空列表。

## 备注

该函数遍历位位置 0 到 63，使用 `(mask >> i) & 1 == 1` 测试每个位。匹配的位置通过 `Iterator::collect()` trait 收集到输出列表中。由于位从 LSB 到 MSB 测试，结果列表天然按升序排序。

### 限制

- 该函数最多支持 64 个逻辑处理器，这是单个 Windows 处理器组的限制。超过 64 个处理器的系统需要使用由 [parse_cpu_spec](parse_cpu_spec.md) 处理的范围型 CPU 规格（`"0-7;64-71"`），而非位掩码表示法。
- 返回类型使用 `CONSUMER_CPUS`（32）作为内联容量。如果掩码中设置了超过 32 个 CPU，列表会溢出到堆分配，这对于配置解析热路径来说是可以接受的。

### 可见性

此函数是模块私有的（`fn`，而非 `pub fn`）。它专门由 [parse_cpu_spec](parse_cpu_spec.md) 在解析十六进制 CPU 掩码字符串时调用（例如 `"0xFF"`）。

### 逆向操作

此函数的逆向操作是 [cpu_indices_to_mask](cpu_indices_to_mask.md)，它将 CPU 索引切片转换回 `usize` 位掩码。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用者** | [parse_cpu_spec](parse_cpu_spec.md)（十六进制掩码分支） |
| **被调用者** | 无 |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 逆向转换 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| CPU 索引格式化器 | [format_cpu_indices](format_cpu_indices.md) |
| 十六进制掩码解析器 | [parse_mask](parse_mask.md) |
| 集合类型 | [List](../collections.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
