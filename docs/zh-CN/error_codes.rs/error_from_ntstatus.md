# error_from_ntstatus 函数 (error_codes.rs)

将 NTSTATUS 状态码映射为人类可读的常量名称。在整个项目中使用，以便在本机 API 调用（`Nt*` / `Zw*` 函数）返回失败状态码时生成有意义的日志消息。

## 语法

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

## 参数

`status: i32`

由本机 Windows API 函数返回的 NTSTATUS 状态码。NTSTATUS 值是有符号 32 位整数，其最高位表示严重性：第 31 位被置位的值（当解释为 `i32` 时为负数）为错误码，零表示成功，小正值表示信息性状态。

## 返回值

`String` — 状态码的符号名称（例如 `"STATUS_ACCESS_DENIED"`）。如果代码不在查找表中，则返回格式为 `"NTSTATUS_0x{code:08X}"` 的回退字符串，使用无符号表示形式。

## 备注

### 已识别代码

该函数通过 `i32::cast_unsigned` 获取 `status` 的无符号表示并在其上进行 `match`，可识别以下 NTSTATUS 代码：

| 代码 | 名称 |
|------|------|
| `0x00000000` | `STATUS_SUCCESS` |
| `0x00000001` | `STATUS_WAIT_1` |
| `0xC0000001` | `STATUS_UNSUCCESSFUL` |
| `0xC0000002` | `STATUS_NOT_IMPLEMENTED` |
| `0xC0000003` | `STATUS_INVALID_INFO_CLASS` |
| `0xC0000004` | `STATUS_INFO_LENGTH_MISMATCH` |
| `0xC0000008` | `STATUS_INVALID_HANDLE` |
| `0xC000000D` | `STATUS_INVALID_PARAMETER` |
| `0xC0000017` | `STATUS_NO_MEMORY` |
| `0xC0000018` | `STATUS_CONFLICTING_ADDRESSES` |
| `0xC0000022` | `STATUS_ACCESS_DENIED` |
| `0xC0000023` | `STATUS_BUFFER_TOO_SMALL` |
| `0xC0000034` | `STATUS_OBJECT_NAME_NOT_FOUND` |
| `0xC000004B` | `STATUS_THREAD_IS_TERMINATING` |
| `0xC0000061` | `STATUS_PRIVILEGE_NOT_HELD` |
| `0xC00000BB` | `STATUS_NOT_SUPPORTED` |
| `0xC000010A` | `STATUS_PROCESS_IS_TERMINATING` |

### 无符号转换

该函数使用 `i32::cast_unsigned(status)` 将有符号 `i32` 转换为其等价的无符号 `u32` 位表示，然后进行匹配。这避免了负数字面量模式的需求，并与 Windows SDK 头文件中使用的 NTSTATUS 代码的常规十六进制表示（例如 `ntstatus.h`）保持一致。

### 与 error_from_code_win32 的关系

此函数是处理 NTSTATUS 代码的对应函数，与处理 Win32 错误码的 [error_from_code_win32](error_from_code_win32.md) 配成一对。两个函数覆盖不同的错误码空间：

- **Win32 错误** — 由 `GetLastError()` 等函数返回，通常为小正整数值。
- **NTSTATUS 代码** — 由 `Nt*`/`Zw*` 本机 API 函数返回，错误通常为 `0xC*` 值。

调用方根据产生错误码的具体 API 来选择相应的函数。

### 回退格式

未知代码格式化为 `"NTSTATUS_0x{code:08X}"`，其中 `code` 被强制转换为 `u32` 以便显示，确保无论输入的符号如何，输出均为一致的无符号十六进制格式。

## 需求

| | |
|---|---|
| **模块** | `src/error_codes.rs` |
| **调用方** | [apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md)、[winapi.rs](../winapi.rs/README.md) 中的本机 API 封装 |
| **被调用方** | 无 |
| **Win32 API** | 无（纯查找函数） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [error_codes.rs](README.md) |
| Win32 错误码查找 | [error_from_code_win32](error_from_code_win32.md) |
| IO 优先级应用 | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| 内存优先级应用 | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
