# get_thread_ideal_processor_ex 函数 (winapi.rs)

检索线程的当前理想处理器分配。包装 Windows `GetThreadIdealProcessorEx` API，将处理器组和编号作为 `PROCESSOR_NUMBER` 结构体返回。

## 语法

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `thread_handle` | `HANDLE` | 具有 `THREAD_QUERY_LIMITED_INFORMATION` 或 `THREAD_QUERY_INFORMATION` 访问权限的有效线程句柄。通常源自 [ThreadHandle](ThreadHandle.md)`.r_limited_handle` 或 `.r_handle`。 |

## 返回值

`Result<PROCESSOR_NUMBER, Error>` — 成功时，返回一个 `PROCESSOR_NUMBER` 结构体，包含：

| 字段 | 类型 | 描述 |
|------|------|------|
| `Group` | `u16` | 线程理想处理器的处理器组。在单组系统（≤64 个逻辑处理器）上，始终为 `0`。 |
| `Number` | `u8` | 组内作为线程当前理想处理器的逻辑处理器编号。 |
| `Reserved` | `u8` | 保留；始终为 `0`。 |

失败时，返回包含底层 Win32 错误码的 `windows::core::Error`（例如，如果句柄无效或缺乏所需访问权限，则为 `ERROR_INVALID_HANDLE`）。

## 备注

- 该函数在栈上分配一个默认的 `PROCESSOR_NUMBER`，并将其作为输出参数传递给 `GetThreadIdealProcessorEx`。成功时，返回填充的结构体。
- 理想处理器是一个调度*提示*——Windows 倾向于在指定处理器上运行线程，但在负载下可能将其调度到别处。此函数读取当前提示，该提示可能由操作系统、应用程序本身或先前对 [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) 的调用设置。
- 此函数由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 逻辑用于在决定是否需要更新之前读取当前理想处理器。返回的组和编号与 [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) 进行比较，以避免冗余的 `SetThreadIdealProcessorEx` 调用。

### 与 set_thread_ideal_processor_ex 的关系

| 函数 | 方向 | API |
|------|------|------|
| **get_thread_ideal_processor_ex** | 读取 | `GetThreadIdealProcessorEx` |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | 写入 | `SetThreadIdealProcessorEx` |

两个函数操作相同的每个线程的理想处理器属性。获取变体仅需读访问权限；设置变体需要写访问权限（`THREAD_SET_INFORMATION`）。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **被调用者** | 无（薄 Win32 包装器） |
| **Win32 API** | [GetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |
| **权限** | 线程句柄上的 `THREAD_QUERY_LIMITED_INFORMATION` 或 `THREAD_QUERY_INFORMATION` 访问权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 写入对应函数 | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| 理想处理器跟踪状态 | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| 线程句柄包装器 | [ThreadHandle](ThreadHandle.md) |
| 理想处理器应用逻辑 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
