# ApplyConfigResult 结构体 (apply.rs)

用于收集配置应用过程中产生的变更和错误的累加器。模块中的每个 `apply_*` 函数都接收此结构体的可变引用，并追加人类可读的消息来描述所做的更改或遇到的故障。

## 语法

```ProcGovernor/src/apply.rs#L31-35
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `changes` | `Vec<String>` | 已成功应用或在试运行模式下将要应用的设置的描述。格式：`"$operation details"`，调用者会自动添加 `"{pid:>5}::{config.name}::"` 前缀。 |
| `errors` | `Vec<String>` | 应用过程中遇到的故障描述。格式：`"$fn_name: [$operation][$error_message] details"`。仅记录唯一错误（参见 [log_error_if_new](log_error_if_new.md)）。 |

## 方法

### new

```ProcGovernor/src/apply.rs#L38-40
pub fn new() -> Self
```

创建一个新的 `ApplyConfigResult`，包含空的 `changes` 和 `errors` 向量。委托给 `Default::default()`。

### add_change

```ProcGovernor/src/apply.rs#L45-47
pub fn add_change(&mut self, change: String)
```

向 `changes` 向量追加一条变更描述。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `change` | `String` | 对已应用（或试运行）变更的人类可读描述。 |

### add_error

```ProcGovernor/src/apply.rs#L51-53
pub fn add_error(&mut self, error: String)
```

向 `errors` 向量追加一条错误描述。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `error` | `String` | 对故障的人类可读描述，包含操作标签和 Win32/NTSTATUS 错误文本。 |

### is_empty

```ProcGovernor/src/apply.rs#L55-57
pub fn is_empty(&self) -> bool
```

若 `changes` 和 `errors` 均为空则返回 `true`，表示当前应用周期内对该进程未发生任何可观察的操作。

**返回值**

`bool` — 当未记录任何变更和错误时为 `true`。

## 备注

- 所有 `apply_*` 函数均以 `&mut ApplyConfigResult` 作为最后一个参数，在整个模块中遵循一致的约定。
- 调用者（位于 `main.rs`）使用 `is_empty()` 来跳过对不需要变更的进程的日志输出。
- `add_change` 和 `add_error` 被标记为 `#[inline(always)]`，因为它们在应用循环的每个热路径上都被调用。

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用者** | 所有 `apply_*` 函数、[log_error_if_new](log_error_if_new.md)、主应用循环 |
| **依赖** | 无（纯数据结构体） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概览 | [apply.rs](README.md) |
| 错误去重辅助函数 | [log_error_if_new](log_error_if_new.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
