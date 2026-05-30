# read_utf16le_file 函数 (config.rs)

读取以 UTF-16 小端序编码的文件，并将其内容作为标准 Rust UTF-8 `String` 返回。由 [convert](convert.md) 函数用于导入 Process Lasso 配置文件，这些文件通常以 UTF-16LE 编码保存。

## 语法

```rust
pub fn read_utf16le_file(path: &str) -> Result<String>
```

## 参数

`path: &str`

要读取的 UTF-16LE 编码文件的文件系统路径。该路径直接传递给 `std::fs::read` 以进行原始字节加载。

## 返回值

`Result<String>`——成功时返回 `Ok(String)`，包含从 UTF-16LE 解码为 UTF-8 的文件内容。失败时返回从底层 `std::fs::read` 调用传播的 `Err`（例如文件未找到、权限被拒绝）。

## 备注

### 解码算法

1. 整个文件通过 `std::fs::read` 作为原始字节向量读入内存。
2. 字节被分块为两两一对（`chunks_exact(2)`），每对使用 `u16::from_le_bytes([c[0], c[1]])` 解释为小端序 `u16` 代码单元。
3. 生成的 `Vec<u16>` 通过 `String::from_utf16_lossy` 解码为 Rust `String`，对于无效的 UTF-16 代理序列，用 Unicode 替换字符（U+FFFD）替换，而不是返回错误。

### BOM 处理

此函数**不**剥离 UTF-16LE 字节序标记（BOM，`U+FEFF`）。如果源文件以 BOM 开头，返回字符串的第一个字符将是 `\u{FEFF}`。在实践中，这不会影响 [convert](convert.md) 函数，因为 BOM 字符类似于空白，在基于行的解析期间被忽略。

### 奇数字节计数

如果文件的字节数为奇数，最后一个字节会被 `chunks_exact(2)` 静默丢弃。对于这种边缘情况，不会产生任何警告或错误。

### 有损转换

使用 `from_utf16_lossy` 意味着此函数永远不会因为编码问题而失败——只有 I/O 错误能导致失败。任何格式错误的 UTF-16 序列会用 `�`（U+FFFD）替换，这可能产生意外的输出，但不会 panic 或返回错误。

### 与标准文件读取的比较

对于 UTF-8 编码的文件（ProcGovernor 配置文件的默认编码），[read_config](read_config.md) 使用 `std::io::BufReader` 进行逐行读取。`read_utf16le_file` 仅在 Process Lasso 导入时需要，后者使用 Windows 原生的 UTF-16LE 编码。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用者** | [convert](convert.md) |
| **被调用者** | `std::fs::read`、`u16::from_le_bytes`、`String::from_utf16_lossy` |
| **所需权限** | 对指定路径的文件系统读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [config.rs](README.md) |
| Process Lasso 转换器 | [convert](convert.md) |
| 标准配置读取器（UTF-8） | [read_config](read_config.md) |
| 黑名单读取器（UTF-8） | [read_bleack_list](read_bleack_list.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/ProcGovernor/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
