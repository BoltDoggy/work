# Quick Start Guide: Git Worktree 管理工具

**Feature**: Git Worktree 管理工具
**Date**: 2026-01-10
**Phase**: Phase 1 - Design & Contracts

## Overview

本文档提供 Git Worktree 管理工具的快速入门指南，包括安装、基本使用和常见场景示例。

## Prerequisites

1. **Git**: Git 2.5.0 或更高版本（推荐 2.30.0+）
2. **Rust**: Rust 1.75.0 或更高版本（仅编译时需要）
3. **操作系统**: Linux, macOS, 或 Windows (Git Bash/WSL)

## Installation

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

### 使用预编译二进制文件（未来）

```bash
# Linux
curl -L https://github.com/yourusername/work/releases/latest/download/work-linux-x86_64.tar.gz | tar xz
sudo mv work /usr/local/bin/

# macOS
brew install work  # 假设发布到 Homebrew

# Windows
# 下载 .exe 文件并添加到 PATH
```

## Basic Usage

### 1. 列出所有 Worktree

```bash
# 人类可读格式（默认）
work list

# JSON 格式（用于脚本）
work list -o json

# 简短命令
work ls
```

**示例输出**:

```
NAME          BRANCH          PATH                                    CURRENT  STATUS
main          main            /home/user/project                     *        Healthy
feature-auth  feature-auth    /home/user/project/worktrees/feature-auth           Healthy
bug-fix-123   bug-fix-123     /home/user/project/worktrees/bug-fix-123            Modified (2 files)
```

**JSON 输出**:

```json
{
  "worktrees": [
    {
      "name": "main",
      "branch": "main",
      "path": "/home/user/project",
      "is_current": true,
      "is_bare": false,
      "is_detached": false,
      "head_commit": "a1b2c3d4e5f6789012345678901234567890abcd",
      "upstream_branch": "origin/main",
      "uncommitted_changes": null,
      "last_modified": "2026-01-10T15:30:00Z"
    }
  ]
}
```

### 2. 切换到 Worktree

```bash
# 使用 worktree 名称
work switch feature-auth

# 使用交互式选择
work switch

# 切换后自动 cd 到目标目录（需要 shell 集成）
eval "$(work switch feature-auth --print-path)"
```

**Shell 集成** (添加到 `~/.bashrc` 或 `~/.zshrc`):

```bash
# Bash/Zsh
workcd() {
    local path=$(work switch "$@" --print-path)
    if [ $? -eq 0 ]; then
        cd "$path"
    fi
}

alias ws=workcd
```

### 3. 创建 Worktree

```bash
# 基于现有分支创建
work create feature-auth

# 创建新分支并创建 worktree
work create feature-ui --branch main

# 指定自定义路径
work create feature-auth --path ~/worktrees/feature-auth

# 交互式创建（选择基准分支）
work create --interactive
```

**示例输出**:

```
✓ Created worktree 'feature-auth'
  Branch: feature-auth
  Path: /home/user/project/worktrees/feature-auth
  Commit: a1b2c3d (Add initial auth support)

💡 Tip: Run 'work switch feature-auth' to navigate to the new worktree
```

### 4. 删除 Worktree

```bash
# 删除指定 worktree
work delete feature-auth

# 交互式删除（显示列表并选择）
work delete --interactive

# 强制删除（忽略未提交的更改）
work delete feature-auth --force

# 批量删除多个
work delete feature-auth feature-ui bug-fix-123
```

**确认提示**:

```
⚠️  Delete worktree 'feature-auth'?
    Path: /home/user/project/worktrees/feature-auth
    Uncommitted changes: 2 files

[y/N] _
```

### 5. 查看 Worktree 详情

```bash
# 显示详细信息
work info feature-auth

# JSON 格式
work info feature-auth -o json
```

**示例输出**:

```
Worktree: feature-auth
  Branch: feature-auth
  Path: /home/user/project/worktrees/feature-auth
  HEAD: a1b2c3d4e5f6789012345678901234567890abcd
  Author: Developer <dev@example.com>
  Date: 2026-01-10 14:30:00 +0000
  Message: Add OAuth2 login support

  Upstream: origin/feature-auth

  Uncommitted changes:
    Modified: src/auth.rs, tests/auth_tests.rs
    Staged: README.md
    Untracked: notes.txt
```

### 6. 清理无效 Worktree

```bash
# 清理所有无效的 worktree
work prune

# 预览将要清理的 worktree（不实际删除）
work prune --dry-run
```

