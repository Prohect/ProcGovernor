# ConfigConstants 结构体 (config.rs)

控制主线程选择算法滞后行为的调优常量。这些值决定线程从主状态晋升和降级的激进程度，防止当线程利用率徘徊在决策边界附近时发生快速振荡。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
```

## 成员

| 成员 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `min_active_streak` | `u8` | `2` | 线程在晋升为主线程之前必须超过 `entry_threshold` 的最小连续轮询迭代次数。较高的值增加稳定性但延迟响应性。 |
| `keep_threshold` | `f64` | `0.69` | 总 CPU 周期份额的分数，当前主线程的份额低于此值时将被降级。必须小于等于 `entry_threshold` 以防止晋升/降级振荡。 |
| `entry_threshold` | `f64` | `0.42` | 总 CPU 周期份额的分数，非主线程必须连续 `min_active_streak` 次迭代超过此值才能晋升为主线程状态。 |

## Default 实现

```rust
impl Default for ConfigConstants {
    fn default() -> Self {
        ConfigConstants {
            min_active_streak: 2,
            keep_threshold: 0.69,
            entry_threshold: 0.42,
        }
    }
}
```

`Default` 实现提供了适用于大多数游戏和实时工作负载的保守值。`keep_threshold`（0.69）和 `entry_threshold`（0.42）之间的差距创建了一个滞后区间，防止线程在接近单一阈值时被反复晋升和降级。

## 备注

### 滞后机制

主线程调度器使用双阈值滞后系统：

1. **进入：** 线程必须在至少 `min_active_streak` 次连续迭代中保持 CPU 周期份额高于 `entry_threshold` 才能晋升。
2. **保持：** 一旦晋升，只要线程的周期份额保持高于 `keep_threshold`，它就保持主线程状态。
3. **降级：** 当主线程的周期份额降至 `keep_threshold` 以下时，它立即被降级。

在默认配置中，`entry_threshold` 低于 `keep_threshold`（其中 `keep_threshold` 是保持线程所需的较高值），这意味着勉强符合主线程资格的线程不会立即面临降级风险。

### 配置语法

常量在配置文件顶部使用 `@NAME = value` 语法定义：

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1
```

解析由 [parse_constant](parse_constant.md) 处理。未知常量名称会产生警告但不会导致解析错误。

### 热重载时传播

当通过 [hotreload_config](hotreload_config.md) 热重载配置文件时，新的 `ConfigConstants` 会被复制到 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 实例中，使更新后的阈值在下次轮询迭代中生效。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用者** | [read_config](read_config.md)（通过 [parse_constant](parse_constant.md) 填充）、[hotreload_config](hotreload_config.md)（传播到调度器） |
| **消费者** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| **依赖项** | 无（纯数据结构） |
| **所需权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| 常量解析器 | [parse_constant](parse_constant.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 线程级配置 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 热重载机制 | [hotreload_config](hotreload_config.md) |
| 配置结果容器 | [ConfigResult](ConfigResult.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
