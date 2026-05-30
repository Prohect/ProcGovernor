# winapi 模块 (ProcGovernor)

`winapi` 模块为 ProcGovernor 中使用的 Windows API 函数提供安全的 Rust 包装器，涵盖进程和线程句柄管理、CPU 集合拓扑查询和转换、权限提升、亲和性检查、理想处理器分配、模块枚举、地址解析、定时器分辨率控制和子进程清理。所有句柄持有类型通过 `Drop` 实现 RAII 以防止资源泄漏。

## 外部 FFI

该模块直接链接到 `ntdll.dll` 以调用 `windows` crate 未提供的未公开 NT API：

| 函数 | 描述 |
|------|------|
| `NtQueryInformationProcess` | 查询每个进程的信息类（IO 优先级、内存优先级等）。 |
| `NtQueryInformationThread` | 查询每个线程的信息类（通过 `ThreadQuerySetWin32StartAddress` 获取起始地址）。 |
| `NtSetInformationProcess` | 设置每个进程的信息类（IO 优先级、内存优先级）。 |
| `NtSetTimerResolution` | 将全局 Windows 定时器分辨率设置为指定的间隔。 |

## 静态变量

| 名称 | 类型 | 描述 |
|------|------|------|
| `CPU_SET_INFORMATION` | `Lazy<Mutex<Vec<CpuSetData>>>` | 从 `GetSystemCpuSetInformation` 获取的缓存系统 CPU 集合拓扑。在首次访问时填充一次，并在所有 CPU 索引 ↔ CPU 集合 ID 转换中重用。 |
| `MODULE_CACHE` | `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` | 每个 PID 的模块基地址范围和名称缓存。在首次调用 [resolve_address_to_module](resolve_address_to_module.md) 时按进程填充，并在进程退出时通过 [drop_module_cache](drop_module_cache.md) 清除。 |

## 结构体

| 名称 | 描述 |
|------|------|
| [CpuSetData](CpuSetData.md) | 轻量级缓存条目，将 CPU 集合 ID 与逻辑处理器索引配对。 |
| [ProcessHandle](ProcessHandle.md) | RAII 包装器，用于最多四个不同访问级别（读/写 × 受限/完全）的进程句柄。 |
| [ThreadHandle](ThreadHandle.md) | RAII 包装器，用于最多四个不同访问级别（读/写 × 受限/完全）的线程句柄。 |

## 函数 — 句柄管理

| 名称 | 描述 |
|------|------|
| [get_process_handle](get_process_handle.md) | 以多个访问级别打开一个进程，返回 [ProcessHandle](ProcessHandle.md)。 |
| [get_thread_handle](get_thread_handle.md) | 以多个访问级别打开一个线程，返回 [ThreadHandle](ThreadHandle.md)。 |
| [try_open_thread](try_open_thread.md) | 较低级别的辅助函数，以指定访问权限打开单个线程句柄。 |

## 函数 — CPU 集合转换

| 名称 | 描述 |
|------|------|
| [cpusetids_from_indices](cpusetids_from_indices.md) | 将逻辑 CPU 索引（0, 1, 2…）转换为不透明的 Windows CPU 集合 ID。 |
| [cpusetids_from_mask](cpusetids_from_mask.md) | 将亲和性位掩码转换为 CPU 集合 ID。 |
| [indices_from_cpusetids](indices_from_cpusetids.md) | 将 CPU 集合 ID 转换回逻辑 CPU 索引。 |
| [mask_from_cpusetids](mask_from_cpusetids.md) | 将 CPU 集合 ID 转换为亲和性位掩码。 |
| [filter_indices_by_mask](filter_indices_by_mask.md) | 将 CPU 索引列表过滤为仅在亲和性掩码中存在的索引。 |

## 函数 — 权限与提升

| 名称 | 描述 |
|------|------|
| [is_running_as_admin](is_running_as_admin.md) | 检查当前进程是否具有提升的管理员令牌。 |
| [request_uac_elevation](request_uac_elevation.md) | 使用 `runas` 动词重新启动当前进程以触发 UAC 提升。 |
| [enable_debug_privilege](enable_debug_privilege.md) | 在当前进程令牌上启用 `SeDebugPrivilege`。 |
| [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) | 在当前进程令牌上启用 `SeIncreaseBasePriorityPrivilege`。 |

## 函数 — 进程与线程检查

| 名称 | 描述 |
|------|------|
| [is_affinity_unset](is_affinity_unset.md) | 检查进程的亲和性掩码是否等于完整的系统掩码（即未应用限制）。 |
| [get_thread_start_address](get_thread_start_address.md) | 通过 `NtQueryInformationThread` 检索线程的 Win32 起始地址。 |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | `SetThreadIdealProcessorEx` 的包装器，设置理想处理器提示。 |
| [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) | `GetThreadIdealProcessorEx` 的包装器，查询当前理想处理器。 |

## 函数 — 模块解析

| 名称 | 描述 |
|------|------|
| [resolve_address_to_module](resolve_address_to_module.md) | 使用缓存的模块数据将虚拟地址解析为 `"模块.dll+0x偏移"` 字符串。 |
| [drop_module_cache](drop_module_cache.md) | 移除特定 PID 的缓存模块数据。 |
| [enumerate_process_modules](enumerate_process_modules.md) | 列出进程中所有模块的基地址、大小和名称。 |

## 函数 — 系统工具

| 名称 | 描述 |
|------|------|
| [terminate_child_processes](terminate_child_processes.md) | 终止当前进程的所有子进程（启动时清理）。 |
| [set_timer_resolution](set_timer_resolution.md) | 通过 `NtSetTimerResolution` 设置 Windows 全局定时器分辨率。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 进程快照模块 | [process.rs](../process.rs/README.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 配置应用 | [apply.rs](../apply.rs/README.md) |
| 优先级类型定义 | [priority.rs](../priority.rs/README.md) |
| CLI 参数解析 | [cli.rs](../cli.rs/README.md) |
| 错误码格式化 | [error_codes.rs](../error_codes.rs/README.md) |
| 日志记录与错误去重 | [logging.rs](../logging.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
