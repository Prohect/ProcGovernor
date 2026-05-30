# get_thread_start_address 函数 (winapi.rs)

通过 `NtQueryInformationThread` 使用 `ThreadQuerySetWin32StartAddress` 信息类（类 9）检索线程的 Win32 起始地址。起始地址标识线程被创建以执行的函数，从而支持基于模块的理想处理器分配和诊断报告。

## 语法

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `thread_handle` | `HANDLE` | 具有至少 `THREAD_QUERY_INFORMATION` 访问权限的有效线程句柄。通常是 [ThreadHandle](ThreadHandle.md) 的 `r_handle` 字段。使用 `r_limited_handle`（`THREAD_QUERY_LIMITED_INFORMATION`）**不**足以进行此查询——使用类 9 的 `NtQueryInformationThread` 需要完全查询权限。 |

## 返回值

`usize` — 线程入口点位于进程地址空间中的虚拟地址。如果查询失败（例如，句柄缺乏足够访问权限或线程已退出），返回 `0`。

## 备注

### NT API 详情

该函数调用未公开的 `NtQueryInformationThread` FFI 绑定（从 `ntdll.dll` 链接），使用：

| 参数 | 值 |
|------|------|
| `thread_handle` | 从调用者传入 |
| `thread_information_class` | `9`（`ThreadQuerySetWin32StartAddress`） |
| `thread_information` | 指向 `usize` 输出缓冲区的指针 |
| `thread_information_length` | `size_of::<usize>()`（64 位系统上为 8 字节） |
| `return_length` | 指向 `u32` 的指针（调用后未使用） |

该函数检查返回值上的 `NTSTATUS.is_ok()`。如果状态指示成功，则返回起始地址；否则返回 `0`。

### 起始地址与入口点

此信息类返回的 Win32 起始地址是传递给 `CreateThread` / `_beginthreadex`（或等效函数）的函数的地址。这可能与内核看到的实际线程入口点（`RtlUserThreadStart`）不同，后者在跳转到用户指定的起始函数之前执行 CRT 初始化。

### 服务中的用法

起始地址用于两个关键场景：

1. **基于模块的理想处理器分配** — 地址传递给 [resolve_address_to_module](resolve_address_to_module.md) 以确定线程属于哪个模块（DLL/EXE）。配置中的模块前缀匹配规则随后将特定的理想处理器分配给匹配模块内的线程。

2. **诊断报告** — [ThreadStats](../scheduler.rs/ThreadStats.md) `Debug` 实现和 [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) 中的进程退出报告都将起始地址解析为 `"模块.dll+0x偏移"` 字符串，以实现人类可读的线程识别。

### 失败情况

该函数在以下情况下返回 `0`：

- 线程句柄没有 `THREAD_QUERY_INFORMATION` 访问权限（仅获取了受限访问）。
- 线程已退出且句柄已过期。
- NT API 调用返回意外的 `NTSTATUS` 错误。

返回值 `0` 被下游消费者视为"未知"。[resolve_address_to_module](resolve_address_to_module.md) 对零地址返回字符串 `"0x0"`。

### 句柄要求

必须使用 [ThreadHandle](ThreadHandle.md) 的 `r_handle` 字段（以 `THREAD_QUERY_INFORMATION` 打开），而不是 `r_limited_handle`。如果 `r_handle` 无效（在线程句柄获取期间访问被拒绝），此函数将返回 `0`。调用者应在调用此函数之前检查 `r_handle.is_invalid()` 以避免浪费的系统调用。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[ThreadStats](../scheduler.rs/ThreadStats.md) |
| **被调用者** | `NtQueryInformationThread`（ntdll FFI） |
| **NT API** | 使用 `ThreadQuerySetWin32StartAddress`（类 9）的 `NtQueryInformationThread` |
| **权限** | 需要在线程句柄上具有 `THREAD_QUERY_INFORMATION` 访问权限；对于受保护进程，可能需要 [SeDebugPrivilege](enable_debug_privilege.md) 来获取该句柄 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 地址到模块解析 | [resolve_address_to_module](resolve_address_to_module.md) |
| 线程句柄管理 | [ThreadHandle](ThreadHandle.md)、[get_thread_handle](get_thread_handle.md) |
| 存储起始地址的线程统计 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 理想处理器分配 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
