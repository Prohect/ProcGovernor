# is_affinity_unset 函数 (winapi.rs)

检查进程的 CPU 亲和性掩码是否等于完整的系统亲和性掩码，表示未应用自定义亲和性限制。由 `-find` 模式用于识别尚未配置特定 CPU 亲和性的进程。

## 语法

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 要查询的进程标识符。 |
| `process_name` | `&str` | 进程映像名称，用于错误日志记录，以及在访问被拒绝时记录到 `-find` 失败集合中。 |

## 返回值

`bool` — 如果进程的当前亲和性掩码等于系统范围的亲和性掩码（表示没有亲和性限制生效），则为 `true`。如果进程有自定义亲和性掩码，或在检查期间任何 API 调用失败，则为 `false`。

## 备注

### 算法

该函数执行以下步骤：

1. **打开进程** — 调用 `OpenProcess`，使用 `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION` 访问权限。
2. **查询亲和性** — 调用 `GetProcessAffinityMask` 获取进程的 `current_mask` 和系统范围的 `system_mask`。
3. **比较** — 如果 `current_mask == system_mask`，返回 `true`。
4. **关闭句柄** — 在返回前通过 `CloseHandle` 关闭进程句柄。

### 错误处理

| 失败点 | 行为 |
|------|------|
| `OpenProcess` 失败 | 通过 `log_to_find` 记录；如果 Win32 错误码为 `5`（`ERROR_ACCESS_DENIED`），将 `process_name` 插入全局 fail-find 集合。返回 `false`。 |
| `OpenProcess` 返回无效句柄 | 通过 `log_to_find` 记录。返回 `false`。 |
| `GetProcessAffinityMask` 失败 | 通过 `log_to_find` 记录；如果是 `ERROR_ACCESS_DENIED`，插入 fail-find 集合。返回 `false`。 |

失败时保守地返回 `false` 意味着无法查询的进程被视为"已配置"，防止它们出现在 `-find` 模式输出中。

### Fail-find 集合

当 `OpenProcess` 或 `GetProcessAffinityMask` 调用期间遇到 `ERROR_ACCESS_DENIED`（代码 5）时，进程名称被插入全局 fail-find 集合（通过 `get_fail_find_set!()` 宏访问）。此集合由 `-find` 模式用于跟踪服务由于权限不足而无法检查的进程，允许单独报告它们。

### 句柄管理

此函数打开和关闭自己的临时进程句柄，而不是重用来自 [ProcessHandle](ProcessHandle.md) 的句柄。这是因为 `-find` 模式作为一次性扫描运行，而不是持久轮询循环，因此缓存句柄没有好处。

### 系统亲和性掩码

`GetProcessAffinityMask` 的 `system_mask` 输出表示系统上可用的所有逻辑处理器（在当前处理器组内）。在具有 8 个逻辑处理器的系统上，这将是 `0xFF`。其 `current_mask` 等于 `system_mask` 的进程具有默认的"使用所有 CPU"配置。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | `src/main.rs` 中的 `-find` 模式 |
| **被调用者** | `OpenProcess`（Win32）、`GetProcessAffinityMask`（Win32）、`CloseHandle`（Win32）、`GetLastError`、[error_from_code_win32](../error_codes.rs/error_from_code_win32.md)、[log_to_find](../logging.rs/log_to_find.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)（`PROCESS_SET_INFORMATION \| PROCESS_QUERY_INFORMATION`）、[GetProcessAffinityMask](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |
| **权限** | 需要 `PROCESS_QUERY_INFORMATION` 访问权限；[SeDebugPrivilege](enable_debug_privilege.md) 扩展对受保护进程的访问 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 亲和性应用函数 | [apply_affinity](../apply.rs/apply_affinity.md) |
| 亲和性的 CPU 集合替代方案 | [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) |
| CPU 集合 ID ↔ 掩码转换 | [cpusetids_from_mask](cpusetids_from_mask.md)、[mask_from_cpusetids](mask_from_cpusetids.md) |
| 调试权限启用 | [enable_debug_privilege](enable_debug_privilege.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
