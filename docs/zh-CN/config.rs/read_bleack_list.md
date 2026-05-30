# read_bleack_list 函数 (config.rs)

读取包含要排除在配置执行之外的进程名称的黑名单文件。文件中的每行指定一个进程名称；以 `#` 开头的行被视为注释并忽略。

> **注意：** 函数名称保留了源代码中出现的原始拼写 `read_bleack_list`。

## 语法

```rust
pub fn read_bleack_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

## 参数

`path: P`

黑名单文件的文件系统路径。接受任何实现 `AsRef<Path>` 的类型，如 `&str`、`String` 或 `PathBuf`。文件预期为 UTF-8 文本文件，每行一个进程名称。

## 返回值

`Result<Vec<String>>`——成功时返回小写进程名称的 `Vec<String>`。失败时（例如文件未找到、权限被拒绝）返回底层的 `std::io::Error`。

## 备注

### 文件格式

黑名单文件使用简单的面向行格式：

```
# 这是一条注释
svchost.exe
explorer.exe
# 另一条注释
taskmgr.exe
```

### 解析规则

1. 文件被打开并使用缓冲读取器逐行读取。
2. 每行被修剪掉前导和尾部空白。
3. 行被转换为小写，以便与运行中的进程名称进行不区分大小写的匹配。
4. 空行和以 `#` 开头的行被丢弃。
5. 所有存活的行被收集到 `Vec<String>` 中。

成功加载后，函数通过 `log!` 宏记录已加载黑名单项目的数量（例如 `"12 blacklist items loaded"`）。

### 与热重载的关系

此函数由 [hotreload_blacklist](hotreload_blacklist.md) 在黑名单文件的修改时间戳发生变化时调用。热重载期间读取失败时，先前的黑名单通过调用点的 `unwrap_or_default()` 保留。

### 对服务行为的影响

其小写名称出现在黑名单中的进程在应用循环期间被完全跳过。即使配置文件中存在匹配的规则，也不会对黑名单中的进程应用进程级或线程级配置。

### 错误处理

如果文件无法打开，函数从 `File::open` 传播 `std::io::Error`。行读取错误导致迭代器在第一次失败时停止（通过 `map_while(Result::ok)`），但在此点之前成功读取的行仍包含在结果中。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用者** | [hotreload_blacklist](hotreload_blacklist.md)、`main.rs` 中的主初始化 |
| **被调用者** | `std::fs::File::open`、`std::io::BufReader`、`log!` 宏 |
| **所需权限** | 黑名单路径的文件读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 黑名单热重载机制 | [hotreload_blacklist](hotreload_blacklist.md) |
| 配置文件读取器（对应物） | [read_config](read_config.md) |
| 指定黑名单路径的 CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
