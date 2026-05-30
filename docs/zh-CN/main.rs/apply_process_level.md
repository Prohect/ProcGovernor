# apply_process_level 函数 (main.rs)

为给定的 PID 打开进程句柄，并在单次遍历中应用所有进程级设置 — 优先级类、CPU 亲和性掩码、默认 CPU 集合、IO 优先级和内存优先级。这是当进程首次匹配配置规则时（或启用了连续应用时每个迭代）调用一次的进程级包装器。

## 语法

```rust
fn apply_process_level<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

包含所需进程级设置（优先级、亲和性 CPU、CPU 集合 CPU、IO 优先级、内存优先级）的 [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md)。打开进程句柄时，`config.name` 字段用于错误报告。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

一个惰性评估的闭包，返回以线程 ID 为键的进程线程映射。通过调用者（[`apply_config`](apply_config.md)）中的 `OnceCell` 与线程级路径共享。线程映射是 [`apply_affinity`](../apply.rs/apply_affinity.md)（在亲和性更改后重置理想处理器）和 [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md) 所需要的。

`dry_run: bool`

当为 **true** 时，所有下游 `apply_*` 函数记录*将要*更改的内容而不调用任何 Windows API。当为 **false** 时，更改将应用于活动进程。

`apply_configs: &mut ApplyConfigResult`

变更描述和错误消息的累加器。参见 [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md)。

## 返回值

此函数不返回值。所有结果（应用的变更、遇到的错误）记录在 `apply_configs` 中。

## 备注

该函数遵循固定的操作顺序：

1. **打开句柄** — 调用 [`get_process_handle`](../winapi.rs/get_process_handle.md) 获取 [`ProcessHandle`](../winapi.rs/ProcessHandle.md)。如果句柄无法打开（例如，访问被拒绝、进程已退出），函数立即返回，无任何效果且不记录错误。
2. **应用优先级** — 委托给 [`apply_priority`](../apply.rs/apply_priority.md)。
3. **应用亲和性** — 委托给 [`apply_affinity`](../apply.rs/apply_affinity.md)。传递一个局部 `current_mask` 变量以捕获进程当前的亲和性掩码供下游使用。
4. **应用 CPU 集合** — 委托给 [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md)。
5. **应用 IO 优先级** — 委托给 [`apply_io_priority`](../apply.rs/apply_io_priority.md)。
6. **应用内存优先级** — 委托给 [`apply_memory_priority`](../apply.rs/apply_memory_priority.md)。
7. **释放句柄** — 在所有操作完成后显式释放 `ProcessHandle`。

每个 `apply_*` 函数独立检查其对应的配置字段是否设置为 `None`，并在如此时短路。这意味着仅指定了优先级和亲和性的配置不会触及 IO 或内存优先级。

### 线程枚举成本

只有当某个 `apply_*` 函数实际需要线程信息时（例如，`apply_affinity` 重置理想处理器，或 `apply_process_default_cpuset` 重新分配线程），才会调用 `threads` 闭包。底层的 `OnceCell` 确保在进程级和线程级应用之间最多进行一次线程枚举。

## 需求

| | |
|---|---|
| **模块** | [`src/main.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/main.rs) |
| **调用者** | [`apply_config`](apply_config.md) |
| **被调函数** | [`get_process_handle`](../winapi.rs/get_process_handle.md)、[`apply_priority`](../apply.rs/apply_priority.md)、[`apply_affinity`](../apply.rs/apply_affinity.md)、[`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md)、[`apply_io_priority`](../apply.rs/apply_io_priority.md)、[`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| **Win32 API** | 无直接调用（委托给被调函数） |
| **权限** | `SeDebugPrivilege`（用于打开提升进程的句柄） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 线程级对应函数 | [`apply_thread_level`](apply_thread_level.md) |
| 组合调用者 | [`apply_config`](apply_config.md) |
| 应用引擎概览 | [apply.rs](../apply.rs/README.md) |
| 配置结构体 | [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| 结果累加器 | [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
