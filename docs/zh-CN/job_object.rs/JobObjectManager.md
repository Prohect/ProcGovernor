# JobObjectManager 结构体 (job_object.rs)

管理 Windows Job Objects，用于内核强制的 CPU 亲和性限制。Job 使用原始配置规格字符串作为命名对象（例如 `*ecore` → `_ecore`，`0-7` → `0-7`），使其在 Process Explorer 等工具中易于识别。

## 语法

```ProcGovernor/src/job_object.rs#L20-39
pub struct JobObjectManager {
    jobs: HashMap<(String, usize), HANDLE>,
}
```

## 字段

`jobs: HashMap<(String, usize), HANDLE>`

Job Object 句柄的缓存，以 `(spec_string, affinity_mask)` 为键。键中包含掩码，以便配置重载时别名定义的更改可以被检测到并更新现有 Job Object 上的内核强制的亲和性限制。

## 方法

### new

```rust
pub fn new() -> Self
```

创建一个空的 `JobObjectManager`，不包含任何已缓存的 Job 句柄。

---

### get_or_create_job

```rust
fn get_or_create_job(
    &mut self,
    spec: &str,
    cpu_indices: &[u32],
    pid: u32,
    process_name: &str,
    errors: &mut Vec<String>,
) -> Option<HANDLE>
```

返回给定规格字符串 + CPU 掩码的已缓存或新创建的 Job 句柄。

**行为：**

1. 通过 [`cpu_indices_to_mask`](../config.rs/cpu_indices_to_mask.md) 将 `cpu_indices` 转换为亲和性掩码。如果掩码为 0（所有 CPU 均 ≥64），则记录警告且不创建 Job（单组 Job Object API 仅支持 ≤64 逻辑处理器）。
2. 检查缓存中是否有精确的 `(spec, mask)` 命中。如果找到，立即返回已缓存的句柄。
3. 创建或打开命名的 Job Object：`Local\ProcGovernor_Job_{sanitized_spec}`。规格中的 `*` 字符替换为 `_`，因为 `*` 在某些 Windows API 中是通配符。
4. 在 Job 上设置 `JOB_OBJECT_LIMIT_AFFINITY` 以强制执行 CPU 亲和性掩码。
5. 成功时，替换同一规格的旧缓存条目（不同掩码），并缓存新句柄。
6. 失败时，通过 [`is_new_error`](../logging.rs/is_new_error.md) 按唯一的 `(pid, operation, error_code)` 记录一次错误，并关闭句柄。

---

### assign_process

```rust
pub fn assign_process(
    &mut self,
    pid: u32,
    spec: &str,
    cpu_indices: &[u32],
    process_name: &str,
    errors: &mut Vec<String>,
) -> bool
```

将进程分配到由其配置规格字符串标识的 Job Object。

**行为：**

1. 如果 `cpu_indices` 为空（无 Job 亲和性配置），立即返回 `true`。
2. 调用 `get_or_create_job` 获取 Job 句柄。如果失败则返回 `false`。
3. 使用 `PROCESS_SET_QUOTA | PROCESS_TERMINATE`（`AssignProcessToJobObject` 所需）打开目标进程。
4. 调用 `AssignProcessToJobObject` 将进程分配到 Job。
5. 在成功和失败路径上均关闭进程句柄。
6. 成功时返回 `true`，失败时返回 `false`。

**重要：** 一旦进程被分配到某个 Job，便无法重新分配。如果进程已在另一个 Job 中（例如由其父进程启动时已在 Job Object 下），则预期会失败。

## Drop 实现

```rust
impl Drop for JobObjectManager {
    fn drop(&mut self) { ... }
}
```

在关闭时关闭所有已缓存的 Job Object 句柄。由于未使用 `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`，关闭句柄不会终止已分配的进程——它们会保持运行并维持其亲和性限制不变（只要有进程分配到该命名 Job Object，该对象就会保持存在）。

## 需求

| | |
|---|---|
| **模块** | `src/job_object.rs` |
| **调用者** | [`apply_job_object_affinity`](../apply.rs/apply_job_object_affinity.md)、主轮询循环 |
| **被调函数** | [`cpu_indices_to_mask`](../config.rs/cpu_indices_to_mask.md)、[`is_new_error`](../logging.rs/is_new_error.md)、[`log_to_find`](../logging.rs/log_to_find.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | [`CreateJobObjectW`](https://learn.microsoft.com/zh-cn/windows/win32/api/jobapi2/nf-jobapi2-createjobobjectw)、[`SetInformationJobObject`](https://learn.microsoft.com/zh-cn/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject)、[`AssignProcessToJobObject`](https://learn.microsoft.com/zh-cn/windows/win32/api/jobapi2/nf-jobapi2-assignprocesstojobobject)、[`OpenProcess`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)、[`CloseHandle`](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | `PROCESS_SET_QUOTA`、`PROCESS_TERMINATE`（用于打开进程）；建议使用管理员权限 |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [apply_job_object_affinity](../apply.rs/apply_job_object_affinity.md) | 调用 `assign_process` 的应用函数 |
| [apply_affinity](../apply.rs/apply_affinity.md) | 软性每进程 CPU 亲和性（对比） |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `job_object_affinity_spec` 和 `job_object_affinity_cpus` 的配置结构体 |
| [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) | CPU 索引列表 → 亲和性位掩码转换 |
| [Operation](../logging.rs/Operation.md) | 错误操作码：`CreateJobObject`、`SetInformationJobObject`、`AssignProcessToJobObject`、`OpenProcessForJobAssignment` |
