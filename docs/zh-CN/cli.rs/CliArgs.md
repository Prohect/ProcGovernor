# CliArgs 结构体 (cli.rs)

保存所有解析后的命令行参数，用于控制 ProcGovernor 的运行时行为。单个实例在启动时通过 `new()` 创建，由 [parse_args](parse_args.md) 填充，然后在整个应用程序生命周期中以引用方式传递。

## 语法

```rust
#[derive(Debug, Default)]
pub struct CliArgs {
    pub interval_ms: u32,
    pub help_mode: bool,
    pub help_all_mode: bool,
    pub convert_mode: bool,
    pub autogroup_mode: bool,
    pub find_mode: bool,
    pub validate_mode: bool,
    pub process_logs_mode: bool,
    pub dry_run: bool,
    pub config_file_name: String,
    pub blacklist_file_name: Option<String>,
    pub in_file_name: Option<String>,
    pub out_file_name: Option<String>,
    pub no_uac: bool,
    pub loop_count: Option<u32>,
    pub time_resolution: u32,
    pub log_loop: bool,
    pub skip_log_before_elevation: bool,
    pub no_debug_priv: bool,
    pub no_inc_base_priority: bool,
    pub no_etw: bool,
    pub continuous_process_level_apply: bool,
}
```

## 成员

| 成员 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `interval_ms` | `u32` | `5000` | 应用周期之间的轮询间隔（毫秒）。解析时限制在 \[16, 86 400 000\] 范围内。 |
| `help_mode` | `bool` | `false` | 当为 `true` 时，通过 [print_help](print_help.md) 打印基本使用帮助并退出。由 `-help`、`--help`、`-?`、`/?` 或 `?` 触发。 |
| `help_all_mode` | `bool` | `false` | 当为 `true` 时，通过 [print_help_all](print_help_all.md) 打印完整帮助（CLI 选项 + 配置格式）并退出。由 `-helpall` 或 `--helpall` 触发。 |
| `convert_mode` | `bool` | `false` | 激活 Process Lasso 配置转换器模式。需要 `-in` 和 `-out` 文件参数。 |
| `autogroup_mode` | `bool` | `false` | 激活将具有相同设置的规则自动分组到命名组块中。需要 `-in` 和 `-out` 文件参数。 |
| `find_mode` | `bool` | `false` | 查找 CPU 亲和性与系统默认值匹配的进程（即未被管理的进程）。可通过 `-blacklist` 文件进行过滤。 |
| `validate_mode` | `bool` | `false` | 验证配置文件是否存在语法错误和未定义的别名，然后退出。隐式启用控制台输出。 |
| `process_logs_mode` | `bool` | `false` | 处理来自 `-find` 模式的日志文件，以发现新进程及其可执行文件路径。使用 `-config`、`-blacklist`、`-in`（日志目录）和 `-out`（结果文件）。 |
| `dry_run` | `bool` | `false` | 模拟应用周期，不调用任何修改进程或线程状态的 Windows API。更改将如同已应用一样被记录。 |
| `config_file_name` | `String` | `"config.ini"` | 配置文件的路径。可通过 `-config <file>` 参数覆盖。 |
| `blacklist_file_name` | `Option<String>` | `None` | 黑名单文件的可选路径，由 `-find` 和 `-processlogs` 模式用于排除已知进程。 |
| `in_file_name` | `Option<String>` | `None` | `-convert` 模式的输入文件路径，或 `-processlogs` 模式的输入日志目录。 |
| `out_file_name` | `Option<String>` | `None` | `-convert`、`-autogroup` 和 `-processlogs` 模式的输出文件路径。 |
| `no_uac` | `bool` | `false` | 跳过启动时的自动 UAC 提权请求。适用于无需管理员权限的调试场景。 |
| `loop_count` | `Option<u32>` | `None` | 当设置时，将服务限制为有限次数的轮询迭代。最小值为 1。主要用于测试。当为 `None` 时，服务无限期运行。 |
| `time_resolution` | `u32` | `0` | 以 100 纳秒为单位的 Windows 计时器分辨率（例如 `5210` = 0.5210 ms）。值为 `0` 表示不修改系统计时器分辨率。 |
| `log_loop` | `bool` | `false` | 在每个轮询迭代开始时记录一条消息。在调试时有助于验证循环时序。 |
| `skip_log_before_elevation` | `bool` | `false` | 在 UAC 提权完成之前抑制所有日志输出。防止进程以管理员身份重新启动时产生重复的日志条目。 |
| `no_debug_priv` | `bool` | `false` | 跳过在启动时请求 `SeDebugPrivilege`。没有此权限，服务无法打开系统进程的句柄。 |
| `no_inc_base_priority` | `bool` | `false` | 跳过在启动时请求 `SeIncreaseBasePriorityPrivilege`。没有此权限，将进程设置为 `High` 或 `Realtime` 优先级可能会失败。 |
| `no_etw` | `bool` | `false` | 禁用 ETW（Windows 事件跟踪）进程启动监控。禁用后，新启动的进程仅在下一个轮询间隔内被检测到，而不是实时检测。 |
| `continuous_process_level_apply` | `bool` | `false` | 当为 `true` 时，进程级设置（优先级、亲和性、CPU 集合、IO 优先级、内存优先级）在每个轮询迭代中重新应用，而不是每个 PID 只应用一次。适用于外部工具可能重置进程属性的场景。 |

## 方法

### new

```rust
pub fn new() -> Self
```

创建一个具有默认值的 `CliArgs`。将 `interval_ms` 设置为 `5000`，`config_file_name` 设置为 `"config.ini"`；所有其他字段使用其 `Default` trait 值（`false`、`None`、`0` 或空字符串）。

**返回值**

一个具有默认配置的新 `CliArgs` 实例。

## 备注

- 该结构体同时派生 `Debug` 和 `Default`。手动的 `new()` 构造函数覆盖了 `Default` 实现中的两个字段；所有其他字段委托给 `..Default::default()`。
- `CliArgs` 在 `main` 中创建一次，并通过共享引用（`&CliArgs`）传递给轮询循环、配置热重载和应用函数。在 [parse_args](parse_args.md) 完成后，它永远不会被修改。
- 布尔模式标志（`convert_mode`、`find_mode`、`validate_mode` 等）按约定是互斥的，但在类型级别上没有强制执行。如果设置了多个模式，`main` 中第一个检查的模式优先。

## 需求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用者** | `main`、[parse_args](parse_args.md)、配置热重载、应用循环、[hotreload_config](../config.rs/hotreload_config.md)、[hotreload_blacklist](../config.rs/hotreload_blacklist.md) |
| **依赖** | 无（纯数据结构体） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [cli.rs](README.md) |
| 参数解析器 | [parse_args](parse_args.md) |
| 基本帮助输出 | [print_help](print_help.md) |
| 完整帮助输出 | [print_help_all](print_help_all.md) |
| 配置文件类型 | [ConfigResult](../config.rs/ConfigResult.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
