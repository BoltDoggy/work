# Work - Git Worktree 管理工具

一个简化的 Git worktree 管理命令行工具，使 worktree 的使用更加直观和高效。

## 功能特性

- 🎨 **彩色输出**: 清晰的视觉标识，主目录（⌂）、当前 worktree（*）、状态标记
- 📋 **列出 worktree**: 查看所有 worktree 及其状态，简洁的 compact 格式
- 🔄 **切换 worktree**: 快速切换到指定的 worktree，支持 shell 集成
- ➕ **创建 worktree**: 基于现有分支或新分支创建 worktree，自动路径管理
- 🗑️ **删除 worktree**: 安全删除不再需要的 worktree，有未提交更改保护
- ℹ️ **查看详情**: 显示 worktree 的详细信息和文件状态
- 🧹 **清理无效 worktree**: 移除已失效的 worktree 注册
- 🔀 **智能命名**: worktree 名称基于目录名，支持分支切换显示

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/BoltDoggy/work.git
cd work

# 编译并安装
cargo install --path .

# 验证安装
work --version
```

## 快速开始

### 列出所有 worktree

```bash
# 默认 compact 格式（彩色输出）
work list

# 输出示例：
# *⌂  worktree on 001-git-worktree-cli (modified)
#   feature-auth on main
#   feature-bugfix
```

**说明**:
- `*` = 当前 worktree
- `⌂` = 主目录（紫色）
- `on <branch>` = 当分支名与目录名不同时显示
- `(modified)` = 有未提交的更改（红色）

### 切换到 worktree

```bash
# 使用 worktree 名称
work switch feature-auth

# Shell 集成（自动切换目录）
eval "$(work switch feature-auth --print-path)"
```

### 创建新 worktree

```bash
# 创建新分支的 worktree
work create feature-auth

# 基于现有分支创建
work create feature-auth --branch main

# 交互式选择基准分支
work create feature-auth --interactive

# 自定义路径
work create feature-auth --path /custom/path
```

**路径规则**: Worktree 自动创建在 `<repo-name>.worktrees/<name>/` 目录下，与主仓库同级。

### 删除 worktree

```bash
# 删除指定 worktree
work delete feature-auth

# 强制删除（忽略未提交的更改）
work delete feature-auth --force

# 交互式选择并删除
work delete --interactive
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

支持三种输出格式（通过 `-o/--output` 参数指定）：

### Compact 格式（默认）

简洁的单行显示，带颜色编码：

```bash
work list
# 或
work list -o compact
```

**示例输出**:
```
*⌂  worktree on 001-git-worktree-cli (modified)
  feature-auth on main
  feature-bugfix
```

### Table 格式

完整表格显示所有列：

```bash
work list -o table
```

**示例输出**:
```
┌──────────────┬──────────────────────┬─────────────────────────────┬─────────┬───────────────┐
│ NAME         │ BRANCH               │ PATH                        │ CURRENT │ STATUS        │
├──────────────┼──────────────────────┼─────────────────────────────┼─────────┼───────────────┤
│ worktree     │ 001-git-worktree-cli │ /Volumes/code/worktree      │ *       │ Healthy       │
├──────────────┼──────────────────────┼─────────────────────────────┼─────────┼───────────────┤
│ feature-auth │ main                 │ /Volumes/code/worktree....  │         │ Healthy       │
└──────────────┴──────────────────────┴─────────────────────────────┴─────────┴───────────────┘
```

### JSON 格式

机器可解析的 JSON 格式：

```bash
work list -o json
```

**示例输出**:
```json
[
  {
    "name": "worktree",
    "branch": "001-git-worktree-cli",
    "path": "/Volumes/code/worktree",
    "is_current": true,
    "is_bare": false,
    "is_detached": false,
    "head_commit": "abc123",
    "upstream_branch": "origin/001-git-worktree-cli"
  }
]
```

## 性能目标

- 列出 20+ worktree: < 2 秒
- 创建/切换 worktree: < 5 秒
- 启动时间: < 100ms

## 系统要求

- Rust 1.75+ (仅编译时需要)
- Git 2.5.0+ (推荐 2.30.0+)
- Linux, macOS, 或 Windows (Git Bash/WSL)

**注意**: 此工具直接调用系统 `git` 命令，无需 OpenSSL 依赖。

## 开发

### 构建

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release
```

### 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_format_worktree_table
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 本地安装

```bash
# 从源码安装到 ~/.cargo/bin
cargo install --path .

# 验证安装
work --version
```

## 架构设计

### 三层架构

```
src/
├── main.rs           # CLI 入口，命令处理器
├── cli/
│   └── output.rs     # 输出格式化（table, compact, json）
├── core/
│   ├── git_ops.rs    # Git 命令执行封装
│   ├── worktree.rs   # Worktree 数据模型
│   └── repository.rs # 仓库管理
└── utils/
    ├── errors.rs     # 错误类型定义
    └── path.rs       # 路径工具函数
```

### 设计决策

1. **系统 Git 集成**: 使用 `std::process::Command` 调用系统 git，避免 `git2` crate 的 OpenSSL 依赖
2. **目录名优先**: worktree 名称基于目录名而非分支名，支持在 worktree 内切换分支
3. **自动路径管理**: 使用 `git rev-parse --git-common-dir` 查找主仓库，确保从任何 worktree 创建新 worktree 路径都正确

## 命令参考

### work list

列出所有 worktree。

```bash
work list [OPTIONS]

选项：
  -o, --output <FORMAT>    输出格式 [default: compact] [possible values: table, compact, json]
```

### work switch

切换到指定的 worktree。

```bash
work switch [NAME] [OPTIONS]

参数：
  <NAME>    Worktree 名称

选项：
      --print-path    仅输出路径供 shell 集成使用
```

### work create

创建新的 worktree。

```bash
work create <NAME> [OPTIONS]

参数：
  <NAME>    分支名或 worktree 名称

选项：
  -b, --branch <BRANCH>       基准分支（用于创建新分支）
  -p, --path <PATH>           自定义路径
  -i, --interactive           交互式选择基准分支
```

### work delete

删除 worktree。

```bash
work delete [NAMES]... [OPTIONS]

参数：
  <NAMES>...    Worktree 名称（可指定多个）

选项：
  -f, --force              强制删除（忽略未提交的更改）
  -i, --interactive        交互式选择要删除的 worktree
```

### work info

显示 worktree 详细信息。

```bash
work info <NAME> [OPTIONS]

参数：
  <NAME>    Worktree 名称

选项：
  -o, --output <FORMAT>    输出格式 [default: table] [possible values: table, json]
```

### work prune

清理无效的 worktree。

```bash
work prune [OPTIONS]

选项：
      --dry-run    预览将要清理的 worktree（不实际删除）
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
