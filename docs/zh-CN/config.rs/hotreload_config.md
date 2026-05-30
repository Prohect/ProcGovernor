# hotreload_config 函数 (config.rs)

检查磁盘上的配置文件是否已被修改，如果已修改则重新加载它。在成功重载且无解析错误的情况下，活跃配置被替换，主线程调度器的常量被更新，所有进程级应用状态被重置，以便根据运行中的进程重新评估规则。

## 语法

```rust
pub fn hotreload_config(
    cli: &CliArgs,
    configs: &mut ConfigResult,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut List<[u32; PIDS]>,
    full_process_level_match: &mut bool,
)
```

## 参数

`cli: &CliArgs`

对已解析命令行参数的引用。`config_file_name` 字段提供要监控的配置文件路径。

`configs: &mut ConfigResult`

**\[入参, 出参\]** 当前活跃的配置。如果重载成功且无错误，则原地替换为新解析的 [ConfigResult](ConfigResult.md)。如果发现错误，此值保持不变，之前的配置继续生效。

`last_config_mod_time: &mut Option<std::time::SystemTime>`

**\[入参, 出参\]** 跟踪配置文件的最后已知修改时间戳。每次调用时，函数查询文件的元数据并将修改时间与此值进行比较。如果不同（或此值为 `None`），触发重载并存储新的时间戳。调用者在轮询迭代之间持久化此值。

`prime_core_scheduler: &mut PrimeThreadScheduler`

**\[入参, 出参\]** 主线程调度器实例。成功重载时，调度器的 `constants` 字段被替换为新解析配置中的 [ConfigConstants](ConfigConstants.md)，使更新后的滞后阈值立即生效。

`process_level_applied: &mut List<[u32; PIDS]>`

**\[入参, 出参\]** 已应用进程级规则的进程 ID 列表。成功重载时被清空，以便在下一个轮询周期中根据新规则重新评估所有运行中的进程。

`full_process_level_match: &mut bool`

**\[入参, 出参\]** 指示是否所有当前运行中的进程都已根据进程级规则匹配的标志。成功重载时重置为 `true`，通知主循环执行完整的匹配遍历。

## 返回值

此函数不返回值。结果通过可变引用参数传达。

## 备注

### 重载决策逻辑

函数使用两步保护：

1. 查询配置文件路径的 `std::fs::metadata`。如果 metadata 调用失败（例如文件被删除、权限被拒绝），不执行任何操作，之前的配置保持活跃。
2. 将 `metadata.modified()` 与 `*last_config_mod_time` 比较。如果时间戳相等，文件未更改，函数立即返回。如果不同（包括 `last_config_mod_time` 为 `None` 的初始情况），启动重载。

这意味着函数在每次轮询迭代中调用是安全的，开销可忽略不计——文件未更改时仅执行一次 `metadata()` 系统调用。

### 重载过程

当检测到修改时：

1. `*last_config_mod_time` 被更新为新的修改时间。
2. 发出日志消息：`"Configuration file '{path}' changed, reloading..."`。
3. 调用 [read_config](read_config.md) 从头解析文件。
4. 通过 `errors.is_empty()` 检查新的 [ConfigResult](ConfigResult.md)：
   - **无错误：** 活跃的 `configs` 被替换，调用 [print_report](ConfigResult.md)，调度器的常量被更新，`process_level_applied` 被清空，`full_process_level_match` 设为 `true`，完成消息记录总规则数。
   - **存在错误：** 之前的 `configs` 被保留，记录错误消息，打印每条单独错误。服务继续使用旧配置运行。

### 成功重载时的状态重置

清空 `process_level_applied` 强制主轮询循环对所有当前运行中的进程重新应用进程级设置（优先级、亲和性、CPU 集、IO 优先级、内存优先级），即使这些进程已在旧规则下配置过。这确保规则更改（例如将进程从 P 核心移到 E 核心）无需重启进程即可生效。

将 `full_process_level_match` 设为 `true` 告诉主循环对所有运行中的进程执行完整扫描，而非仅扫描新发现的进程。

### 线程安全性

此函数不是线程安全的，只能从主轮询循环调用。所有可变参数都是独占引用，由 Rust 的借用检查器强制执行。

### 错误韧性

函数遵循故障安全模式：格式错误的配置文件永远不会中断运行中的服务。旧配置继续应用，直到保存了有效的配置文件。这允许用户在不冒未配置操作期间风险的情况下在服务运行时编辑配置文件。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用者** | `src/main.rs` 中的主轮询循环 |
| **被调用者** | [read_config](read_config.md)、[ConfigResult::print_report](ConfigResult.md)、`std::fs::metadata` |
| **依赖项** | [CliArgs](../cli.rs/CliArgs.md)、[ConfigResult](ConfigResult.md)、[ConfigConstants](ConfigConstants.md)、[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)、[List](../collections.rs/README.md) |
| **所需权限** | 配置文件路径的文件系统读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 黑名单热重载对应物 | [hotreload_blacklist](hotreload_blacklist.md) |
| 配置文件解析器 | [read_config](read_config.md) |
| 解析结果容器 | [ConfigResult](ConfigResult.md) |
| 传播到调度器的调优常量 | [ConfigConstants](ConfigConstants.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
