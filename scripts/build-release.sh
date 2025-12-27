#!/bin/bash

# Treecmd 跨平台构建脚本
# 用于本地构建所有平台的发布二进制文件

set -e

# 颜色输出
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 输出函数
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查 Rust 是否安装
if ! command -v cargo &> /dev/null; then
    error "Rust 未安装，请先安装 Rust"
    exit 1
fi

# 创建输出目录
OUTPUT_DIR="./release-builds"
mkdir -p "$OUTPUT_DIR"

info "开始构建发布版本..."
info "输出目录: $OUTPUT_DIR"

# 定义构建目标
declare -a TARGETS=(
    "x86_64-pc-windows-msvc:tree.exe:Windows-x86_64"
    "x86_64-unknown-linux-gnu:tree:Linux-x86_64"
    "aarch64-unknown-linux-gnu:tree:Linux-ARM64"
    "x86_64-apple-darwin:tree:macOS-x86_64"
    "aarch64-apple-darwin:tree:macOS-ARM64"
)

# 获取当前版本
VERSION=$(grep -E '^version\s*=' Cargo.toml | head -1 | cut -d'"' -f2)
info "当前版本: v$VERSION"

# 构建函数
build_target() {
    local target=$1
    local bin_name=$2
    local platform_name=$3

    info "构建 $platform_name ($target)..."

    # 检查目标是否可用
    if ! rustup target list | grep -q "$target (installed)"; then
        warn "安装目标平台: $target"
        rustup target add "$target"
    fi

    # 构建
    if cargo build --release --target "$target"; then
        local source="target/$target/release/$bin_name"
        local dest="$OUTPUT_DIR/tree-$VERSION-$platform_name"

        if [[ "$target" == *"windows"* ]]; then
            dest="$dest.exe"
        fi

        cp "$source" "$dest"
        info "✅ 构建成功: $(basename "$dest")"

        # 生成 SHA256
        if command -v shasum &> /dev/null; then
            shasum -a 256 "$dest" > "$dest.sha256"
        elif command -v certutil &> /dev/null; then
            certutil -hashfile "$dest" SHA256 | findstr /v "hash" > "$dest.sha256"
        fi

        # 显示文件大小
        local size=$(stat -f%z "$dest" 2>/dev/null || stat -c%s "$dest" 2>/dev/null)
        info "   文件大小: $(echo "scale=2; $size/1024/1024" | bc) MB"
    else
        error "构建失败: $platform_name"
        return 1
    fi
}

# 本地平台构建（不指定目标）
build_local() {
    info "构建本地平台..."
    if cargo build --release; then
        local source="target/release/tree"
        [[ "$OSTYPE" == "msys" ]] && source="target/release/tree.exe"

        local dest="$OUTPUT_DIR/tree-$VERSION-Local"
        [[ "$OSTYPE" == "msys" ]] && dest="$dest.exe"

        cp "$source" "$dest"
        info "✅ 本地构建成功: $(basename "$dest")"
    else
        error "本地构建失败"
        return 1
    fi
}

# 主流程
main() {
    local mode=${1:-"local"}

    case "$mode" in
        "all")
            info "构建所有目标平台..."
            for target_info in "${TARGETS[@]}"; do
                IFS=':' read -r target bin_name platform_name <<< "$target_info"
                build_target "$target" "$bin_name" "$platform_name"
            done
            ;;
        "local")
            build_local
            ;;
        "help"|"-h"|"--help")
            echo "用法: $0 [mode]"
            echo ""
            echo "模式:"
            echo "  local    - 仅构建本地平台 (默认)"
            echo "  all      - 构建所有目标平台"
            echo "  help     - 显示此帮助信息"
            echo ""
            echo "示例:"
            echo "  $0 local    # 构建当前平台"
            echo "  $0 all      # 构建所有平台"
            exit 0
            ;;
        *)
            error "未知模式: $mode"
            echo "使用 '$0 help' 查看帮助"
            exit 1
            ;;
    esac

    info ""
    info "构建完成！"
    info "输出目录: $OUTPUT_DIR"
    info ""
    info "发布的文件:"
    ls -lh "$OUTPUT_DIR" 2>/dev/null || warn "没有找到构建产物"
}

# 检查是否在项目根目录
if [ ! -f "Cargo.toml" ]; then
    error "请在项目根目录运行此脚本"
    exit 1
fi

# 运行主函数
main "$@"