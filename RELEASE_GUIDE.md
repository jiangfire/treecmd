# Treecmd 发布指南

本文档指导如何为 treecmd 项目创建 GitHub Release 发布版本。

## 📋 发布前检查清单

在创建新版本发布之前，请确保：

- [ ] 所有测试通过：`cargo test`
- [ ] 代码已提交到主分支
- [ ] CHANGELOG.md 已更新
- [ ] Cargo.toml 版本号已更新
- [ ] README.md 中的示例链接已更新
- [ ] 构建脚本可执行：`chmod +x scripts/build-release.sh`

## 🚀 发布流程

### 步骤 1: 更新版本信息

```bash
# 1. 更新 Cargo.toml 中的版本号
# 编辑 Cargo.toml，将 version = "0.1.0" 改为 version = "0.1.1"

# 2. 更新 CHANGELOG.md
# 添加新版本的变更记录

# 3. 提交更改
git add Cargo.toml CHANGELOG.md README.md
git commit -m "release: v0.1.1"
```

### 步骤 2: 创建发布标签

```bash
# 创建轻量标签
git tag v0.1.1

# 或创建带注释的标签（推荐）
git tag -a v0.1.1 -m "Release v0.1.1

- 新增进度显示功能
- 优化并行处理性能
- 修复 JSON 输出重复根节点问题"

# 推送标签到远程仓库
git push origin v0.1.1
```

### 步骤 3: GitHub Actions 自动构建

推送标签后，GitHub Actions 会自动：
1. 检出代码
2. 安装 Rust 工具链
3. 为所有目标平台构建二进制文件
4. 生成 SHA256 哈希值
5. 创建 GitHub Release
6. 上传所有构建产物

**监控构建状态：**
- 访问 GitHub 仓库的 "Actions" 标签页
- 找到对应的 Release 工作流
- 等待所有 job 完成（约 5-10 分钟）

### 步骤 4: 验证发布

构建完成后，访问 GitHub Releases 页面：

1. **检查发布状态**：确保不是草稿状态
2. **验证文件数量**：应有 10 个文件（5 个二进制 + 5 个 SHA256）
3. **检查 Release Notes**：自动生成的说明是否正确
4. **测试下载链接**：随机选择几个平台测试下载

### 步骤 5: 手动优化（可选）

如果需要，可以手动编辑 Release：

1. **改进 Release Notes**：添加更详细的说明
2. **添加安装示例**：在描述中添加各平台的安装命令
3. **标记为 Pre-release**：如果是测试版本
4. **添加讨论链接**：关联相关的 Issue 或 PR

## 🔧 手动构建（备用方案）

如果 GitHub Actions 失败，可以手动构建：

```bash
# 使用构建脚本
./scripts/build-release.sh all

# 或者手动构建每个平台
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

然后手动上传到 GitHub Release。

## 📊 发布后任务

- [ ] 在 README 中更新下载链接（替换 yourusername）
- [ ] 在社区分享发布信息（Twitter, Reddit, etc.）
- [ ] 监控 Issue 反馈
- [ ] 准备下一个版本的开发

## 🔄 版本号规则

遵循 [Semantic Versioning](https://semver.org/)：

```
MAJOR.MINOR.PATCH

例如：
0.1.0  - 初始版本
0.1.1  - Bug 修复
0.2.0  - 新功能（向后兼容）
1.0.0  - 首个稳定版本
```

## 🐛 故障排除

### GitHub Actions 构建失败

**问题**：某个平台构建失败
- **解决**：检查该平台的依赖是否完整，Rust 目标是否安装

**问题**：Release 创建失败
- **解决**：检查 GITHUB_TOKEN 权限，确保有 repo 写入权限

### 本地构建问题

**问题**：交叉编译失败
- **解决**：确保安装了对应的目标平台工具链
  ```bash
  rustup target add x86_64-pc-windows-msvc
  rustup target add x86_64-unknown-linux-gnu
  rustup target add aarch64-unknown-linux-gnu
  rustup target add x86_64-apple-darwin
  rustup target add aarch64-apple-darwin
  ```

**问题**：二进制文件过大
- **解决**：检查 Cargo.toml 中的 [profile.release] 配置是否正确

## 📝 发布说明模板

```markdown
## 版本 v0.1.1 - YYYY-MM-DD

### 🆕 新增功能
-

### 🔧 改进优化
-

### 🐛 问题修复
-

### ⚠️ 变更说明
-

### 📦 下载说明

请根据您的平台下载对应的二进制文件：

- **Windows**: tree-x86_64-pc-windows-msvc.exe
- **Linux x86_64**: tree-x86_64-unknown-linux-gnu
- **Linux ARM64**: tree-aarch64-unknown-linux-gnu
- **macOS x86_64**: tree-x86_64-apple-darwin
- **macOS Apple Silicon**: tree-aarch64-apple-darwin

### 🔍 验证完整性

每个文件都提供了 SHA256 哈希值，可以通过以下命令验证：

```bash
# Linux/macOS
shasum -a 256 -c tree-x86_64-unknown-linux-gnu.sha256

# Windows
certutil -hashfile tree-x86_64-pc-windows-msvc.exe SHA256
```

---

**完整变更记录**: [CHANGELOG.md](CHANGELOG.md)
```

## 🎯 高级技巧

### 自动化版本管理

使用 `cargo-set-version` 自动更新版本：
```bash
cargo install cargo-edit
cargo set-version 0.1.1
```

### 生成 Release Notes

使用 GitHub CLI 自动生成：
```bash
gh release create v0.1.1 --generate-notes
```

### 预发布版本

创建测试版本：
```bash
git tag v0.1.1-beta.1
git push origin v0.1.1-beta.1
```

在 GitHub Actions 中标记为预发布。

---

**最后更新**: 2025-12-27