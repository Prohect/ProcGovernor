# sort_and_group_config 函数 (config.rs)

将共享相同规则设置的进程自动分组到命名组块中，生成紧凑且减少重复的配置文件。这是一个命令行工具函数，通过 `-autogroup` 标志调用。

## 语法

```rust
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## 参数

`in_file: Option<String>`

输入配置文件的路径。必须为 `Some`；如果为 `None`，函数记录错误并立即返回。

`out_file: Option<String>`

将写入分组配置的输出文件路径。必须为 `Some`；如果为 `None`，函数记录错误并立即返回。

## 返回值

此函数不返回值。结果写入输出文件，诊断消息通过日志记录。

## 备注

### 算法

该函数执行多遍转换：

1. **序言提取：** 文件顶部的注释行（`#`）、空行、常量（`@`）或别名（`*`）被收集到序言部分，该部分在输出中原样保留。

2. **规则收集：** 每条规则行和组块被分解为其成员进程名称和规则字符串（单行规则为第一个冒号之后的所有内容，组为闭合 `}:` 之后的部分）。规则字符串作为分组键。

3. **合并：** 具有相同规则字符串的规则将其成员列表合并。这合并了恰好共享相同设置的单个规则和现有组块。

4. **去重和排序：** 在每个合并组内，成员名称按字母顺序排序并去重。

5. **输出生成：** 对于每个唯一的规则字符串：
   - 如果仅有一个进程具有该规则，则作为单行规则发出：`process.exe:rule_string`
   - 如果多个进程共享该规则，则作为命名组块发出。组按顺序命名为 `grp_0`、`grp_1` 等。

### 组格式化

组根据长度以两种风格之一格式化：

- **内联风格**（当整行在 128 个字符以内时）：
  ```
  grp_0 { proc1.exe: proc2.exe: proc3.exe }:normal:*ecore:0:0:low:none:0:1
  ```

- **多行风格**（当内联表示超过 128 个字符时）：
  ```
  grp_1 {
      proc1.exe: proc2.exe: proc3.exe
      proc4.exe: proc5.exe
  }:normal:*pcore:0:0:none:none:0:1
  ```

  在多行模式下，成员被打包到最长 128 个字符的行中，使用 4 空格缩进（`const INDENT: &str = "    "`）。行内成员用冒号分隔。

### 序言保留

常量（`@MIN_ACTIVE_STREAK = 3`）、别名（`*pcore = 0-7`）和前导注释按原顺序和形式保留。仅规则行和组块被重新组织。序言中的尾部空行被修剪为单个分隔行。

### 规则顺序稳定性

唯一规则字符串按它们在输入文件中首次遇到的顺序发出。这保留了原始配置的整体组织，同时合并了重复项。

### 典型用法

```
ProcGovernor.exe -autogroup -in config.txt -out config_grouped.txt
```

这是一个一次性转换工具——输出文件不会被服务自动使用。用户应检查输出并手动替换原始配置文件。

### 日志记录

完成后，函数记录一条总结：

```
Auto-grouped: 42 total process rules → 10 individual + 32 processes merged into 8 groups
Written to config_grouped.txt
```

### 错误处理

- 如果缺少 `-in` 或 `-out`，记录错误消息并返回。
- 如果输入文件无法读取，记录错误并返回。
- 如果输出文件无法创建或写入，记录错误并返回。
- 输入中未闭合的组块被静默跳过。

### 与配置解析器的交互

该函数复用 [collect_members](collect_members.md) 和 [collect_group_block](collect_group_block.md) 来解析输入，确保对组语法和成员名称的一致处理。它**不**调用 [parse_and_insert_rules](parse_and_insert_rules.md) 或验证规则字段——输出保留规则字符串与输入中完全相同。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用者** | `main.rs` 中的 CLI 分派（通过 `-autogroup` 标志调用） |
| **被调用者** | [collect_members](collect_members.md)、[collect_group_block](collect_group_block.md)、`std::fs::read_to_string`、`std::fs::File::create`、`std::io::Write` |
| **依赖项** | [HashMap](../collections.rs/README.md) |
| **所需权限** | 输入和输出路径的文件系统读写权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| Process Lasso 转换器（相关工具） | [convert](convert.md) |
| 组块解析器 | [collect_group_block](collect_group_block.md) |
| 成员收集器 | [collect_members](collect_members.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
