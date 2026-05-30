# get_log_path 函数 (logging.rs)

在 `logs/` 目录下构建带日期戳的日志文件路径，如果目录不存在则创建。

## 语法

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

## 参数

`suffix: &str`

插入到文件名中日期戳和 `.log` 扩展名之间的字符串。对于主日志文件传递 `""`，对于查找模式日志文件传递 `".find"`。

| 后缀值 | 生成的文件名 |
|---|---|
| `""` | `logs/YYYYMMDD.log` |
| `".find"` | `logs/YYYYMMDD.find.log` |

## 返回值

`PathBuf` —— 日志文件的完整构造路径，相对于服务的工作目录。路径遵循模式 `logs/YYYYMMDD{suffix}.log`，其中 `YYYYMMDD` 是从 [LOCAL_TIME_BUFFER](README.md) 获取的当前本地日期。

## 备注

- 该函数通过 `get_local_time!()` 宏从 `LOCAL_TIME_BUFFER` 静态变量读取当前日期，然后在执行文件系统操作之前立即释放锁。
- 如果 `logs/` 目录不存在，该函数通过 `std::fs::create_dir_all` 创建它（以及任何必要的父目录）。目录创建失败被静默忽略；调用者在尝试打开文件时会遇到错误。
- 此函数**不**是 `pub` 的 —— 它是模块私有的，仅在 [LOG_FILE](README.md) 和 [FIND_LOG_FILE](README.md) 静态变量的延迟初始化期间调用。
- 因为 `LOG_FILE` 和 `FIND_LOG_FILE` 在每个进程生命周期中仅延迟初始化一次，日志文件日期在首次使用时确定。服务**不会**在午夜时轮换到新日期的文件；需要重新启动才能开始写入新日期的日志文件。

### 日期格式

日期部分使用零填充的四位年份、两位月份和两位日期，无分隔符：`20250115` 表示 2025 年 1 月 15 日。

## 需求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **可见性** | 私有（模块内部） |
| **调用者** | `LOG_FILE` 静态初始化器、`FIND_LOG_FILE` 静态初始化器 |
| **依赖** | `LOCAL_TIME_BUFFER`、`chrono::Datelike`、`std::fs::create_dir_all` |
| **权限** | 对工作目录的文件系统写入权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [logging.rs](README.md) |
| 主日志输出 | [log_message](log_message.md) |
| 查找日志输出 | [log_to_find](log_to_find.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
