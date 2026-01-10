#!/bin/bash
# Installation script for work CLI tool

set -e

REPO="BoltDoggy/work"
VERSION=${VERSION:-"latest"}
INSTALL_DIR=${INSTALL_DIR:-"$HOME/.local/bin"}
BINARY_NAME="work"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux*)
            OS="linux"
            ;;
        Darwin*)
            OS="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            OS="windows"
            ;;
        *)
            error "Unsupported OS: $OS"
            exit 1
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac

    echo "$OS-$ARCH"
}

# Get download URL
get_download_url() {
    local platform=$1
    local version=$2

    if [ "$version" = "latest" ]; then
        # Get latest release tag
        LATEST_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "tag_name" | cut -d '"' -f 4)
        if [ -z "$LATEST_URL" ]; then
            error "Failed to fetch latest version"
            exit 1
        fi
        version=$LATEST_URL
    fi

    local filename="work-$platform"

    if [ "$platform" = *"windows"* ]; then
        filename="$filename.zip"
    else
        filename="$filename.tar.gz"
    fi

    echo "https://github.com/$REPO/releases/download/$version/$filename"
}

# Download and install
install_binary() {
    local platform=$(detect_platform)
    local download_url=$(get_download_url "$platform" "$VERSION")

    info "Detected platform: $platform"
    info "Download URL: $download_url"

    # Create temp directory
    TMP_DIR=$(mktemp -d)
    cd "$TMP_DIR"

    # Download
    info "Downloading $BINARY_NAME..."
    if ! curl -L -o "$BINARY_NAME.archive" "$download_url"; then
        error "Failed to download binary"
        exit 1
    fi

    # Extract
    info "Extracting archive..."
    if [[ "$download_url" == *.tar.gz ]]; then
        tar xzf "$BINARY_NAME.archive"
    elif [[ "$download_url" == *.zip ]]; then
        unzip -q "$BINARY_NAME.archive"
    else
        error "Unknown archive format"
        exit 1
    fi

    # Make executable
    chmod +x "$BINARY_NAME"

    # Create install directory if needed
    mkdir -p "$INSTALL_DIR"

    # Install
    info "Installing to $INSTALL_DIR..."
    mv "$BINARY_NAME" "$INSTALL_DIR/"

    # Cleanup
    cd -
    rm -rf "$TMP_DIR"

    info "Successfully installed $BINARY_NAME to $INSTALL_DIR/$BINARY_NAME"
}

# Check if INSTALL_DIR is in PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR is not in your PATH"
        info "Add the following to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        echo ""
    fi
}

# Main
info "Installing $BINARY_NAME..."
install_binary
check_path

info "Installation complete!"
info "Run '$BINARY_NAME --version' to verify"
