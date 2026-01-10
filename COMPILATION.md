# 编译和使用指南

## macOS 编译步骤

### 1. 安装 OpenSSL

```bash
# 安装 OpenSSL
brew install openssl

# 设置环境变量
export OPENSSL_DIR=$(brew --prefix openssl)
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include

# 验证安装
echo $OPENSSL_DIR
```

### 2. 编译项目

```bash
# 方法 A: 使用环境变量编译
OPENSSL_DIR=$(brew --prefix openssl) cargo build --release

# 方法 B: 使用 rustflags
export PKG_CONFIG_PATH=$(brew --prefix openssl)/lib/pkgconfig
export PKG_CONFIG_ALL_STATIC=1
export RUSTFLAGS="-L $(brew --prefix openssl)/lib"
cargo build --release

# 方法 C: 使用 pkg-config（推荐）
brew install pkg-config
export PKG_CONFIG_PATH=$(brew --prefix openssl)/lib/pkgconfig
cargo build --release
```

### 3. 验证编译

```bash
# 查看编译的二进制文件
ls -lh target/release/work

# 查看版本信息
./target/release/work --version
```

## Linux 编译步骤

### Ubuntu/Debian

```bash
# 安装依赖
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# 编译
cargo build --release
```

### Fedora/RHEL

```bash
# 安装依赖
sudo dnf install gcc pkg-config openssl-devel

# 编译
cargo build --release
```

## 使用示例

### 基本命令

```bash
# 列出所有 worktree（表格格式）
./target/release/work list

# 列出所有 worktree（JSON 格式）
./target/release/work list -o json

# 切换到 worktree
./target/release/work switch feature-auth

# 获取 worktree 路径（供 shell 使用）
./target/release/work switch feature-auth --print-path
```

### Shell 集成

#### Bash/Zsh

添加到 `~/.bashrc` 或 `~/.zshrc`:

```bash
# Work tree 切换函数
workcd() {
    local path=$(./target/release/work switch "$@" --print-path 2>/dev/null)
    if [ $? -eq 0 ] && [ -n "$path" ]; then
        cd "$path"
        echo "已切换到: $(git branch --show-current)"
    else
        echo "切换失败"
        return 1
    fi
}

# 别名
alias ws=workcd
alias work=./target/release/work

# Tab 补全
_work_completion() {
    local cur_word="${COMP_WORDS[COMP_CWORD]}"
    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=($(compgen -W "list switch create delete info prune" -- "$cur_word"))
    elif [ "$COMP_CWORD" -eq 2 ]; then
        local cmd="${COMP_WORDS[1]}"
        if [ "$cmd" = "switch" ] || [ "$cmd" = "delete" ] || [ "$cmd" = "info" ]; then
            COMPREPLY=($(./target/release/work list -o json 2>/dev/null | jq -r '.worktrees[].name' | grep "$cur_word"))
        fi
    fi
}

complete -F _work_completion work
```

重新加载配置：

```bash
source ~/.bashrc  # 或 source ~/.zshrc
```

使用：

```bash
# 查看所有 worktree
work list

# 切换到 worktree
ws feature-auth

# 或使用完整命令
workcd feature-auth
```

### 常见使用场景

#### 场景 1: 查看所有 worktree

```bash
./target/release/work list
```

输出示例：
```
NAME          BRANCH          PATH                                    CURRENT  STATUS
main          main            /Users/developer/project                 *        Healthy
feature-auth  feature-auth    /Users/developer/project/worktrees/feature-auth           Healthy
bug-fix-123   bug-fix-123     /Users/developer/project/worktrees/bug-fix-123            Modified
```

#### 场景 2: 切换到 worktree

```bash
# 直接切换
./target/release/work switch feature-auth

# Shell 集成切换（自动 cd）
eval "$(./target/release/work switch feature-auth --print-path)"
```

#### 场景 3: 查看 worktree 详情

```bash
./target/release/work info feature-auth
```

输出示例：
```
Worktree: feature-auth
  Branch: feature-auth
  Path: /Users/developer/project/worktrees/feature-auth
  HEAD: a1b2c3d4e5f6789012345678901234567890abcd
  Current: No
  Detached: No
  Upstream: origin/feature-auth
  Last Modified: 2026-01-10 15:30:00
```

#### 场景 4: JSON 输出（用于脚本）

```bash
# 获取所有 worktree 信息
./target/release/work list -o json

# 解析 JSON
./target/release/work list -o json | jq '.worktrees[] | {name, branch, is_current}'

# 切换到第一个非当前 worktree
NEXT_WORKTREE=$(./target/release/work list -o json | jq -r '.worktrees[] | select(.is_current == false) | .name | .')
eval "$(./target/release/work switch $NEXT_WORKTREE --print-path)"
```

### VS Code 集成

创建 VS Code 任务 `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "work: List Worktrees",
      "type": "shell",
      "command": "./target/release/work list"
    },
    {
      "label": "work: Switch Worktree",
      "type": "shell",
      "command": "./target/release/work switch",
      "problemMatcher": []
    }
  ]
}
```

### Git 别名

添加到 `.gitconfig`:

```bash
git config --global alias.wt '!./target/release/work list'
git config --global alias.ws '!f() { ./target/release/work switch "$@" && cd "$(./target/release/work switch "$@" --print-path)"; }; f'
```

使用：

```bash
git wt  # 列出 worktree
git ws feature-auth  # 切换 worktree
```

## 开发工作流

### 编译并测试

```bash
# 1. 编译
OPENSSL_DIR=$(brew --prefix openssl) cargo build --release

# 2. 运行
./target/release/work list

# 3. 快速测试（使用当前 Git 仓库）
./target/release/work info main
```

### 开发模式

```bash
# 使用 debug 模式编译（更快）
OPENSSL_DIR=$(brew --prefix openssl) cargo build

# 运行 debug 版本
./target/debug/work list
```

## 故障排除

### 问题 1: OpenSSL 找不到

**错误**: `Could not find directory of OpenSSL installation`

**解决**:
```bash
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
export PKG_CONFIG_PATH=$(brew --prefix openssl)/lib/pkgconfig
cargo build --release
```

### 问题 2: git2 版本冲突

**错误**: `failed to select a version for git2`

**解决**: 更新 Cargo.toml 中的 git2 版本：
```toml
git2 = "0.19"  # 使用最新稳定版
```

### 问题 3: 权限错误

**错误**: `Permission denied`

**解决**:
```bash
# 给二进制文件添加执行权限
chmod +x target/release/work

# 或者直接使用 rust 运行
cargo run --release -- list
```

## 性能优化

### 使用编译时优化

项目已配置 release 优化：
- LTO (Link Time Optimization)
- 代码生成单元 = 1
- 符号剥离

编译后的二进制文件大小应该 < 5MB。

### 性能测试

```bash
# 测试启动时间
time ./target/release/work --version

# 测试列表性能（在大型仓库）
time ./target/release/work list
```

## 下一步

完成 MVP 后，您可以：

1. **测试功能** - 在实际的 Git 仓库中测试所有命令
2. **提交代码** - 创建 Git 提交
3. **继续开发** - 实现剩余功能（创建、删除 worktree）
4. **发布** - 发布到 GitHub 和 crates.io

## 需要帮助？

- 查看项目 README.md
- 运行 `./target/release/work --help`
- 查看源代码文档
