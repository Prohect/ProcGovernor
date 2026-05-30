# apply_affinity 函数 (apply.rs)

为进程设置 CPU 亲和性掩码，将其限制为只能在指定的逻辑处理器上运行。

## 语法

```ProcGovernor/src/apply.rs#L134-142
pub fn apply_affinity<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

包含 `affinity_cpus` 的进程级配置，即进程应被限制运行的 CPU 索引列表。如果 `affinity_cpus` 为空，函数将立即返回，不执行任何操作。

`dry_run: bool`

若为 `true`，则在 *apply_config_result* 中记录预定的变更，而不调用任何 Windows API。

`current_mask: &mut usize`

**\[out\]** 在成功查询时接收进程当前的亲和性掩码。在成功设置时更新为新的掩码。此值被下游函数（如 [apply_prime_threads_promote](apply_prime_threads_promote.md)）使用，后者用它来将主 CPU 索引过滤为仅在进程亲和性范围内的索引。

`process_handle: &ProcessHandle`

目标进程的句柄包装器。需要读句柄和写句柄；若任一不可用，函数将提前返回。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

目标进程线程映射的惰性访问器。当亲和性成功更改后，传递给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。

`apply_config_result: &mut ApplyConfigResult`

操作过程中产生的变更消息和错误的累加器。

## 返回值

此函数不返回值。结果通过 `current_mask`（副作用）和 `apply_config_result` 传递。

## 备注

该函数通过 `cpu_indices_to_mask` 将配置的 CPU 索引转换为位掩码。然后使用 **GetProcessAffinityMask** 查询当前进程的亲和性掩码，并与目标值进行比较。若不同，则调用 **SetProcessAffinityMask** 应用新掩码。

### 副作用

- **填充 `current_mask`：** 即使配置的亲和性已匹配，当前掩码也会通过 `GetProcessAffinityMask` 的输出参数写入 `*current_mask`。此值被后续的主线程逻辑使用。
- **重置线程理想处理器：** 在成功更改亲和性后，函数立即使用 `config.affinity_cpus` 调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。这会在新的 CPU 集合上重新分配线程理想处理器，以防止出现将线程聚集在不再处于亲和性掩码中的 CPU 上的过时分配。
- **将 `current_mask` 更新为新值：** 在成功设置后，`*current_mask` 被覆盖为新的亲和性掩码。

### 错误处理

- 如果 **GetProcessAffinityMask** 失败，错误将通过 [log_error_if_new](log_error_if_new.md) 按 `(pid, operation, error_code)` 去重记录一次，函数将退出而不尝试设置。
- 如果 **SetProcessAffinityMask** 失败，错误同样会被记录，且掩码不会被更新。
- 在 `dry_run` 模式下两个错误路径均被抑制（获取路径的错误完全跳过）。

### 亲和性掩码格式

亲和性掩码是一个 `usize` 位掩码，其中位 *N* 表示逻辑处理器 *N*。例如，CPU `[0, 2, 4]` 生成的掩码为 `0x15`。转换后的零掩码会导致函数跳过设置操作，因为 Windows 拒绝零亲和性掩码。

### 变更消息格式

```/dev/null/example.txt#L1
Affinity: 0xFF -> 0x15
```

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用者** | 主轮询循环（通过进程级配置应用） |
| **被调函数** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md)、`cpu_indices_to_mask` |
| **Win32 API** | [GetProcessAffinityMask](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask)、[SetProcessAffinityMask](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask) |
| **权限** | `PROCESS_QUERY_LIMITED_INFORMATION`（读）、`PROCESS_SET_INFORMATION`（写） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | 变更和错误的累加器 |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | 通过 CPU 集合实现的软性 CPU 偏好（硬亲和性的替代方案） |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | 在亲和性更改后重新分配线程理想处理器 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 使用 `current_mask` 过滤主 CPU 索引 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `affinity_cpus` 的配置结构体 |
| [ProcessHandle](../winapi.rs/ProcessHandle.md) | 具有读/写访问级别的进程句柄包装器 |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
