# get_handles 函数 (apply.rs)

从 [ProcessHandle](../winapi.rs/ProcessHandle.md) 中提取读句柄和写句柄，优先选择完整访问句柄而非受限访问句柄。这是所有需要查询或修改进程状态的每进程应用函数的通用入口点。

## 语法

```ProcGovernor/src/apply.rs#L63-64
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>)
```

## 参数

`process_handle: &ProcessHandle`

一个 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用，该结构体包含最多四个句柄：读和写访问的完整和受限变体。

## 返回值

返回一个元组 `(Option<HANDLE>, Option<HANDLE>)`，其中：

| 索引 | 描述 |
|-------|-------------|
| `.0` | 读句柄 — 如果 `r_handle` 存在（`Some`），则为该值，否则为 `Some(r_limited_handle)`。 |
| `.1` | 写句柄 — 如果 `w_handle` 存在（`Some`），则为该值，否则为 `Some(w_limited_handle)`。 |

当 `ProcessHandle` 成功打开时，两个元素始终为 `Some`，因为 `r_limited_handle` 和 `w_limited_handle` 是无条件填充的。`Option` 包装器允许调用者使用 `let (Some(r), Some(w)) = get_handles(...) else { return; }` 进行模式匹配以在句柄无效时退出。

## 备注

- 此函数标记为 `#[inline(always)]`，因为它在每个进程级应用函数（`apply_priority`、`apply_affinity`、`apply_io_priority`、`apply_memory_priority`、`apply_process_default_cpuset`）的顶部都被调用。
- 完整访问句柄（`r_handle` / `w_handle`）优先，因为它们以更广泛的访问权限（`PROCESS_QUERY_INFORMATION` / `PROCESS_SET_INFORMATION`）打开。受限句柄（`r_limited_handle` / `w_limited_handle`）使用 `PROCESS_QUERY_LIMITED_INFORMATION` / `PROCESS_SET_LIMITED_INFORMATION`，这可能不足以完成所有操作（例如，IO 优先级的 `NtQueryInformationProcess` 需要完全查询访问权限）。
- 该函数**不**验证返回的句柄是否确实是有效的 Win32 句柄；调用者依赖 Win32 API 调用本身在句柄过期或不足时报告错误。

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **可见性** | `fn`（crate 私有） |
| **调用者** | [apply_priority](apply_priority.md)、[apply_affinity](apply_affinity.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_io_priority](apply_io_priority.md)、[apply_memory_priority](apply_memory_priority.md) |
| **被调函数** | 无 |
| **API / 操作系统** | 无 |
| **权限** | 无（句柄打开在别处完成） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| ProcessHandle 结构体 | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| 进程优先级应用 | [apply_priority](apply_priority.md) |
| 进程亲和性应用 | [apply_affinity](apply_affinity.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
