# set_timer_resolution 函数 (winapi.rs)

通过未公开的 `NtSetTimerResolution` NT API 将 Windows 全局定时器分辨率设置为用户指定的间隔。这允许服务请求更高频率的系统定时器滴答，从而减少对时间敏感工作负载的调度延迟。

## 语法

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 对已解析的命令行参数的引用。`time_resolution` 字段以 100 纳秒为单位指定所需的定时器分辨率。值为 `0` 时禁用该功能（无操作）。 |

## 返回值

此函数不返回值。成功或失败通过日志输出传达。

## 备注

### 分辨率单位

`cli.time_resolution` 值以 100 纳秒（100ns）为单位表示，与 `NtSetTimerResolution` 使用的原生单位匹配。常见值：

| 所需分辨率 | 值（100ns 单位） | 等效值 |
|----------|---------------|------|
| 0.5 ms | `5000` | 0.5000 ms |
| 1.0 ms | `10000` | 1.0000 ms |
| 15.6 ms（默认） | `156250` | 15.6250 ms |

### 行为

1. 如果 `cli.time_resolution == 0`，函数立即返回，不调用任何 API。
2. 否则，使用 `set_resolution = true` 调用 `NtSetTimerResolution` 以请求指定的间隔。
3. API 在其 `p_current_resolution` 输出参数中返回*先前*（"elder"）的定时器分辨率。
4. 成功时（`NTSTATUS >= 0`），请求的和先前的分辨率都以毫秒为单位记录（值 ÷ 10,000）。
5. 失败时（`NTSTATUS < 0`），NTSTATUS 码以十六进制格式记录。

### 系统范围的影响

`NtSetTimerResolution` 影响全局 Windows 定时器分辨率，而不仅仅是调用进程。系统使用任何进程请求的*最小*（最精确）分辨率。当请求进程退出时，其分辨率请求被自动释放，系统可能恢复到较粗的间隔。

### 日志输出示例

**成功：**
```
Succeed to set timer resolution: 0.5000ms
elder timer resolution: 156250
```

**失败：**
```
Failed to set timer resolution: 0xC0000022
```

### NtSetTimerResolution 签名

```c
NTSTATUS NtSetTimerResolution(
    ULONG   DesiredResolution,  // 以 100ns 为单位
    BOOLEAN SetResolution,      // TRUE 表示设置，FALSE 表示重置
    PULONG  CurrentResolution   // 接收先前的分辨率
);
```

这是一个通过 winapi.rs 中的 `#[link(name = "ntdll")]` FFI 块直接从 `ntdll.dll` 导入的未公开 NT API。

### 与多媒体定时器的关系

这实现了与 `winmm.dll` 中已公开的 `timeBeginPeriod`/`timeEndPeriod` API 相同的效果，但直接使用较低级别的 NT 接口。NT API 提供更精细的粒度（100ns 单位 vs. `timeBeginPeriod` 的 1ms 单位）。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | `src/main.rs` 中的主启动逻辑 |
| **被调用者** | `NtSetTimerResolution`（ntdll.dll FFI） |
| **NT API** | `NtSetTimerResolution`（未公开，通过 `ntdll.dll` 链接） |
| **权限** | 无要求；任何进程都可以请求更高的定时器分辨率 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| CLI 参数解析 | [cli.rs](../cli.rs/README.md) |
| NT FFI 声明 | [winapi.rs](README.md)（外部 FFI 部分） |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
