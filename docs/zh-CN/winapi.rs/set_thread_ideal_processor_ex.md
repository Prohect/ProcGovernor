# set_thread_ideal_processor_ex 函数 (winapi.rs)

Windows `SetThreadIdealProcessorEx` API 的包装器，为线程设置理想处理器提示，指定处理器组和该组内的逻辑处理器编号。返回先前的理想处理器分配。

## 语法

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `thread_handle` | `HANDLE` | 具有 `THREAD_SET_INFORMATION` 访问权限的有效线程句柄。通常源自 [ThreadHandle](ThreadHandle.md)`.w_handle`。调用者必须在调用此函数前验证句柄非无效。 |
| `group` | `u16` | 理想处理器的处理器组编号。在单组系统（≤64 个逻辑处理器）上，始终为 `0`。 |
| `number` | `u8` | 指定 `group` 中要设置为线程理想处理器的逻辑处理器编号。例如，`0` 目标为组中的第一个处理器。 |

## 返回值

`Result<PROCESSOR_NUMBER, Error>` — 成功时，返回线程**先前**的理想处理器，作为 `PROCESSOR_NUMBER` 结构体（包含 `Group`、`Number` 和 `Reserved` 字段）。失败时，返回包含底层 Win32 错误码的 Windows `Error`。

## 备注

- 该函数从 `group` 和 `number` 参数构造一个 `PROCESSOR_NUMBER` 结构体（`Reserved` 设置为 `0`），并将其传递给 `SetThreadIdealProcessorEx`。一个可变的 `previous` 输出参数捕获 API 返回的先前理想处理器分配。
- 理想处理器是一个**调度提示**，而非硬约束。Windows 调度器倾向于在指定处理器上调度线程，但如果理想处理器繁忙，可以将线程放在线程亲和性掩码内的任何处理器上。对于硬性 CPU 绑定，使用通过 `SetThreadSelectedCpuSets` 的 CPU 集合。
- 此函数由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 和 [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) 调用，以将调度器引导到对延迟敏感的线程的特定核心。
- 此函数返回的先前理想处理器由 [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) 用于在多次轮询迭代中跟踪分配更改。

### 错误场景

| 条件 | 行为 |
|------|------|
| 无效的线程句柄 | 返回带有 `ERROR_INVALID_HANDLE` 的 `Err(Error)` |
| 句柄缺乏 `THREAD_SET_INFORMATION` 访问权限 | 返回带有 `ERROR_ACCESS_DENIED` 的 `Err(Error)` |
| 无效的 group 或 number（超出系统拓扑） | 返回 `Err(Error)`——行为取决于操作系统版本 |

### 与 get_thread_ideal_processor_ex 的关系

此函数是 [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) 的写入对应函数。它们一起构成了用于管理线程理想处理器提示的读/写对。服务通常读取当前分配以检测外部工具的更改，然后在配置指示不同处理器时写入新分配。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **被调用者** | `SetThreadIdealProcessorEx`（Win32） |
| **Win32 API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **权限** | 需要线程句柄上的 `THREAD_SET_INFORMATION` 访问权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 读取对应函数 | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| 理想处理器状态跟踪 | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| 提供写访问的线程句柄 | [ThreadHandle](ThreadHandle.md) |
| 设置理想处理器的应用函数 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
