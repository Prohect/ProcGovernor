# process_logs 函数 (main.rs)

处理查找模式日志文件以发现新的未管理进程。从日志目录中读取所有 `.find.log` 文件，提取唯一的进程名称，过滤掉已存在于配置或黑名单中的进程，并使用 [Everything 搜索](https://www.voidtools.com/) (`es.exe`) 在磁盘上定位可执行文件路径。结果写入文本文件以供手动审查。

## 语法

```rust
fn process_logs(
    configs: &ConfigResult,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
)
```

## 参数

`configs: &ConfigResult`

包含所有进程级和线程级配置映射的已解析 [ConfigResult](../config.rs/ConfigResult.md)。用于确定哪些进程已被管理。

`blacklist: &[String]`

除 `configs` 中已有的进程外，应从结果中排除的小写进程名称列表。

`logs_path: Option<&str>`

包含 `.find.log` 文件的目录路径。当为 `None` 时默认为 `"logs"`。

`output_file: Option<&str>`

写入结果的输出文本文件路径。当为 `None` 时默认为 `"new_processes_results.txt"`。

## 返回值

此函数不返回值。输出写入 `output_file` 指定的文件。

## 备注

该函数实现了一个多阶段管道：

1. **日志扫描** — 遍历 `logs_path` 中名称以 `.find.log` 结尾的所有文件。对于每个文件，解析行以查找模式 `"find <process_name>"` 并提取以 `.exe` 结尾的进程名称。所有名称被转换为小写并收集到 `HashSet` 中以进行去重。

2. **过滤** — 移除出现在任何等级的 `configs.process_level_configs` 或 `configs.thread_level_configs` 中的进程名称，以及出现在 `blacklist` 中的任何名称。

3. **可执行文件定位** — 对于每个剩余的进程名称，调用 Everything 命令行界面 (`es.exe`) 并传入 `-utf8-bom -r ^<escaped_name>$` 以执行精确文件名的正则搜索。进程名称中的 `.` 字符被转义为 `\.` 以确保正确的正则匹配。

4. **编码处理** — 通过 `GetConsoleOutputCP` 检测控制台输出代码页，并选择合适的编码（代码页 936 使用 GBK，否则使用 `windows-<codepage>`）。`es.exe` 的输出按此编码解码。如果存在 UTF-8 BOM 前缀 (`0xEF 0xBB 0xBF`)，将被去除。

5. **输出格式化** — 每个进程以块的形式写入：
   - `Process: <name>` 标题
   - `Found:` 部分带缩进路径，或 `Not found, result empty` / `Not found, es failed`
   - `---` 分隔符

### 副作用

- 将全局 `use_console` 标志设置为 `true`（强制控制台输出模式）。
- 生成外部 `es.exe` 进程。如果 Everything 搜索未安装或 `es.exe` 不在 `PATH` 上，所有查找将报告失败。

### 编码边界情况

该函数通过使用 `encoding_rs` crate 处理 `es.exe` 输出编码与 Rust UTF-8 字符串之间的不匹配。在中文区域系统上（代码页 936），使用 GBK 解码；在其它区域上，选择合适的 Windows 代码页编码。

## 需求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用者** | [main](main.md)（当 `cli.process_logs_mode` 为 `true` 时） |
| **被调函数** | `std::fs::read_dir`、`std::fs::read_to_string`、`std::fs::write`、`std::process::Command` (`es.exe`)、`GetConsoleOutputCP`、`encoding_rs::Encoding::for_label_no_replacement` |
| **外部工具** | [Everything CLI (`es.exe`)](https://www.voidtools.com/support/everything/command_line_interface/) |
| **权限** | 无超出文件系统读/写访问权限 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 查找模式运行时发现 | [process_find](process_find.md) |
| 配置类型 | [ConfigResult](../config.rs/ConfigResult.md) |
| 入口点和模式分派 | [main](main.md) |
| 日志基础设施 | [logging.rs](../logging.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
