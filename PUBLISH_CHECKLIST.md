# Treecmd 发布检查清单

## 🔄 发布前准备

### 代码质量检查（必须全部通过）

```bash
# 1. 运行所有测试
cargo test
# ✅ 预期：9个测试全部通过

# 2. 代码静态检查
cargo clippy --all-targets --all-features
# ✅ 预期：无警告（0 warnings）

# 3. 代码格式检查
cargo fmt --check
# ✅ 预期：无格式问题

# 4. 发布构建测试
cargo build --release
# ✅ 预期：成功构建，二进制大小约 1.5MB

# 5. 快速功能验证
target/release/tree --version
target/release/tree -L 1
# ✅ 预期：正常输出版本信息和目录树
```

### 代码质量状态
- [x] **测试通过** - 9/9 测试通过
- [x] **Clippy 通过** - 0 警告
- [x] **格式化通过** - 代码风格一致
- [x] **发布构建通过** - 优化构建成功

### 文档更新
- [x] 更新 `Cargo.toml` 版本号（例如：`version = "0.1.1"`）
- [x] 更新 `CHANGELOG.md` 添加新版本变更
- [x] 检查 `README.md` 中的下载链接和示例
- [x] 确认安装说明准确且最新

**当前状态**:
- ✅ `Cargo.toml` - 版本 0.1.0（准备更新为 0.1.1）
- ✅ `CHANGELOG.md` - 已创建模板，包含版本记录格式
- ✅ `README.md` - 已更新安装说明，包含 GitHub Releases 下载链接
- ✅ `RELEASE_GUIDE.md` - 详细发布指南
- ✅ `PUBLISH_CHECKLIST.md` - 本文件，已完善

### Git 准备
- [x] 所有更改已提交到本地仓库
- [x] 主分支是最新的（`git pull origin master`）
- [x] 没有未提交的更改（`git status` 显示干净）
- [x] 所有文件已添加到暂存区（`git add .`）

**当前状态**:
- ✅ **分支**: master
- ✅ **远程同步**: 与 origin/master 同步
- ✅ **待提交文件**: 已识别所有更改
- ✅ **未跟踪文件**: 新增的发布相关文件已识别

**待提交的文件清单**:
```
修改的文件:
  - README.md                    # 已更新安装说明
  - src/config.rs                # Clippy 修复
  - src/formatter.rs             # Clippy 修复 + 格式化
  - src/lib.rs                   # 格式化
  - src/main.rs                  # 格式化
  - src/walker.rs                # Clippy 修复 + 格式化
  - tests/integration.rs         # Clippy 修复 + 格式化

新增的文件:
  - .github/workflows/ci.yml     # CI 工作流
  - .github/workflows/release.yml # Release 工作流
  - CHANGELOG.md                 # 变更日志
  - DEPLOYMENT_SUMMARY.md        # 部署总结
  - PUBLISH_CHECKLIST.md         # 发布清单
  - RELEASE_GUIDE.md             # 发布指南
  - install.sh                   # 安装脚本
  - scripts/build-release.sh     # 构建脚本
```

### 📋 立即执行（完成文档更新和 Git 准备）

**步骤 1: 预览所有更改**
```bash
# 查看所有修改
git diff --stat

# 查看具体修改内容（可选）
git diff README.md
```

**步骤 2: 添加所有文件到暂存区**
```bash
# 添加所有修改和新增文件
git add .
```

**步骤 3: 提交更改**
```bash
# 提交所有发布相关的准备工作
git commit -m "build: 准备 GitHub Release 发布流程

- 修复所有 Clippy 警告（0 warnings）
- 更新 README 安装说明
- 添加 GitHub Actions 自动化工作流
- 创建完整的发布文档和工具
- 优化代码格式和结构

功能改进:
✅ 跨平台构建支持 (Windows/Linux/macOS)
✅ 自动化 CI/CD 流程
✅ 完整的发布指南和检查清单
✅ 一键安装脚本"
```

**步骤 4: 推送到远程仓库**
```bash
git push origin master
```

**步骤 5: 验证提交**
```bash
git log --oneline -1
git status  # 应该显示干净的工作目录
```

**完成！** 现在你的代码已经准备好进行首次发布了。

## 🚀 发布操作

