# Changelog

所有关于 treecmd 的变更记录都会记录在此文件中。

## [Unreleased]

### Added
-

### Changed
-

### Fixed
-

### Removed
-

---

## [0.1.0] - 2025-12-27

### Added
- ✨ **基础功能**
  - 完整的目录树显示功能
  - 支持 Linux tree 命令的所有常用参数
  - 跨平台支持 (Windows, Linux, macOS)

- 🎨 **输出格式**
  - 彩色输出支持
  - JSON 格式输出
  - 文件类型标记 (/, @, *)
  - 权限、大小、时间、所有者信息显示

- ⚡ **性能优化**
  - 默认深度限制 (3层)
  - 并行处理支持 (Rayon)
  - 优化的二进制大小 (~1.5MB)
  - 进度显示功能

- 🔧 **增强特性**
  - 正则表达式过滤
  - 自定义排序
  - 符号链接跟随
  - 文件系统边界限制

- 🧪 **质量保证**
  - 完整的测试套件 (9个集成测试)
  - GitHub Actions CI/CD
  - 自动化发布流程

### 技术特性
- 使用 Rust 开发，高性能且内存安全
- 基于 clap 的命令行参数解析
- walkdir 库的高效目录遍历
- serde_json 的结构化输出
- atty 的终端检测

---

## 发布说明模板

创建新版本时，使用以下格式：

```markdown
## [版本号] - YYYY-MM-DD

### Added (新增功能)
-

### Changed (变更)
-

### Fixed (修复问题)
-

### Deprecated (即将移除)
-

### Removed (已移除)
-

### Security (安全相关)
-
```

---

## 发布流程

### 1. 准备发布
```bash
# 更新版本号 (Cargo.toml)
# 更新 CHANGELOG.md
# 确保所有测试通过
cargo test
```

### 2. 创建发布标签
```bash
git add CHANGELOG.md Cargo.toml
git commit -m "release: v0.1.1"
git tag v0.1.1
git push origin v0.1.1
```

### 3. 自动化构建
GitHub Actions 将自动：
- 构建所有平台的二进制文件
- 生成 SHA256 哈希值
- 创建 GitHub Release
- 上传所有构建产物

### 4. 手动检查
- 访问 GitHub Releases 页面
- 检查所有文件是否上传成功
- 验证 Release Notes 是否正确
- 如需要，手动编辑 Release 说明

---

## 版本号规则

遵循 [Semantic Versioning](https://semver.org/)：

- **主版本号 (MAJOR)**: 不兼容的 API 变更
- **次版本号 (MINOR)**: 向后兼容的功能新增
- **修订号 (PATCH)**: 向后兼容的问题修复

示例：
- `v1.0.0` - 首个稳定版本
- `v1.1.0` - 新功能但兼容
- `v1.1.1` - Bug 修复
- `v2.0.0` - 重大变更，不兼容旧版