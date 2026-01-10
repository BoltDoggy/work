# 安装指南

本文档提供 `work` CLI 工具的详细安装说明。

## 目录

- [快速安装](#快速安装)
- [系统要求](#系统要求)
- [安装方式](#安装方式)
- [验证安装](#验证安装)
- [卸载](#卸载)
- [故障排除](#故障排除)

## 快速安装

```bash
curl -sSL https://raw.githubusercontent.com/BoltDoggy/work/main/install.sh | bash
```

## 系统要求

- **操作系统**: Linux, macOS, 或 Windows (WSL)
- **Git**: 2.5.0+ (推荐 2.30.0+)
- **架构**: x86_64 或 aarch64 (ARM64)

## 安装方式

### 方式一：一键安装脚本 ⭐ 推荐

这是最简单的安装方式，自动检测您的系统并下载对应的二进制文件。

```bash
# 基础安装
curl -sSL https://raw.githubusercontent.com/BoltDoggy/work/main/install.sh | bash

# 指定版本
VERSION=v0.1.0 curl -sSL https://raw.githubusercontent.com/BoltDoggy/work/main/install.sh | bash

# 自定义安装目录
INSTALL_DIR=/usr/local/bin curl -sSL https://raw.githubusercontent.com/BoltDoggy/work/main/install.sh | bash
```

**安装位置**: 默认安装到 `~/.local/bin/work`

**添加到 PATH**:

```bash
# 临时添加（当前会话）
export PATH="$PATH:$HOME/.local/bin"

# 永久添加（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.bashrc
source ~/.bashrc
```

### 方式二：预编译二进制文件

从 [GitHub Releases](https://github.com/BoltDoggy/work/releases) 下载预编译的二进制文件。

#### Linux (x86_64)

```bash
# 下载
wget https://github.com/BoltDoggy/work/releases/latest/download/work-linux-x86_64.tar.gz
# 或
curl -LO https://github.com/BoltDoggy/work/releases/latest/download/work-linux-x86_64.tar.gz

# 解压
tar xzf work-linux-x86_64.tar.gz

# 安装
sudo mv work /usr/local/bin/

# 验证
work --version
```

#### Linux (ARM64/aarch64)

```bash
# 下载
wget https://github.com/BoltDoggy/work/releases/latest/download/work-linux-aarch64.tar.gz

# 解压
tar xzf work-linux-aarch64.tar.gz

# 安装
sudo mv work /usr/local/bin/
```

#### macOS (Intel)

```bash
# 下载
curl -LO https://github.com/BoltDoggy/work/releases/latest/download/work-macos-x86_64.tar.gz

# 解压
tar xzf work-macos-x86_64.tar.gz

# 安装
sudo mv work /usr/local/bin/
```

#### macOS (Apple Silicon)

```bash
# 下载
curl -LO https://github.com/BoltDoggy/work/releases/latest/download/work-macos-aarch64.tar.gz

# 解压
tar xzf work-macos-aarch64.tar.gz

# 安装
sudo mv work /usr/local/bin/
```

#### Windows

1. 下载 [work-windows-x86_64.zip](https://github.com/BoltDoggy/work/releases/latest/download/work-windows-x86_64.zip)
2. 解压到任意目录（如 `C:\Program Files\work\`）
3. 将解压目录添加到系统 PATH：
   - 打开"系统属性" → "高级" → "环境变量"
   - 在"系统变量"中找到 `Path`，点击"编辑"
   - 添加解压目录路径
   - 重启命令提示符

### 方式三：Homebrew

适用于 macOS 和 Linux 用户。

```bash
# 添加 tap
brew tap BoltDoggy/work

# 安装
brew install work

# 验证
work --version

# 更新
brew upgrade work
```

### 方式四：Cargo

如果您已经安装 Rust 工具链。

#### 从 crates.io 安装（待发布）

```bash
cargo install work-cli
```

#### 从 GitHub 源码安装

```bash
cargo install --git https://github.com/BoltDoggy/work
```

### 方式五：从源码编译

```bash
# 克隆仓库
git clone https://github.com/BoltDoggy/work.git
cd work

# 编译并安装
cargo install --path .

# 二进制文件将安装到 ~/.cargo/bin/work
```

## 验证安装

```bash
# 检查版本
work --version

# 查看帮助
work --help

# 列出 worktree
work list
```

## 卸载

### 从脚本安装的版本

```bash
# 删除二进制文件
rm ~/.local/bin/work

# 或如果安装到 /usr/local/bin
sudo rm /usr/local/bin/work
```

### 从 Homebrew 安装的版本

```bash
brew uninstall work
```

### 从 Cargo 安装的版本

```bash
cargo uninstall work-cli
# 或
cargo uninstall --git https://github.com/BoltDoggy/work
```

## 故障排除

### 问题：命令未找到

**症状**: 运行 `work` 命令时提示 `command not found`

**解决方案**:

1. 确认二进制文件位置：
   ```bash
   which work
   # 或
   find ~ -name "work" -type f 2>/dev/null
   ```

2. 检查 PATH：
   ```bash
   echo $PATH
   ```

3. 添加到 PATH（如果需要）：
   ```bash
   # 临时
   export PATH="$PATH:/path/to/work"

   # 永久（添加到 ~/.bashrc 或 ~/.zshrc）
   echo 'export PATH="$PATH:/path/to/work"' >> ~/.bashrc
   source ~/.bashrc
   ```

### 问题：权限被拒绝

**症状**: `Permission denied` when running `work`

**解决方案**:
```bash
chmod +x $(which work)
```

### 问题：版本过旧

**解决方案**:
```bash
# 重新运行安装脚本
curl -sSL https://raw.githubusercontent.com/BoltDoggy/work/main/install.sh | bash

# 或使用包管理器更新
brew upgrade work
cargo install work-cli --force
```

### 问题：无法连接 GitHub Releases

**解决方案**:
- 检查网络连接
- 使用代理或镜像
- 从源码编译作为替代方案

## 更多帮助

- 查看 [README.md](README.md) 了解使用方法
- 提交 [Issue](https://github.com/BoltDoggy/work/issues)
- 查看 [GitHub Discussions](https://github.com/BoltDoggy/work/discussions)