**示例输出**:

```
✓ Cleaned up 2 stale worktrees:
  - feature-old (directory not found)
  - bug-fix-456 (directory not found)
```

## Common Workflows

### 工作流 1: 开始新功能开发

```bash
# 1. 创建新的 worktree
work create feature-new-ui --branch main

# 2. 切换到新的 worktree
work switch feature-new-ui

# 3. 开始开发...
# (在新的 worktree 中工作)

# 4. 完成后删除 worktree
work delete feature-new-ui
```

### 工作流 2: 修复紧急 Bug

```bash
# 1. 基于 main 创建 worktree
work create hotfix-critical-bug --branch main

# 2. 切换并修复
work switch hotfix-critical-bug
# (修复 bug...)

# 3. 提交并合并到 main
git add .
git commit -m "Fix critical bug"
git checkout main
git merge hotfix-critical-bug

# 4. 删除 worktree
work delete hotfix-critical-bug
```

### 工作流 3: 并行处理多个任务

```bash
# 创建多个 worktree
work create feature-auth --branch main
work create feature-db --branch main
work create bug-fix-123 --branch main

# 列出所有 worktree
work list

# 切换到需要的 worktree
work switch feature-auth
# (完成 auth 相关工作)

work switch feature-db
# (完成 DB 相关工作)
```

### 工作流 4: 代码审查

```bash
# 1. 为 PR 创建 worktree
work create pr-review-456 --branch origin/pr-456

# 2. 切换并审查
work switch pr-review-456

# 3. 审查完成后删除
work delete pr-review-456
```

## Advanced Usage

### 自动补全

**Bash**:

```bash
# 添加到 ~/.bashrc
eval "$(work completion bash)"
```

**Zsh**:

```bash
# 添加到 ~/.zshrc
eval "$(work completion zsh)"
```

**Fish**:

```bash
# 添加到 ~/.config/fish/completions/work.fish
work completion fish > ~/.config/fish/completions/work.fish
```

### 配置文件

创建 `~/.workconfig.toml`:

```toml
[general]
default_branch = "main"
auto_prune = true
confirm_delete = true

[output]
default_format = "table"
show_untracked_files = true

[aliases]
ls = "list"
rm = "delete"
```

### 环境变量

```bash
# 设置默认输出格式
export WORK_OUTPUT_FORMAT=json

# 禁用确认提示
export WORK_CONFIRM_DELETE=false

# 设置日志级别
export RUST_LOG=debug
```

## Integration with Other Tools

### VS Code

在 VS Code 中打开 worktree:

```bash
# 打开新的 worktree
work create feature-auth
code $(work switch feature-auth --print-path)
```

### Git Aliases

添加到 `~/.gitconfig`:

```ini
[alias]
    # 列出 worktree
    wt = "!work list"
    # 切换 worktree
    wts = "!f() { work switch \"$1\" && cd \"$(work switch \"$1\" --print-path)\"; }; f"
```

### FZF (Fuzzy Finder)

```bash
# 使用 fzf 交互式选择 worktree
work switch $(work list -o json | jq -r '.worktrees[].name' | fzf)
```

## Troubleshooting

### 问题: "Not a git repository"

**原因**: 当前目录不是 Git 仓库或不在 worktree 中

**解决**:
```bash
cd /path/to/your/git/repository
work list
```

### 问题: "Worktree not found"

**原因**: Worktree 名称拼写错误或已被删除

**解决**:
```bash
# 列出所有 worktree 确认名称
work list

# 或使用交互式选择
work switch --interactive
```

### 问题: "Cannot delete current worktree"

**原因**: 尝试删除当前所在的 worktree

**解决**:
```bash
# 先切换到其他 worktree
work switch main
# 然后删除
work delete feature-auth
```

### 问题: "Worktree has uncommitted changes"

**原因**: Worktree 包含未提交的更改

**解决**:
```bash
# 提交或暂存更改
work switch feature-auth
git add .
git commit -m "WIP"

# 或使用 --force 强制删除（不推荐）
work delete feature-auth --force
```

## Performance Tips

1. **大型仓库**: 如果 worktree 数量超过 50，使用 `--json` 格式更快
2. **并行操作**: 创建多个 worktree 时可以并行执行
3. **缓存**: 工具会缓存仓库状态，避免重复打开

## Next Steps

- 查看 README.md 了解更多功能
- 查看 `work --help` 了解所有命令
- 查看 `work <command> --help` 了解特定命令的选项
