# apply_job_object_affinity 函数 (apply.rs)

应用内核强制的 Job Object CPU 亲和性限制。与使用每进程 `SetProcessAffinityMask` 的 `apply_affinity` 不同，Job Object 可防止进程及其子进程在指定掩码之外的 CPU 上运行。

## 语法

```ProcGovernor/src/apply.rs#L134-160
pub fn apply_job_object_affinity(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    job_manager: &mut JobObjectManager,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

包含 `job_object_affinity_cpus`（要强制执行的 CPU 索引）和 `job_object_affinity_spec`（用于命名 Job Object 的原始规格字符串）的进程级配置。如果 `job_object_affinity_cpus` 为空，函数将立即返回，不执行任何操作。

`dry_run: bool`

若为 `true`，则在 *apply_config_result* 中记录预定的变更，而不调用任何 Windows API 或修改 Job Object。

`job_manager: &mut JobObjectManager`

缓存并管理命名 Windows Job Objects 的 Job Object 管理器。参见 [`JobObjectManager`](../job_object.rs/JobObjectManager.md)。

`apply_config_result: &mut ApplyConfigResult`

操作过程中产生的变更消息和错误的累加器。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 传递。

## 备注

此函数委托给 [`JobObjectManager::assign_process`](../job_object.rs/JobObjectManager.md#assign_process)，后者处理 Job Object 的创建、缓存和进程分配。Job Object 使用原始配置规格命名（例如 `*ecore` → `Local\ProcGovernor_Job__ecore`），使其在 Process Explorer 等工具中易于识别。

### 副作用

- **创建命名 Job Object**：在首次使用给定亲和性规格时创建。
- **将进程分配到 Job**：通过 `AssignProcessToJobObject` 完成。一旦分配，进程无法重新分配到其他 Job。
- **在配置重载时更新 Job 的亲和性限制**：如果 CPU 掩码发生变化（例如别名重新定义）。

### 错误处理

来自 Job Object 创建、亲和性限制配置、进程句柄打开和进程分配的错误会推送到 `apply_config_result.errors` 中。每个错误通过 [`is_new_error`](../logging.rs/is_new_error.md) 按唯一的 `(pid, operation, error_code)` 组合去重记录。

分配进程到 Job 失败（例如因为进程已在另一个 Job 中）不会阻止其他规则字段的应用。

### 变更消息格式

```/dev/null/example.txt#L1
Job Affinity: -> [0, 1, 2, 3]
```

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用者** | 主轮询循环中的 [`apply_process_level`](../main.rs/apply_process_level.md) |
| **被调函数** | [`JobObjectManager::assign_process`](../job_object.rs/JobObjectManager.md)、[`format_cpu_indices`](../config.rs/format_cpu_indices.md) |
| **Win32 API** | （委托给 `JobObjectManager`） |
| **权限** | 管理员权限（`AssignProcessToJobObject` 需要；建议使用提升的权限） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [JobObjectManager](../job_object.rs/JobObjectManager.md) | 创建、缓存和分配 Job Object 的 Job Object 管理器 |
| [apply_affinity](apply_affinity.md) | 软性每进程 CPU 亲和性（与内核强制的 Job 亲和性对比） |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `job_object_affinity_spec` 和 `job_object_affinity_cpus` 的配置结构体 |
| [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) | 将 CPU 索引列表转换为亲和性位掩码 |
| [is_new_error](../logging.rs/is_new_error.md) | Job Object 分配使用的错误去重机制 |
