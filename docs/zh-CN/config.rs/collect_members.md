# collect_members 函数 (config.rs)

从文本片段中解析冒号分隔的进程名称列表，并将其追加到累加器向量中。内部用于从单行和多行组块定义中提取组成员名称。

## 语法

```rust
fn collect_members(text: &str, members: &mut Vec<String>)
```

## 参数

`text: &str`

包含冒号分隔的进程名称的字符串片段。通常是组定义中 `{` 和 `}` 之间的内容，或多行组块中的单行。每个项目周围的空白被修剪。以 `#` 开头的项目被视为注释并跳过。

`members: &mut Vec<String>`

**\[入参, 出参\]** 累加器向量，解析出的进程名称被追加到其中。现有条目被保留；新名称被推到末尾。所有名称转换为小写。

## 返回值

此函数不返回值。结果通过 `members` 出参传达。

## 备注

### 解析规则

1. 输入 `text` 在 `:`（冒号）字符上拆分。
2. 每个结果片段被修剪掉前导和尾部空白。
3. 片段被转换为小写，供下游不区分大小写地匹配。
4. 空片段和以 `#` 开头的片段被丢弃。
5. 所有存活的片段被推入 `members`。

### 分隔符选择

使用冒号 `:` 分隔符是因为主规则语法已经使用 `:` 作为字段分隔符。在组块内部（`{` 和 `}` 之间），进程名称用冒号分隔——而非逗号或分号——以保持与外部配置行格式的一致性。

### 不进行去重

`collect_members` **不**检查重复名称。如果同一进程名称在输入中出现多次或跨多次调用出现，它将在 `members` 中出现多次。去重（如果需要）由调用者或后续阶段（如 [sort_and_group_config](sort_and_group_config.md)）处理。

### 示例

给定输入文本 `"game.exe: helper.exe: # comment: tool.EXE"`，函数将 `["game.exe", "helper.exe", "tool.exe"]` 追加到 `members`。注释片段被跳过，`tool.EXE` 被转为小写。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有（`fn`）——配置模块内部 |
| **调用者** | [read_config](read_config.md)（内联组解析）、[collect_group_block](collect_group_block.md)（多行组解析）、[sort_and_group_config](sort_and_group_config.md)（自动分组工具） |
| **被调用者** | 无（仅使用 `str` 标准库方法） |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 多行组块收集器 | [collect_group_block](collect_group_block.md) |
| 消费组成员的规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 主配置读取器 | [read_config](read_config.md) |
| 自动分组工具 | [sort_and_group_config](sort_and_group_config.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
