# error_codes 模块 (ProcGovernor)

`error_codes` 模块为 Win32 和 NTSTATUS 错误码提供人类可读的错误消息查找。应用程序其余部分不在日志输出中直接显示原始数值码，而是调用这些函数生成熟悉的符号名称（例如 `ACCESS_DENIED`、`STATUS_INVALID_HANDLE`），与 Windows SDK 头文件中定义的常量保持一致。未知代码则回退到十六进制格式化字符串。

## 函数

| 名称 | 描述 |
|------|-------------|
| [error_from_code_win32](error_from_code_win32.md) | 将 Win32 `u32` 错误码映射为其符号名称（例如 `5` → `"ACCESS_DENIED"`）。对于未映射的代码，返回 `"WIN32_ERROR_CODE_0x{code:08X}"`。 |
| [error_from_ntstatus](error_from_ntstatus.md) | 将 NTSTATUS `i32` 状态码映射为其符号名称（例如 `0xC0000022` → `"STATUS_ACCESS_DENIED"`）。对于未映射的代码，返回 `"NTSTATUS_0x{code:08X}"`。 |

## 备注

两个函数均使用 `match` 语句匹配精心选取的错误码子集，这些错误码在操作进程句柄、线程优先级、亲和性掩码以及其他 Windows 资源管理 API 时经常出现。该子集有意保持精简——仅包含在 ProcGovernor 正常运行或调试过程中已观测到的代码。这既保持了查找速度（编译器生成的跳转表），也避免了引入完整的 Windows SDK 错误目录。

这些函数是纯函数——不执行 I/O、不持有状态，可从任意上下文安全调用。

## 另请参阅

| 主题 | 链接 |
|-------|------|
| apply 模块中的错误去重 | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| apply 模块（主要消费者） | [apply.rs](../apply.rs/README.md) |
| Windows API 封装 | [winapi.rs](../winapi.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