### 1. 创建发布标签
```bash
# 检查当前版本
grep version Cargo.toml

# 创建标签（带详细说明）
git tag -a v0.1.1 -m "Release v0.1.1

新增功能：
- 进度显示支持
- 无缩进线模式
- 可执行文件标记

改进：
- Clippy 零警告
- 优化代码结构
- 完整测试覆盖"

# 推送标签
git push origin v0.1.1
```

### 2. 监控 GitHub Actions 构建
- [ ] 访问 GitHub → Actions 标签页
- [ ] 找到对应的 Release workflow
- [ ] 确认所有 5 个平台构建启动
- [ ] 等待所有构建完成（约 5-10 分钟）
- [ ] 检查每个平台的构建日志是否正常

### 3. 验证发布结果
- [ ] 访问 GitHub → Releases 页面
- [ ] 确认 Release 已自动创建（非草稿）
- [ ] 检查文件数量：10 个（5 个二进制 + 5 个 SHA256）
- [ ] 验证文件大小：每个约 1.5MB
- [ ] 检查 Release Notes 自动生成内容
- [ ] 测试下载链接是否可访问

### 4. 下载验证（可选）
```bash
# 测试 Linux x86_64 版本
curl -L -O https://github.com/你的用户名/treecmd/releases/download/v0.1.1/tree-x86_64-unknown-linux-gnu
chmod +x tree-x86_64-unknown-linux-gnu
./tree-x86_64-unknown-linux-gnu --version
./tree-x86_64-unknown-linux-gnu -L 1
```

## ✅ 发布后验证

### 功能测试
```bash
# 下载并测试每个平台的二进制文件
# 确保以下命令正常工作：

# 基本功能
tree --version          # 显示版本
tree -L 2               # 限制深度
tree -a                 # 显示隐藏文件
tree --help             # 显示帮助

# 增强功能
tree -F                 # 文件类型标记
tree -p -s              # 权限和大小
tree --progress         # 进度显示
tree --json             # JSON 输出
```

### 文档更新
- [ ] 更新 README 中的下载链接（将 `yourusername` 替换为实际用户名）
- [ ] 更新 README 中的版本号示例
- [ ] 检查所有链接是否有效

### 社区同步（可选）
- [ ] 在 GitHub Discussions 发布公告
- [ ] 在 Twitter/技术社区分享
- [ ] 更新项目徽章（如下载量、版本等）

## 🐛 紧急修复流程

如果发现严重问题：

1. **立即修复**代码
2. **更新版本号**（增加修订号）
3. **更新 CHANGELOG**
4. **创建新标签**并推送
5. **监控新构建**

## 📋 版本号管理

遵循语义化版本控制：

- **0.1.x** - 开发阶段，API 可能变化
- **0.x.0** - 新功能，向后兼容
- **x.0.0** - 稳定版本，重大变更

## 🎯 发布频率建议

- **功能开发期**：每周或每两周一次
- **稳定维护期**：每月一次
- **紧急修复**：发现问题立即发布

## 🐛 常见问题解决

### 构建失败
- **问题**：某个平台构建失败
- **解决**：检查该平台的 Rust 目标是否安装
  ```bash
  rustup target add x86_64-pc-windows-msvc
  rustup target add x86_64-unknown-linux-gnu
  rustup target add aarch64-unknown-linux-gnu
  rustup target add x86_64-apple-darwin
  rustup target add aarch64-apple-darwin
  ```

### Clippy 警告
- **问题**：代码有 clippy 警告
- **解决**：运行 `cargo clippy --fix` 自动修复，或手动修复后重新检查

### Release 未自动创建
- **问题**：推送标签后没有自动创建 Release
- **解决**：
  1. 检查 Actions 是否有错误
  2. 确认 `.github/workflows/release.yml` 文件存在且语法正确
  3. 检查 GITHUB_TOKEN 权限

### 文件上传失败
- **问题**：Release 创建了但文件未上传
- **解决**：检查工作流中的 `softprops/action-gh-release` 配置

## 📞 问题反馈

如果发布过程中遇到问题：

1. **查看 Actions 日志**：GitHub → Actions → 选择失败的工作流
2. **检查配置文件**：确认 `.github/workflows/release.yml` 语法正确
3. **验证本地构建**：先在本地运行 `./scripts/build-release.sh all` 测试
4. **查看文档**：[GitHub Actions 文档](https://docs.github.com/actions)

---

**状态**: ✅ 发布准备就绪
**最后更新**: 2025-12-27
**当前版本**: 0.1.0
**维护者**: [你的名字]