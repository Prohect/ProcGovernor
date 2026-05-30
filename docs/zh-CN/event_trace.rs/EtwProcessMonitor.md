# EtwProcessMonitor 结构体 (event_trace.rs)

管理监控进程启动和停止事件的 ETW（Windows 事件跟踪）实时跟踪会话的生命周期。该监控器订阅 **Microsoft-Windows-Kernel-Process** 提供程序，并通过标准 `mpsc` 通道投递 [EtwProcessEvent](EtwProcessEvent.md) 值。

## 语法

```rust
pub struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `control_handle` | `CONTROLTRACE_HANDLE` | 由 `StartTraceW` 返回的句柄，用于控制（停止）ETW 会话。 |
| `trace_handle` | `PROCESSTRACE_HANDLE` | 由 `OpenTraceW` 返回的句柄，用于关闭跟踪消费者并解除处理线程的阻塞。 |
| `properties_buf` | `Vec<u8>` | 堆分配缓冲区，为 `EVENT_TRACE_PROPERTIES` 结构和附加的会话名称提供存储。需要在会话持续期间保持有效，因为 `ControlTraceW` 在停止时需要该缓冲区。 |
| `process_thread` | `Option<thread::JoinHandle<()>>` | 运行 `ProcessTrace` 的后台线程的 Join 句柄。在 `stop()` 中等待线程结束后设置为 `None`。 |

## 方法

### start

```rust
pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>
```

创建并启动新的 ETW 实时跟踪会话以监控进程事件。

**返回值**

`Result<(EtwProcessMonitor, Receiver<EtwProcessEvent>), String>` — 成功时返回监控器实例和一个产出 [EtwProcessEvent](EtwProcessEvent.md) 值的通道接收方。失败时返回人类可读的错误字符串。

**备注**

启动过程按顺序执行六个步骤：

1. **创建通道** — 分配 `mpsc` 通道，并将发送方安装到全局 [ETW_SENDER](README.md) 静态变量中，以便操作系统回调能够访问它。
2. **准备 `EVENT_TRACE_PROPERTIES`** — 分配足够大的缓冲区以容纳属性结构体加上 UTF-16 会话名称 `"ProcGovernor_EtwProcessMonitor"`。配置为实时模式并使用 QPC 时间戳。
3. **停止已有会话** — 调用 `stop_existing_session` 清理任何具有相同名称的残留会话（例如上次崩溃留下的）。
4. **启动跟踪** — 调用 `StartTraceW` 创建 ETW 会话并获取控制句柄。
5. **启用提供程序** — 使用 Microsoft-Windows-Kernel-Process 提供程序 GUID（`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`）和 `WINEVENT_KEYWORD_PROCESS`（`0x10`）关键词，在 `TRACE_LEVEL_INFORMATION` 级别调用 `EnableTraceEx2`。
6. **打开并处理跟踪** — 使用 `PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD` 和 [etw_event_callback](README.md) 函数指针调用 `OpenTraceW`。生成名为 `"etw-process-trace"` 的后台线程调用 `ProcessTrace`，该调用将阻塞直到跟踪被关闭。

如果任一步骤失败，在返回错误之前会释放所有此前已获取的资源（停止会话、清除发送方）。

### stop

```rust
pub fn stop(&mut self)
```

停止 ETW 跟踪会话并释放所有资源。

**备注**

关闭序列如下：

1. 检查并清除全局 `ETW_ACTIVE` 标志。如果已非活动，则立即返回（幂等操作）。
2. 对 `trace_handle` 调用 `CloseTrace` 以解除后台线程中 `ProcessTrace` 调用的阻塞。
3. 使用 `EVENT_TRACE_CONTROL_STOP` 调用 `ControlTraceW` 以终止 ETW 会话。
4. 等待后台处理线程结束。
5. 清除全局 `ETW_SENDER` 以丢弃通道发送方，使消费者观察到挂断信号。

### stop_existing_session

```rust
fn stop_existing_session(wide_name: &[u16])
```

尝试停止任何具有给定名称的已有 ETW 会话。这是一个不需要 `EtwProcessMonitor` 实例的静态辅助函数。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `wide_name` | `&[u16]` | 要停止的以空字符结尾的 UTF-16 会话名称。 |

**备注**

分配一个临时的 `EVENT_TRACE_PROPERTIES` 缓冲区，并使用 `EVENT_TRACE_CONTROL_STOP` 调用 `ControlTraceW`。错误会被静默忽略，因为会话可能并不存在。

## Trait 实现

### Drop

```rust
impl Drop for EtwProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
```

`Drop` 实现调用 `stop()`，确保监控器超出作用域时 ETW 会话始终被清理，即使在提前返回或 panic 的情况下也是如此。

## 备注

- 同一时间只能有一个 `EtwProcessMonitor` 处于活动状态，因为回调通过单个全局发送方（`ETW_SENDER`）进行通信。启动第二个监控器会替换发送方，使第一个监控器的接收方成为孤儿。
- 会话名称 `"ProcGovernor_EtwProcessMonitor"` 是固定常量。如果服务崩溃而未调用 `stop()`，残留会话将保留在内核中，直到下次调用 `start()` 时由 `stop_existing_session` 进行清理。
- `ProcessTrace` 是一个阻塞的 Win32 调用，仅在跟踪句柄被关闭时返回。因此它在专用后台线程而非主线程上运行。
- `properties_buf` 必须在整个会话生命期内保持有效并位于同一地址，因为 `ControlTraceW` 在停止时会写回同一缓冲区。

## 需求

| | |
|---|---|
| **模块** | `src/event_trace.rs` |
| **调用方** | 主服务循环（`src/main.rs`）、调度器（`src/scheduler.rs`） |
| **被调用方** | [etw_event_callback](README.md)（注册为操作系统回调） |
| **Win32 API** | [StartTraceW](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-starttracew)、[EnableTraceEx2](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-enabletraceex2)、[OpenTraceW](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-opentracew)、[ProcessTrace](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-processtrace)、[CloseTrace](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-closetrace)、[ControlTraceW](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-controltracew) |
| **权限** | 管理员权限或 `SeSystemProfilePrivilege`（ETW 内核级提供程序所需） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [event_trace.rs](README.md) |
| 事件负载 | [EtwProcessEvent](EtwProcessEvent.md) |
| 错误码辅助函数 | [error_from_code_win32](../error_codes.rs/README.md) |
| 调度器集成 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
