# ThreadHandle 结构体 (winapi.rs)

以多个访问级别打开的 Windows 线程句柄集合的 RAII 包装器。在释放时自动关闭所有有效句柄。当结构体存在时，`r_limited_handle` 始终有效；如果相应的 `OpenThread` 调用因权限不足而失败，其他句柄可能无效。

## 语法

```rust
#[derive(Debug)]
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## 成员

| 成员 | 类型 | 访问权限 | 描述 |
|------|------|------|------|
| `r_limited_handle` | `HANDLE` | `THREAD_QUERY_LIMITED_INFORMATION` | 始终有效的读句柄，用于基本线程查询，如周期时间检索。这是构建结构体所需的最低访问级别。 |
| `r_handle` | `HANDLE` | `THREAD_QUERY_INFORMATION` | 完全读句柄，用于高级查询，如 [get_thread_start_address](get_thread_start_address.md)（通过 `NtQueryInformationThread`）。如果访问被拒绝，可能为无效句柄（`HANDLE::default()`）。使用前以 `is_invalid()` 检查。 |
| `w_limited_handle` | `HANDLE` | `THREAD_SET_LIMITED_INFORMATION` | 受限写句柄，用于设置线程 CPU 集合分配等操作。如果访问被拒绝，可能无效。 |
| `w_handle` | `HANDLE` | `THREAD_SET_INFORMATION` | 完全写句柄，用于 [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) 和线程优先级更改等操作。如果访问被拒绝，可能无效。 |

## Drop

```rust
impl Drop for ThreadHandle {
    fn drop(&mut self);
}
```

关闭结构体持有的所有句柄。`r_limited_handle` 始终无条件关闭（它保证有效）。其余三个句柄（`r_handle`、`w_limited_handle`、`w_handle`）仅在它们不是无效句柄时才被关闭，由 `HANDLE::is_invalid()` 确定。

### 句柄关闭顺序

1. `r_limited_handle` — 始终关闭
2. `r_handle` — 如果非无效则关闭
3. `w_limited_handle` — 如果非无效则关闭
4. `w_handle` — 如果非无效则关闭

每次关闭调用 `CloseHandle` 并丢弃结果。

## 备注

### 与 ProcessHandle 的区别

与 [ProcessHandle](ProcessHandle.md)（对其可选句柄使用 `Option<HANDLE>`）不同，`ThreadHandle` 使用原始 `HANDLE` 值，并依赖 `is_invalid()` 来区分有效和失败的句柄。这种设计差异的存在是因为线程句柄在紧密循环中更频繁地被访问（每个线程、每次迭代），避免 `Option` 解包可以降低开销。

### 访问级别策略

四个句柄表示读/写 × 受限/完全访问的矩阵：

|  | 受限 | 完全 |
|--|------|------|
| **读** | `r_limited_handle` | `r_handle` |
| **写** | `w_limited_handle` | `w_handle` |

受限访问权限更经常成功（例如，对于受保护进程），但支持较少的操作。调用者应尽可能优先使用受限句柄，并在使用前回退到对完全句柄检查 `is_invalid()`。

### ThreadStats 中的缓存

`ThreadHandle` 实例缓存在 [ThreadStats](../scheduler.rs/ThreadStats.md)`.handle` 中，以避免在每次轮询迭代时重新打开句柄。句柄在线程的生存期内或直到父进程退出且 [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) 释放统计条目为止持久存在。

### Send 安全性

`ThreadHandle` 是 `Send` 的，因为 `HANDLE` 是指针大小值的薄包装器，Windows 句柄可以从同一进程内的任何线程使用。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [get_thread_handle](get_thread_handle.md)、[ThreadStats](../scheduler.rs/ThreadStats.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **被调用者** | `CloseHandle`（Win32） |
| **Win32 API** | [CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | 释放时无要求；创建时需要适当的线程访问权限（参见 [get_thread_handle](get_thread_handle.md)） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 线程句柄工厂 | [get_thread_handle](get_thread_handle.md) |
| 较低级别的线程打开器 | [try_open_thread](try_open_thread.md) |
| 进程句柄对应物 | [ProcessHandle](ProcessHandle.md) |
| 线程统计缓存 | [ThreadStats](../scheduler.rs/ThreadStats.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
