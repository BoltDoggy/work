#!/usr/bin/env bash
# 快速编译脚本 for macOS

set -e

echo "🔧 检测系统..."

# 检测操作系统
OS="$(uname -s)"
case "${OS}" in
    Linux*)
        echo "✅ 检测到 Linux"
        ;;
    Darwin*)
        echo "✅ 检测到 macOS"
        ;;
    *)
        echo "❌ 不支持的操作系统: ${OS}"
        exit 1
        ;;
esac

# 检查 Homebrew (macOS)
if [[ "${OS}" == "Darwin" ]]; then
    if ! command -v brew &> /dev/null; then
        echo "❌ Homebrew 未安装"
        echo "请访问 https://brew.sh/ 安装 Homebrew"
        exit 1
    fi
    echo "✅ Homebrew 已安装"
fi

# 检查 Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust 未安装"
    echo "请访问 https://rustup.rs/ 安装 Rust"
    exit 1
fi
echo "✅ Rust 已安装"

# 检查 OpenSSL
if [[ "${OS}" == "Darwin" ]]; then
    if ! brew list openssl &> /dev/null; then
        echo "📦 安装 OpenSSL..."
        brew install openssl
    else
        echo "✅ OpenSSL 已安装"
    fi
    export OPENSSL_DIR=$(brew --prefix openssl)
    export PKG_CONFIG_PATH=$(brew --prefix openssl)/lib/pkgconfig
fi

echo ""
echo "🚀 开始编译..."

# 编译
if [[ "${OS}" == "Darwin" ]]; then
    OPENSSL_DIR=$(brew --prefix openssl) cargo build --release
else
    cargo build --release
fi

echo ""
echo "✅ 编译完成！"
echo ""
echo "二进制文件: target/release/work"
echo ""
echo "尝试运行:"
echo "  ./target/release/work --help"
echo "  ./target/release/work list"
echo ""
echo "更多信息请查看 COMPILATION.md"
