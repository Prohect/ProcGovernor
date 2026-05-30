# hotreload_blacklist 函数 (config.rs)

检查磁盘上的黑名单文件是否已被修改，如果修改时间戳发生变化则重新加载它。此函数在每次轮询迭代中调用，支持在不重启服务的情况下实时更新进程排除列表。

## 语法

```rust
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## 参数

`cli: &CliArgs`

对已解析命令行参数的引用。`blacklist_file_name` 字段（`Option<String>`）指定黑名单文件的路径。如果为 `None`，函数立即返回而不执行任何操作。

`blacklist: &mut Vec<String>`

**\[入参, 出参\]** 当前内存中的小写进程名称黑名单。成功重载时，此向量被替换为新解析的内容。如果黑名单文件变得不可访问（删除、移动或权限更改），该向量被清空。

`last_blacklist_mod_time: &mut Option<std::time::SystemTime>`

**\[入参, 出参\]** 跟踪黑名单文件的最后已知修改时间戳。用于检测更改：

- `None` 表示之前没有成功的读取（初始状态或文件变得不可访问）。
- `Some(time)` 保存最近一次成功重载的 `modified()` 时间戳。

函数将文件的当前修改时间与此存储值进行比较。如果不同，触发重载并更新此值。如果文件变得不可访问，此值被重置为 `None`。

## 返回值

此函数不返回值。副作用通过 `blacklist` 和 `last_blacklist_mod_time` 出参传达。

## 备注

### 重载逻辑

函数遵循以下决策树：

1. **未配置黑名单文件：** 如果 `cli.blacklist_file_name` 为 `None`，函数立即返回。
2. **文件不可访问：** 如果对黑名单路径的 `std::fs::metadata()` 调用失败：
   - 如果之前记录了修改时间（`last_blacklist_mod_time.is_some()`），黑名单被清空，时间戳被重置为 `None`。
   - 如果之前没有时间戳，不执行任何操作（避免在启动时没有文件存在的情况下重复日志记录）。
3. **文件未更改：** 如果文件的 `modified()` 时间戳与 `*last_blacklist_mod_time` 匹配，函数返回而不重载。
4. **文件已更改：** 如果时间戳不同，文件通过 [read_bleack_list](read_bleack_list.md) 重载。成功时，`*blacklist` 被替换，`*last_blacklist_mod_time` 被更新。失败时，保留之前的黑名单和时间戳。

### 文件消失处理

当先前可访问的黑名单文件变得不可访问时（例如被用户删除），函数主动清空内存中的黑名单。这确保先前被黑名单排除的进程不再被阻止。发出日志消息：`"Blacklist file '{path}' no longer accessible, clearing blacklist."`。

### 日志记录

- 重载时：`"Blacklist file '{path}' changed, reloading..."`
- 重载后：`"Blacklist reload complete: {N} items loaded."`
- 文件消失时：`"Blacklist file '{path}' no longer accessible, clearing blacklist."`

### 轮询频率

此函数在每次主服务轮询迭代中调用一次。开销极小——文件未更改时每次迭代仅一次 `metadata()` 系统调用。除非修改时间戳不同，否则不发生文件 I/O。

### 线程安全性

此函数不是线程安全的。它设计为从单线程主轮询循环中调用。对 `blacklist` 和 `last_blacklist_mod_time` 的可变引用在编译时强制执行独占访问。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用者** | `main.rs` 中的主轮询循环 |
| **被调用者** | [read_bleack_list](read_bleack_list.md)、`std::fs::metadata` |
| **依赖项** | [CliArgs](../cli.rs/CliArgs.md) |
| **所需权限** | 黑名单文件的文件系统读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 黑名单文件读取器 | [read_bleack_list](read_bleack_list.md) |
| 配置热重载对应物 | [hotreload_config](hotreload_config.md) |
| CLI 参数结构体 | [CliArgs](../cli.rs/CliArgs.md) |
| 主服务循环 | [main.rs](../main.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
