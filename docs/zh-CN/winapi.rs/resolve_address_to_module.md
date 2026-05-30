# resolve_address_to_module 函数 (winapi.rs)

将虚拟内存地址解析为人类可读的带有偏移字符串的模块名称（例如，`"kernel32.dll+0x1A40"`）。使用按 PID 的模块缓存，避免在每次调用时重新枚举已加载的模块。

## 语法

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 应搜索其模块列表的进程标识符。作为 `MODULE_CACHE` 静态的键。 |
| `address` | `usize` | 要解析的虚拟地址。通常是从 [get_thread_start_address](get_thread_start_address.md) 获取的线程 Win32 起始地址。 |

## 返回值

`String` — 三种格式之一，取决于解析结果：

| 条件 | 格式 | 示例 |
|------|------|------|
| `address == 0` | `"0x0"` | `"0x0"` |
| 地址落在已知模块内 | `"{模块名称}+0x{偏移:X}"` | `"game.dll+0x1A40"` |
| 地址不匹配任何模块 | `"0x{地址:X}"` | `"0x7FF612340000"` |

## 备注

### 模块缓存

该函数由静态 `MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` 支持。在给定 PID 的第一次调用时，该函数调用 [enumerate_process_modules](enumerate_process_modules.md) 构建 `(基地址, 大小, 模块名称)` 元组列表，然后存储在缓存中。同一 PID 的后续调用重用缓存数据而不重新枚举。

PID 的缓存条目在以下情况下被移除：

- 进程退出且 [drop_module_cache](drop_module_cache.md) 从 [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) 被调用。

### 地址匹配

该函数对缓存的模块列表执行线性搜索，查找第一个满足 `base <= address < base + size` 的模块。如果找到匹配，模块内的偏移量（`address - base`）被追加到模块名称。如果没有模块跨越该地址，则返回原始十六进制地址。

### 性能

`MODULE_CACHE` mutex 每次调用获取一次。模块列表在执行线性搜索之前从缓存中克隆出来，在（可能较慢的）字符串格式化步骤之前释放锁。这最小化了当多个调用快速连续发生时对锁的争用。

### 零地址快捷方式

地址为 `0` 时立即返回字符串 `"0x0"`，不获取缓存锁。这是起始地址无法查询的线程的常见情况（例如，由于句柄访问权限不足）。

### 使用上下文

此函数从两个主要位置调用：

1. **ThreadStats 自定义 `Debug` 实现** — 解析 `start_address` 以实现人类可读的调试输出。
2. **PrimeThreadScheduler::drop_process_by_pid** — 在配置了 `track_top_x_threads` 时解析退出报告中的线程起始地址。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用者** | [ThreadStats::fmt (Debug)](../scheduler.rs/ThreadStats.md)、[PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) |
| **被调用者** | [enumerate_process_modules](enumerate_process_modules.md)（在缓存未命中时） |
| **Win32 API** | 无直接调用；依赖 [enumerate_process_modules](enumerate_process_modules.md) 获取模块数据 |
| **权限** | 底层模块枚举需要 `PROCESS_QUERY_INFORMATION` 和 `PROCESS_VM_READ` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [winapi.rs](README.md) |
| 模块缓存清理 | [drop_module_cache](drop_module_cache.md) |
| 模块枚举 | [enumerate_process_modules](enumerate_process_modules.md) |
| 线程起始地址检索 | [get_thread_start_address](get_thread_start_address.md) |
| 使用此函数进行 Debug 输出的线程统计 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 进程退出报告 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
