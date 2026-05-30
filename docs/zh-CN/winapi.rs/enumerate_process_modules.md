# enumerate_process_modules 函数 (winapi.rs)

枚举目标进程中所有已加载的模块，返回每个模块的基地址、大小和名称。由 [resolve_address_to_module](resolve_address_to_module.md) 内部用于填充按 PID 的模块缓存，以实现地址到模块的解析。

## 语法

```rust
fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 应枚举其模块的目标进程的进程标识符。 |

## 返回值

`Vec<(usize, usize, String)>` — 一个元组向量，其中每个元素表示一个已加载的模块：

| 元组索引 | 类型 | 描述 |
|------|------|------|
| `.0` | `usize` | 模块在目标进程虚拟地址空间中的基地址（`MODULEINFO::lpBaseOfDll`）。 |
| `.1` | `usize` | 模块映像的大小（以字节为单位）（`MODULEINFO::SizeOfImage`）。 |
| `.2` | `String` | 模块的基名称（例如，`"kernel32.dll"`、`"ntdll.dll"`），通过 `GetModuleBaseNameW` 获取。 |

在以下情况下返回空的 `Vec`：
- 无法打开进程（例如，访问被拒绝、无效 PID）。
- 进程句柄在打开后无效。
- `EnumProcessModulesEx` 失败（例如，32 位进程查询 64 位进程而没有 WOW64 支持）。

## 备注

### 实现步骤

1. **打开进程** — 通过 `OpenProcess` 以 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 打开目标进程。两个访问权限都是必需的：`PROCESS_QUERY_INFORMATION` 用于模块枚举，`PROCESS_VM_READ` 用于从目标进程的地址空间读取模块名称。

2. **枚举模块** — 调用 `EnumProcessModulesEx` 并设置 `LIST_MODULES_ALL` 以检索 32 位和 64 位模块的模块句柄。该函数使用固定大小的 1024 个 `HMODULE` 槽位数组，这对绝大多数进程来说是足够的。

3. **查询每个模块** — 对于每个返回的模块句柄：
   - `GetModuleInformation` 检索包含 `lpBaseOfDll`（基地址）和 `SizeOfImage`（模块大小）的 `MODULEINFO` 结构体。
   - `GetModuleBaseNameW` 将模块的基名称检索为 UTF-16 字符串，通过 `String::from_utf16_lossy` 转换为 Rust `String`。
   - 其中任一调用失败的模块被静默跳过。

4. **清理** — 在返回前通过 `CloseHandle` 关闭进程句柄，无论在成功还是提前退出路径上。

### 模块限制

该函数在栈上为 1024 个模块句柄分配空间。如果进程有超过 1024 个已加载模块，仅返回前 1024 个。实际上，即使复杂的应用程序也很少超过几百个模块。

### 可见性

此函数是模块私有的（`fn`，而非 `pub fn`），仅由 [resolve_address_to_module](resolve_address_to_module.md) 在模块缓存填充期间调用。外部代码应使用 `resolve_address_to_module` 而不是直接调用此函数。

### 错误处理

所有 Win32 API 失败通过返回空结果或跳过失败的模块来处理——不记录或传播任何错误。这是有意为之，因为模块枚举是一个尽力而为的诊断功能；失败不会影响服务的核心功能。

### 访问要求

`PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 的组合比服务中其他地方使用的受限访问句柄更为严格。这意味着对于 [get_process_handle](get_process_handle.md) 以受限访问成功的进程，模块枚举可能会失败。[SeDebugPrivilege](enable_debug_privilege.md) 通常可以解决大多数进程的访问问题。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [resolve_address_to_module](resolve_address_to_module.md)（通过 [MODULE_CACHE](README.md) 填充） |
| **被调用者** | `OpenProcess`、`EnumProcessModulesEx`、`GetModuleInformation`、`GetModuleBaseNameW`、`CloseHandle`（Win32） |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)、[EnumProcessModulesEx](https://learn.microsoft.com/zh-cn/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex)、[GetModuleInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/psapi/nf-psapi-getmoduleinformation)、[GetModuleBaseNameW](https://learn.microsoft.com/zh-cn/windows/win32/api/psapi/nf-psapi-getmodulebasenamew)、[CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | 目标进程上的 `PROCESS_QUERY_INFORMATION` 和 `PROCESS_VM_READ`；推荐 [SeDebugPrivilege](enable_debug_privilege.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 地址解析消费者 | [resolve_address_to_module](resolve_address_to_module.md) |
| 模块缓存清理 | [drop_module_cache](drop_module_cache.md) |
| 线程起始地址查询 | [get_thread_start_address](get_thread_start_address.md) |
| 存储 start_address 的 ThreadStats | [ThreadStats](../scheduler.rs/ThreadStats.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
