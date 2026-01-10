# Work - Git Worktree 管理工具

一个简化的 Git worktree 管理命令行工具，使 worktree 的使用更加直观和高效。

## 功能特性

- 📋 **列出 worktree**: 查看所有 worktree 及其状态
- 🔄 **切换 worktree**: 快速切换到指定的 worktree
- ➕ **创建 worktree**: 基于现有分支或新分支创建 worktree
- 🗑️ **删除 worktree**: 安全删除不再需要的 worktree
- ℹ️ **查看详情**: 显示 worktree 的详细信息
- 🧹 **清理无效 worktree**: 移除已失效的 worktree 注册

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/yourusername/work.git
cd work

# 编译并安装
cargo install --path .

# 验证安装
work --version
```

## 快速开始

### 列出所有 worktree

```bash
work list
```

### 切换到 worktree

```bash
# 使用 worktree 名称
work switch feature-auth

# 交互式选择
work switch

# Shell 集成（自动切换目录）
eval "$(work switch feature-auth --print-path)"
```

### 创建新 worktree

```bash
# 基于现有分支
work create feature-auth

# 创建新分支
work create feature-ui --branch main
```

### 删除 worktree

```bash
work delete feature-auth
```

## Shell 集成

添加到 `~/.bashrc` 或 `~/.zshrc`:

```bash
workcd() {
    local path=$(work switch "$@" --print-path)
    if [ $? -eq 0 ]; then
        cd "$path"
    fi
}

alias ws=workcd
```

然后使用 `ws <worktree-name>` 快速切换。

## 输出格式

支持人类可读和机器可解析两种格式：

```bash
# 表格格式（默认）
work list

# JSON 格式
work list -o json
```

## 性能目标

- 列出 20+ worktree: < 2 秒
- 创建/切换 worktree: < 5 秒
- 启动时间: < 100ms

## 系统要求

- Rust 1.75+ (仅编译时需要)
- Git 2.5.0+ (推荐 2.30.0+)
- Linux, macOS, 或 Windows (Git Bash/WSL)

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
