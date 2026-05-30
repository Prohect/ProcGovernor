# collections 模块 (ProcGovernor)

`collections` 模块定义了贯穿整个项目使用的面向性能的类型别名和预调优容量常量。通过集中管理这些定义，每个模块共享相同的高性能哈希映射、哈希集合和小型向量实现，而无需重复 crate 级别的导入。

## 类型别名

| 名称 | 底层类型 | 描述 |
|------|----------------|-------------|
| `HashMap<K, V>` | `FxHashMap<K, V>` ([rustc-hash](https://crates.io/crates/rustc-hash)) | `std::collections::HashMap` 的替代品，使用 Fx 哈希算法。对于进程/线程 ID 查找中常见的小整数或指针大小键，速度显著更快。不具备密码学安全性——这是可接受的，因为所有键均来自可信的本地系统数据。 |
| `HashSet<V>` | `FxHashSet<V>` ([rustc-hash](https://crates.io/crates/rustc-hash)) | `std::collections::HashSet` 的替代品，由相同的 Fx 哈希算法支持。用于 PID、TID 的去重以及错误跟踪集合。 |
| `List<E>` | `SmallVec<E>` ([smallvec](https://crates.io/crates/smallvec)) | 栈分配向量，仅当元素数量超过类型参数 `E` 中编码的内联容量时才会溢出到堆（例如，`[u32; 32]` 可在栈上存储最多 32 个 `u32` 值）。消除了常见小型 CPU 索引列表和线程 ID 向量的堆分配开销。 |

## 宏重导出

| 名称 | 底层宏 | 描述 |
|------|-----------------|-------------|
| `list!` | `smallvec::smallvec!` | 构造包含字面值的 `List<E>` 实例的便捷宏，类似于 `vec![]`。 |

## 常量

| 名称 | 类型 | 值 | 描述 |
|------|------|-------|-------------|
| `PIDS` | `usize` | `256` | 进程 ID 列表的内联容量。大小可覆盖典型被管理进程数量，无需堆分配。用作 `List<[u32; PIDS]>` 的类型参数。 |
| `TIDS_FULL` | `usize` | `96` | 每个进程完整线程 ID 列表的内联容量。可容纳可能会产生大量线程的重量级应用（游戏、浏览器）。 |
| `TIDS_CAPED` | `usize` | `32` | 受限（已过滤）线程 ID 列表的内联容量。仅在需跟踪线程子集（如 Prime 线程候选）时使用。 |
| `CONSUMER_CPUS` | `usize` | `32` | 亲和性、CPU 集合和 Prime 线程规范中 CPU 索引数组的内联容量。覆盖消费级处理器多达 32 个逻辑内核，无需堆分配。 |
| `PENDING` | `usize` | `16` | 待处理操作队列的内联容量，例如单个应用周期内的延迟线程提升或降级操作。 |

## 语法

```rust
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

pub type HashMap<K, V> = FxHashMap<K, V>;
pub type HashSet<V> = FxHashSet<V>;
pub type List<E> = SmallVec<E>;
pub use smallvec::smallvec as list;

pub const PIDS: usize = 256;
pub const TIDS_FULL: usize = 96;
pub const TIDS_CAPED: usize = 32;
pub const CONSUMER_CPUS: usize = 32;
pub const PENDING: usize = 16;
```

## 备注

- **为何选择 `FxHashMap` 而非 `std::HashMap`？** 默认的 `std::HashMap` 使用 SipHash-1-3，以吞吐量为代价提供 HashDoS 防护。ProcGovernor 仅在紧密轮询循环中对本地可信数据（PID、TID、进程名）进行哈希，因此更快的 Fx 算法可带来可测量的延迟改善，且不存在安全性牺牲。
- **为何选择 `SmallVec` 而非 `Vec`？** 大多数亲和性规则针对少于 32 个 CPU，且大多数进程少于 96 个线程。`SmallVec` 将这些常见情况完全保持在栈上，避免了应用循环热路径中的分配器开销。上述容量常量针对消费级硬件调优；拥有超过 32 个逻辑处理器的工作站或服务器拓扑会透明地溢出到堆。
- **命名约定：** 常量使用短大写名称（`PIDS`、`TIDS_FULL`），因为它们在类型签名中作为泛型数组大小参数出现（例如 `List<[u32; CONSUMER_CPUS]>`），简洁性提升了调用处的可读性。

## 需求

| | |
|---|---|
| **模块** | `src/collections.rs` |
| **Crate 依赖** | [rustc-hash](https://crates.io/crates/rustc-hash)、[smallvec](https://crates.io/crates/smallvec) |
| **调用方** | 项目中的所有模块均从 `collections` 导入 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 配置解析（主要消费者） | [config.rs](../config.rs/README.md) |
| 应用循环（热路径消费者） | [apply.rs](../apply.rs/README.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 进程快照数据 | [ProcessEntry](../process.rs/ProcessEntry.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
