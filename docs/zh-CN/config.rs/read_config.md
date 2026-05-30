# read_config 函数 (config.rs)

主配置文件读取器。打开指定文件，遍历所有行，并根据每行的前缀字符将其分派到适当的子解析器。返回一个完全填充的 [ConfigResult](ConfigResult.md)，包含所有已解析的规则、别名、常量、组、错误和警告。

## 语法

```rust
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## 参数

`path: P`

配置文件的文件系统路径。接受任何实现 `AsRef<Path>` 的类型，包括 `&str`、`String` 和 `PathBuf`。文件预期为 UTF-8 编码，每行一个指令。

## 返回值

[ConfigResult](ConfigResult.md)——一个结构体，包含：

- `process_level_configs` 和 `thread_level_configs` 映射，填充了已解析的规则，按等级和进程名称索引。
- 从 `@CONSTANT = value` 指令填充的 `constants`（如未指定则使用默认值）。
- 别名、组、组成员、进程规则、冗余规则和线程级配置的计数器。
- `errors`——致命解析错误，导致配置无效。
- `warnings`——非致命问题，记录日志但不阻止使用配置。

如果文件无法打开，函数返回仅包含一条错误消息且无规则的 `ConfigResult`。

## 备注

### 行分派

函数将所有行读入内存，然后使用基于索引的循环遍历（以支持多行组块，其将索引前进多于一行）。每个非空、非注释行根据其前导字符进行分派：

| 前缀 | 处理器 | 描述 |
|------|--------|------|
| `#` | （跳过） | 注释行——完全忽略。 |
| `@` | [parse_constant](parse_constant.md) | 期望 `@NAME = value`。解析调优常量，如 `MIN_ACTIVE_STREAK`、`KEEP_THRESHOLD`、`ENTRY_THRESHOLD`。 |
| `*` | [parse_alias](parse_alias.md) | 期望 `*name = cpu_spec`。定义命名 CPU 别名，可被后续规则行引用。 |
| 包含 `{` | 组块 | 行中包含组定义。如果 `}` 在同一行，则为内联组；否则 [collect_group_block](collect_group_block.md) 读取后续行直到找到 `}`。 |
| （其他） | 单条规则 | 行在 `:` 上拆分，第一段是进程名称。剩余段转发给 [parse_and_insert_rules](parse_and_insert_rules.md)。 |

### 解析顺序

指令按文件顺序处理。这意味着：

1. **常量**（`@`）应出现在依赖调优行为的任何规则之前，但它们可以出现在任何位置。
2. **别名**（`*`）必须在任何引用它们的规则行之前定义。对未定义别名的前向引用产生错误。
3. **组和规则**可以以任何顺序出现；对同一进程名称的后定义覆盖先定义并产生警告。

### 组处理

组块有两种形式：

**内联组**（单行）：
```
group_name { proc1.exe: proc2.exe }:normal:*ecore:0:0:low:none:0:1
```

**多行组：**
```
group_name {
    proc1.exe: proc2.exe
    proc3.exe
}:normal:*ecore:0:0:low:none:0:1
```

在这两种情况下，组名从 `{` 之前的文本提取。如果名称为空，则从行号生成匿名标签 `"anonymous@L{N}"`。成员由 [collect_members](collect_members.md) 收集，`}:` 之后的规则后缀被拆分并转发给 [parse_and_insert_rules](parse_and_insert_rules.md)。未闭合的组（缺少 `}`）产生致命错误。

### 单行规则处理

非组、非指令行预期具有以下格式：

```
process.exe:priority:affinity:cpuset:prime:io:memory:ideal:grade
```

最少需要的字段数为 3（名称、优先级、亲和性）。更少的字段产生致命错误。进程名称（字段 0）被转为小写并作为单元素成员列表传递给 [parse_and_insert_rules](parse_and_insert_rules.md)。

### 错误处理

- 如果配置文件无法打开（权限、文件缺失），添加一条错误并立即返回。
- 子解析器（常量、别名、规则）的解析错误累积在 `ConfigResult.errors` 和 `ConfigResult.warnings` 中。调用者应在使用结果前检查 [is_valid](ConfigResult.md)。

### CPU 别名作用域

别名存储在 `read_config` 调用的本地 `HashMap<String, List<[u32; CONSUMER_CPUS]>>` 中。它们不会持久化在返回的 `ConfigResult` 中——仅保留 `aliases_count`。这意味着别名是解析时概念，在存储到规则结构体之前完全解析为具体的 CPU 索引列表。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用者** | [hotreload_config](hotreload_config.md)、主服务启动 |
| **被调用者** | [parse_constant](parse_constant.md)、[parse_alias](parse_alias.md)、[collect_members](collect_members.md)、[collect_group_block](collect_group_block.md)、[parse_and_insert_rules](parse_and_insert_rules.md) |
| **依赖项** | `std::fs::File`、`std::io::BufReader`、[ConfigResult](ConfigResult.md)、[HashMap](../collections.rs/README.md) |
| **所需权限** | 配置文件路径的文件系统读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 聚合解析结果 | [ConfigResult](ConfigResult.md) |
| 规则插入逻辑 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 别名定义 | [parse_alias](parse_alias.md) |
| 常量定义 | [parse_constant](parse_constant.md) |
| 组块收集器 | [collect_group_block](collect_group_block.md) |
| 热重载包装函数 | [hotreload_config](hotreload_config.md) |
| Process Lasso 转换器 | [convert](convert.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
