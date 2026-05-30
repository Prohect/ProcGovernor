# parse_alias 函数 (config.rs)

从配置文件中解析 `*name = cpu_spec` 别名定义行，并将结果 CPU 索引列表插入别名映射中。别名为 CPU 规格说明提供命名快捷方式，可在配置的其他位置通过 `*name` 语法引用。

## 语法

```rust
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## 参数

`name: &str`

别名名称（不含前导 `*` 前缀），已由调用者转为小写。例如，给定配置行 `*PCore = 0-7`，调用者传递 `"pcore"`。空名称触发错误。

`value: &str`

`=` 号右侧的 CPU 规格说明字符串，已由调用者修剪。这被转发给 [parse_cpu_spec](parse_cpu_spec.md) 解析为 CPU 索引列表。支持范围（`0-7`）、分号分隔的值（`0;4;8`）、十六进制掩码（`0xFF`）或 `"0"`（空集）。

`line_number: usize`

配置文件中此别名定义出现的基于 1 的行号。包含在错误和警告消息中。

`cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>`

**\[入参, 出参\]** 配置解析期间构建的别名映射。成功时，以 `name` 为键、解析的 CPU 列表为值插入新条目。如果同名别名已存在，则被静默覆盖。

`result: &mut ConfigResult`

**\[入参, 出参\]** 解析结果累加器。成功时，`result.aliases_count` 递增。失败时（空名称），错误被推送到 `result.errors`。如果解析的 CPU 集为空且值不是字面量 `"0"`，警告被推送到 `result.warnings`。

## 返回值

此函数不返回值。结果通过 `cpu_aliases` 和 `result` 传达。

## 备注

### 验证

该函数执行两项验证检查：

1. **空名称：** 如果 `name` 为空，推送错误 `"Line {N}: Empty alias name"`，函数返回而不插入任何内容。
2. **非零值的空 CPU 集：** 如果 [parse_cpu_spec](parse_cpu_spec.md) 返回空列表但 `value` 不是字面量 `"0"`，发出警告，因为用户可能在 CPU 规格说明中出现笔误。别名仍然以空 CPU 列表插入。

### 别名命名约定

- 别名名称不区分大小写。调用者在传递前将名称转为小写。
- 对别名名称字符没有限制，超出配置解析器的行拆分逻辑施加的限制（名称是 `*` 和 `=` 之间的所有内容）。
- 常见约定包括 `pcore`、`ecore`、`perf`、`efficiency`，或应用程序特定的名称如 `game_cpus`。

### 覆盖行为

如果同一别名名称在配置文件中被定义多次，每次后续定义静默覆盖前一个。对重新定义不发出警告——仅最终值保留在 `cpu_aliases` 中。引用别名的规则行将使用其行位置时的当前值（因为别名在顺序解析中先于规则解析）。

### 配置语法示例

```
*pcore = 0-7
*ecore = 8-19
*all = 0-19
*fast = 0;2;4;6
```

### 与 resolve_cpu_spec 的交互

一旦定义，别名被 [resolve_cpu_spec](resolve_cpu_spec.md) 消费，当 CPU 字段值以 `*` 开头时使用。例如，规则行中的亲和性字段 `*pcore` 导致 `resolve_cpu_spec` 在别名映射中查找 `"pcore"` 并返回存储的 CPU 列表 `[0, 1, 2, 3, 4, 5, 6, 7]`。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有（`fn`） |
| **调用者** | [read_config](read_config.md)（对遇到的每个 `*name = value` 行） |
| **被调用者** | [parse_cpu_spec](parse_cpu_spec.md) |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 别名消费者 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 常量解析器（类似模式） | [parse_constant](parse_constant.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 聚合结果 | [ConfigResult](ConfigResult.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
