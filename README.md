# Modern Tree Command

一个用Rust开发的现代tree命令，兼容Linux tree命令的所有参数，可在Windows、Linux和macOS上运行。

## 功能特性

### 🚀 核心功能
- **完全兼容Linux tree命令**：支持所有Linux tree命令的参数
- **跨平台支持**：可在Windows、Linux和macOS上运行
- **高性能**：优化的目录遍历算法，默认限制深度为3层
- **轻量级**：优化的二进制大小，仅约1.3MB
- **彩色输出**：支持终端彩色显示，可通过参数控制
- **JSON输出**：支持以JSON格式输出目录结构

### 💡 增强特性
- **深度限制**：默认限制遍历深度为3层，避免在大型目录中性能问题
- **并行处理**：支持多线程并行处理（可选）
- **正则表达式过滤**：支持使用正则表达式过滤文件和目录
- **文件类型指示**：在目录后添加"/"，符号链接后添加"@"
- **权限显示**：显示文件权限信息
- **大小显示**：显示文件大小，支持人性化格式
- **时间显示**：显示文件修改时间

## 安装方法

### 从源码构建

```bash
# 克隆仓库
git clone <repository-url>
cd treecmd

# 开发构建
cargo build

# 发布构建（推荐，优化大小和性能）
cargo build --release

# 可执行文件位于 target/release/tree.exe (Windows) 或 target/release/tree (Linux/macOS)
```

### 直接使用二进制文件

从GitHub Releases页面下载对应平台的二进制文件，添加到系统PATH中即可使用。

## 使用说明

### 基本用法

```bash
tree [options] [path]
```

### 主要参数

| 参数 | 长选项 | 描述 |
|------|--------|------|
| `-a` | `--all` | 显示所有文件和目录，包括隐藏文件 |
| `-A` | `--ascii` | 使用ASCII线条字符 |
| `-C` | `--color` | 彩色输出文件名 |
| `-d` | `--dirs-only` | 仅显示目录，不显示文件 |
| `-D` | `--mtime` | 显示文件最后修改时间 |
| `-F` | `--filelimit` | 在目录后添加"/"，符号链接后添加"@" |
| `-f` | `--full-path` | 显示每个文件的完整路径前缀 |
| `-g` | `--gid` | 显示文件所属组名称或GID |
| `-i` | `--noreport` | 不显示缩进线，仅显示文件列表 |
| `-I` | `--exclude` | 排除匹配指定模式的文件和目录 |
| `-l` | `--follow-links` | 跟随符号链接，视为目录 |
| `-L` | `--level` | 限制显示的目录深度（默认：3） |
| `-n` | `--no-color` | 不显示彩色输出 |
| `-P` | `--include` | 仅显示匹配指定模式的文件和目录 |
| `-p` | `--perms` | 显示文件权限 |
| `-q` | `--quiet` | 用问号代替不可打印字符 |
| `-s` | `--size` | 显示文件大小 |
| `-t` | `--sort` | 按指定顺序排序（默认：name，可选：time） |
| `-u` | `--uid` | 显示文件所有者名称或UID |
| `-x` | `--samefilesystem` | 仅遍历当前文件系统 |
| `--json` | | 以JSON格式输出结果 |
| `--progress` | | 显示处理进度 |
| `--threads` | | 指定并行处理的线程数 |

## 使用示例

### 默认输出

```bash
tree
```

输出：
```
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── config.rs
│   ├── formatter.rs
│   ├── main.rs
│   └── walker.rs
└── target
    ├── debug
    └── release

5 directorys,  6 files
```

### 显示所有文件（包括隐藏文件）

```bash
tree -a
```

### 仅显示目录

```bash
tree -d
```

### 限制深度为2

```bash
tree -L 2
```

### 显示文件大小和权限

```bash
tree -p -s
```

### 按修改时间排序

```bash
tree -t
```

### 排除特定文件

```bash
tree -I "*.log"
```

### 仅显示特定类型的文件

```bash
tree -P "*.rs"
```

## 与Linux tree命令的兼容性

本项目完全兼容Linux tree命令的所有参数，在功能和输出格式上保持一致。主要差异包括：

1. 二进制大小更小（约1.3MB vs Linux tree的约4MB）
2. 默认深度限制为3层，提高性能
3. 增强的JSON输出格式
4. 并行处理支持

## 性能优化

### 二进制大小优化

项目使用了多种优化技术来减小二进制大小：

- **链接时优化（LTO）**：`lto = true`
- **单代码生成单元**：`codegen-units = 1`
- **Abort on panic**：`panic = "abort"`
- **Strip调试信息**：`strip = true`
- **条件编译**：仅在需要时包含彩色输出等功能
- **移除不必要的依赖**：手动实现了部分功能，避免引入大型库

### 性能优化

- **默认深度限制**：默认限制深度为3层，避免在大型目录中遍历过深
- **高效的目录遍历**：使用walkdir库进行高效的目录遍历
- **并行处理支持**：可选的多线程处理

## 构建选项

### 开发构建

```bash
cargo build
```

### 发布构建（优化大小和性能）

```bash
cargo build --release
```

### 构建不带彩色输出的版本

```bash
cargo build --release --no-default-features
```

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！

## 联系方式

如有问题或建议，请通过GitHub Issues与我联系。