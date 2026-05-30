# collect_group_block 函数 (config.rs)

从多行组块定义中收集进程名称，读取行直到遇到闭合的 `}` 大括号。返回累积的成员列表、闭合大括号后的任何规则后缀，以及要从中恢复解析的行索引。

## 语法

```rust
fn collect_group_block(
    lines: &[String],
    start_index: usize,
    first_line_content: &str,
) -> Option<(Vec<String>, Option<String>, usize)>
```

## 参数

`lines: &[String]`

从配置文件读取的完整行集。函数从 `start_index` 向前读取，寻找闭合的 `}` 大括号。

`start_index: usize`

开始扫描的基于零的行索引。这应该是包含开始 `{` 大括号的行之后的直接下一行。函数读取 `lines[start_index]`、`lines[start_index + 1]` 等，直到找到 `}` 或到达文件末尾。

`first_line_content: &str`

在同行的开始 `{` 之后出现的任何内容。例如，给定 `group_name { proc1.exe: proc2.exe`，此参数将是 `"proc1.exe: proc2.exe"`。如果开始大括号是其行上的最后一个非空白字符，则此参数将是空字符串或空白。此内容在读取后续行之前被解析为成员名称。

## 返回值

`Option<(Vec<String>, Option<String>, usize)>`——成功时返回 `Some(...)`，如果在未找到闭合 `}` 的情况下到达文件末尾则返回 `None`。

元组元素为：

| 索引 | 类型 | 描述 |
|------|------|------|
| `.0` | `Vec<String>` | 收集的组成员名称，已转为小写并修剪。从 `first_line_content` 和所有后续行（直至并包含 `}` 所在行）中累积。 |
| `.1` | `Option<String>` | 闭合 `}` 之后的规则后缀。如果包含 `}` 的行在大括号后有 `:rule_fields...`，则第一个 `:` 之后的文本被返回为 `Some(rule_fields)`。如果 `}` 之后没有 `:`，则返回 `None`。 |
| `.2` | `usize` | 包含 `}` 的行之后的行索引。调用者应从该索引恢复解析。 |

## 备注

### 解析算法

1. 如果 `first_line_content` 非空且不以 `#` 开头，则将其传递给 [collect_members](collect_members.md)，提取在开始 `{` 同一行上出现的任何进程名称。
2. 函数然后从 `start_index` 开始遍历 `lines`：
   - 如果一行包含 `}`，则 `}` 之前的内容被解析为成员，`}` 之后的内容被检查规则后缀（前导 `:` 后的文本），函数成功返回。
   - 否则，整行（如果非空且非注释）通过 [collect_members](collect_members.md) 解析为成员。
3. 如果循环耗尽所有行而未找到 `}`，函数返回 `None`，表示未闭合的组块。

### 规则后缀提取

闭合 `}` 之后紧跟的文本确定该组是否有关联规则。解析器期望格式 `}:priority:affinity:...`。前导 `:` 被剥离，剩余文本成为在 `.1` 中返回的规则后缀。此后缀随后被 [parse_and_insert_rules](parse_and_insert_rules.md) 在 `:` 上拆分以创建实际的配置条目。

如果 `}` 之后没有 `:`（例如，闭合大括号独自占一行），[read_config](read_config.md) 中的调用者将其视为错误——没有规则定义的组。

### 注释处理

组块内部以 `#` 开头的行被静默忽略，不贡献成员。这允许用户在组内注释掉单个进程名称：

```
my_group {
    active_game.exe
    # disabled_game.exe
    another_game.exe
}:normal:*ecore:0:0:none:none:0:1
```

### 单行 vs. 多行

此函数仅为多行组调用——即开始 `{` 行不同时包含闭合 `}` 的情况。单行组（例如 `group { a: b }:rule`）由 [read_config](read_config.md) 内联处理，不调用此函数。

### 错误情况

当函数返回 `None` 时，[read_config](read_config.md) 中的调用者将错误推送到 [ConfigResult](ConfigResult.md)，格式为 `"Line {N}: Unclosed group '{name}' - missing }"`，并跳到下一行。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有（`fn`）——配置模块内部 |
| **调用者** | [read_config](read_config.md)（多行组解析）、[sort_and_group_config](sort_and_group_config.md)（自动分组读取器） |
| **被调用者** | [collect_members](collect_members.md) |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 成员名称解析器 | [collect_members](collect_members.md) |
| 为收集的成员插入规则 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 主配置读取器 | [read_config](read_config.md) |
| 带错误报告的配置结果 | [ConfigResult](ConfigResult.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
