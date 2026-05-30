# parse_cpu_spec 函数 (config.rs)

将 CPU 规格说明字符串解析为排序的 CPU 索引列表。这是整个配置系统中使用的核心 CPU 集解析器——每个 CPU 别名定义、亲和性字段、CPU 集字段和主线程字段最终都通过此函数处理。

## 语法

```rust
pub fn parse_cpu_spec(s: &str) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

`s: &str`

描述一组逻辑处理器索引的字符串。接受以下格式：

| 格式 | 示例 | 结果 |
|------|------|------|
| 空字符串或 `"0"` | `""`、`"0"` | 空列表（无 CPU / 不更改） |
| 十六进制位掩码 | `"0xFF"`、`"0x15"` | 从置位位导出的 CPU 索引（旧格式，≤64 核） |
| 包含范围 | `"0-7"` | `[0, 1, 2, 3, 4, 5, 6, 7]` |
| 分号分隔的单个值 | `"0;4;8"` | `[0, 4, 8]` |
| 混合范围和单个值 | `"0-3;8;12-15"` | `[0, 1, 2, 3, 8, 12, 13, 14, 15]` |

## 返回值

`List<[u32; CONSUMER_CPUS]>`——一个排序、去重的 [SmallVec](../collections.rs/README.md) 支持的 `u32` CPU 索引列表。当输入为空、`"0"` 或无法解析时返回空列表。

## 备注

### 解析算法

1. **修剪**输入字符串。
2. **空/零检查：** 如果修剪后的字符串为空或恰好为 `"0"`，立即返回空列表。值 `"0"` 是配置约定，表示"无 CPU 限制"。
3. **十六进制前缀检测：** 如果字符串以 `"0x"` 或 `"0X"` 开头，将剩余部分解析为十六进制 `u64`，并委托给 [mask_to_cpu_indices](mask_to_cpu_indices.md) 提取置位位的位置。解析失败时返回空列表。
4. **分号拆分：** 在 `';'` 上拆分字符串。每个段独立处理：
   - 如果段中包含 `'-'`，将破折号前后的部分解析为 `u32` 起始值和结束值，并插入包含范围 `[start, end]` 中的每个整数。
   - 否则，将段解析为单个 `u32` CPU 索引。
   - 在插入过程中跳过重复值（`contains` 检查）。
5. **排序**结果列表为升序后再返回。

### 设计选择

- **分号作为分隔符：** 冒号（`:`）在配置规则格式中保留为字段分隔符，因此 CPU 规格使用分号分隔单个值或范围。
- **`"0"` 表示不更改：** 这是一个有意的哨兵值。配置字段中的 `0` 表示服务不应修改相应设置。这与十六进制掩码 `0x0`（同样导致空列表）不同。
- **十六进制掩码限于 64 核：** 十六进制位掩码格式是从 Process Lasso 兼容性继承的旧式便利功能。在拥有超过 64 个逻辑处理器的系统上，请改用范围语法（`"0-7;64-71"`）。
- **无错误报告：** 分号分隔规格中的无效标记会被静默跳过（`unwrap_or` 为 `0` 或忽略解析失败）。验证错误在更高层通过检查结果列表是否意外为空来捕获。

### 栈分配

返回类型 `List<[u32; CONSUMER_CPUS]>` 是一个 `SmallVec`，在栈上内联存储最多 `CONSUMER_CPUS`（32）个 CPU 索引。典型消费者系统（≤32 核）永远不会进行堆分配。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用者** | [parse_alias](parse_alias.md)、[resolve_cpu_spec](resolve_cpu_spec.md)、[parse_mask](parse_mask.md)、[convert](convert.md) |
| **被调用者** | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 位掩码到 CPU 索引 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU 索引到位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 格式化 CPU 索引用于显示 | [format_cpu_indices](format_cpu_indices.md) |
| 别名解析包装函数 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 集合类型 | [List / CONSUMER_CPUS](../collections.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
