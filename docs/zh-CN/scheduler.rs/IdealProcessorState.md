# IdealProcessorState 结构体 (scheduler.rs)

跟踪单个线程的理想处理器分配状态。存储当前和先前的处理器组/编号对，使服务能够检测更改并避免在多次轮询迭代中进行冗余的 `SetThreadIdealProcessorEx` 调用。

## 语法

```rust
#[derive(Debug, Clone, Copy)]
pub struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `current_group` | `u16` | 当前分配的理想处理器的处理器组。在具有单个处理器组（≤64 个逻辑处理器）的系统上，始终为 `0`。 |
| `current_number` | `u8` | `current_group` 中当前设置为线程理想处理器的逻辑处理器编号。 |
| `previous_group` | `u16` | 上一次轮询迭代中理想处理器的处理器组。用于检测分配是否已更改。 |
| `previous_number` | `u8` | 上一次轮询迭代中 `previous_group` 内的逻辑处理器编号。 |
| `is_assigned` | `bool` | 如果此线程已被服务显式分配了理想处理器，则为 `true`。当为 `false` 时，线程保留其操作系统默认的理想处理器。由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 用于区分托管线程和非托管线程。 |

## 方法

### new

```rust
pub fn new() -> Self
```

创建一个新的 `IdealProcessorState`，所有字段清零且 `is_assigned` 设为 `false`。

**返回值**

`IdealProcessorState` — 表示未分配线程的默认初始化状态。

### Default

```rust
impl Default for IdealProcessorState {
    fn default() -> Self
}
```

委托给 `IdealProcessorState::new()`。

## 备注

- `current_*` / `previous_*` 分离设计使得无需查询操作系统即可检测更改。在每个应用周期中，调用者将新值写入 `current_group` 和 `current_number`，然后与 `previous_group` 和 `previous_number` 比较，以决定是否需要调用 `SetThreadIdealProcessorEx`。
- 成功更新后，调用者将 `current_*` 复制到 `previous_*` 以供下一次迭代使用。
- 该结构体派生 `Copy` 因为它是一个小型、仅栈的值类型（10 字节），没有堆分配或资源所有权。
- `is_assigned` 标志由 `select_top_threads_with_hysteresis` 通过其 `is_currently_assigned` 回调检查，以确定线程是否已经占据主线程槽位。这是防止提升/降级颠簸的迟滞逻辑的关键输入。

### 与 Windows PROCESSOR_NUMBER 的关系

`current_group`/`current_number` 对直接映射到 [SetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) 和 [GetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) 使用的 Windows `PROCESSOR_NUMBER` 结构体。服务对这些 API 的包装器位于 [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) 和 [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md)。

## 需求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用者** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) |
| **依赖** | 无（纯数据结构） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [scheduler.rs](README.md) |
| 父级结构体 | [ThreadStats](ThreadStats.md) |
| 理想处理器设置包装器 | [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) |
| 理想处理器获取包装器 | [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md) |
| 带有迟滞的线程选择 | [PrimeThreadScheduler](PrimeThreadScheduler.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
