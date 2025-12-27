#!/bin/bash

# Treecmd 一键安装脚本
# 支持 Linux 和 macOS

set -e

# 颜色
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# 检测操作系统和架构
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)

    case "$os" in
        Linux)
            OS="linux"
            ;;
        Darwin)
            OS="darwin"
            ;;
        *)
            error "不支持的操作系统: $os"
            exit 1
            ;;
    esac

    case "$arch" in
        x86_64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "不支持的架构: $arch"
            exit 1
            ;;
    esac

    info "检测到平台: $OS-$ARCH"
}

# 获取最新版本
get_latest_version() {
    info "获取最新版本信息..."

    # 尝试使用 GitHub API
    if command -v curl &> /dev/null; then
        LATEST_VERSION=$(curl -s https://api.github.com/repos/yourusername/treecmd/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
        if [ -z "$LATEST_VERSION" ]; then
            warn "无法获取最新版本，使用 v0.1.0"
            LATEST_VERSION="v0.1.0"
        fi
    else
        warn "curl 未安装，使用 v0.1.0"
        LATEST_VERSION="v0.1.0"
    fi

    info "最新版本: $LATEST_VERSION"
}

# 下载并安装
install_treecmd() {
    local version=$1
    local target_file="tree-${version}-${OS}-${ARCH}"

    # 映射架构名称
    if [ "$ARCH" = "x86_64" ]; then
        if [ "$OS" = "linux" ]; then
            target_file="tree-${version#v}-x86_64-unknown-linux-gnu"
        elif [ "$OS" = "darwin" ]; then
            target_file="tree-${version#v}-x86_64-apple-darwin"
        fi
    elif [ "$ARCH" = "aarch64" ]; then
        if [ "$OS" = "linux" ]; then
            target_file="tree-${version#v}-aarch64-unknown-linux-gnu"
        elif [ "$OS" = "darwin" ]; then
            target_file="tree-${version#v}-aarch64-apple-darwin"
        fi
    fi

    local download_url="https://github.com/yourusername/treecmd/releases/download/${version}/${target_file}"

    info "下载: $download_url"

    if ! curl -L -o "/tmp/tree" "$download_url"; then
        error "下载失败"
        exit 1
    fi

    chmod +x "/tmp/tree"

    # 确定安装位置
    local install_dir="/usr/local/bin"
    if [ ! -w "$install_dir" ]; then
        install_dir="$HOME/.local/bin"
        mkdir -p "$install_dir"
        warn "没有写入 /usr/local/bin 的权限，安装到 $install_dir"

        # 添加到 PATH
        if [[ ":$PATH:" != *":$install_dir:"* ]]; then
            echo "" >> ~/.bashrc
            echo "# treecmd 安装目录" >> ~/.bashrc
            echo "export PATH=\"\$PATH:$install_dir\"" >> ~/.bashrc
            info "已将 $install_dir 添加到 PATH，请重新启动终端或运行: source ~/.bashrc"
        fi
    fi

    local install_path="$install_dir/tree"

    # 备份现有版本
    if [ -f "$install_path" ]; then
        backup_path="$install_path.backup.$(date +%Y%m%d_%H%M%S)"
        cp "$install_path" "$backup_path"
        info "已备份现有版本到: $backup_path"
    fi

    # 安装
    if sudo cp "/tmp/tree" "$install_path" 2>/dev/null || cp "/tmp/tree" "$install_path"; then
        info "✅ 安装成功: $install_path"
        rm "/tmp/tree"
    else
        error "安装失败，请检查权限"
        exit 1
    fi
}

# 验证安装
verify_installation() {
    if command -v tree &> /dev/null; then
        info "验证安装..."
        tree --version
        info "✅ treecmd 安装成功！"
        echo ""
        info "使用方法:"
        echo "  tree           - 显示当前目录"
        echo "  tree -L 2      - 限制深度为2"
        echo "  tree --help    - 查看所有选项"
    else
        error "安装验证失败"
        exit 1
    fi
}

# 主函数
main() {
    info "开始安装 treecmd..."

    detect_platform
    get_latest_version
    install_treecmd "$LATEST_VERSION"
    verify_installation
}

# 检查依赖
check_dependencies() {
    local missing=()

    if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
        missing+=("curl 或 wget")
    fi

    if [ ${#missing[@]} -gt 0 ]; then
        error "缺少依赖: ${missing[*]}"
        error "请先安装这些工具"
        exit 1
    fi
}

# 运行
check_dependencies
main "$@"