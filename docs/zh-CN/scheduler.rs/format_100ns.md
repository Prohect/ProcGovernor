# format_100ns 函数 (scheduler.rs)

将以 100 纳秒为单位的时间值格式化为人类可读的 `"秒.毫秒 s"` 字符串。用于进程退出报告中以显示来自 `SYSTEM_THREAD_INFORMATION` 的内核时间、用户时间和其他时间字段。

## 语法

```rust
fn format_100ns(time: i64) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `time` | `i64` | 以 100 纳秒（100ns）为单位的时间长度，如 Windows 内核结构体（如 `SYSTEM_THREAD_INFORMATION.KernelTime` 和 `SYSTEM_THREAD_INFORMATION.UserTime`）所报告的。 |

## 返回值

`String` — 格式为 `"{秒}.{毫秒:03} s"` 的格式化字符串，其中毫秒部分用零填充到三位。

### 示例

| 输入（100ns 单位） | 输出 |
|-------------------|------|
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `10_000` | `"0.001 s"` |
| `99_999` | `"0.009 s"` |

## 备注

- 转换仅使用整数运算——无浮点舍入：
  - 秒：`time / 10_000_000`
  - 毫秒：`(time % 10_000_000) / 10_000`
- 亚毫秒精度（剩余的微秒和 100ns 分量）被截断，而非四舍五入。
- 此函数是模块私有的（`fn`，而非 `pub fn`），并且仅在 [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) 生成线程诊断报告时被调用。
- `" s"` 后缀始终附加，以将输出与日志输出中的原始数字值区分开来。

### 单位参考

| Windows 单位 | = SI 等效值 |
|---|---|
| 1 tick | 100 纳秒 |
| 10,000 ticks | 1 毫秒 |
| 10,000,000 ticks | 1 秒 |

## 需求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用者** | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) |
| **依赖** | 无（纯格式化函数） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| FILETIME 格式化器 | [format_filetime](format_filetime.md) |
| 进程退出报告 | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) |
| 线程统计容器 | [ThreadStats](ThreadStats.md) |
| 模块概述 | [scheduler.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
