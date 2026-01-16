# Data Model: Worktree Branch Source Selection

**Feature**: 005-worktree-branch-source
**Date**: 2025-01-16
**Purpose**: 定义本功能涉及的核心数据结构和实体

## 概述

本功能不引入新的持久化数据存储，所有数据都是临时的运行时状态。主要涉及枚举类型、错误类型和 Git 元数据结构。

## 核心数据结构

### 1. BranchSource (新增枚举)

**位置**: `src/core/git_ops.rs` 或新建 `src/core/branch_source.rs`

**描述**: 表示创建 worktree 时可以选择的分支来源

```rust
/// 分支来源选项
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchSource {
    /// 基于当前工作目录所在的分支
    Current {
        /// 当前分支名称
        branch_name: String,
    },

    /// 基于主仓库（main repository）所在的分支
    Main {
        /// 主目录路径
        main_repo_path: PathBuf,
        /// 主目录当前分支名称
        branch_name: String,
    },

    /// 自定义分支名称
    Custom {
        /// 分支名称（可以是本地分支或远程分支）
        branch_name: String,
        /// 是否为远程分支（如 origin/feature）
        is_remote: bool,
    },
}
```

**字段说明**:

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `Current.branch_name` | `String` | 是 | 当前工作目录的 Git 分支名称 |
| `Main.main_repo_path` | `PathBuf` | 是 | 主仓库的绝对路径 |
| `Main.branch_name` | `String` | 是 | 主仓库当前的 Git 分支名称 |
| `Custom.branch_name` | `String` | 是 | 用户指定的分支名称 |
| `Custom.is_remote` | `bool` | 是 | 标识是否为远程分支（包含 `origin/` 前缀） |

**方法**:

```rust
impl BranchSource {
    /// 获取分支名称（用于创建 worktree）
    pub fn branch_name(&self) -> &str {
        match self {
            BranchSource::Current { branch_name } => branch_name,
            BranchSource::Main { branch_name, .. } => branch_name,
            BranchSource::Custom { branch_name, .. } => branch_name,
        }
    }

    /// 获取描述性标签（用于显示）
    pub fn label(&self) -> String {
        match self {
            BranchSource::Current { branch_name } => {
                format!("基于当前目录分支 ({})", branch_name)
            }
            BranchSource::Main { branch_name, .. } => {
                format!("基于主目录分支 ({})", branch_name)
            }
            BranchSource::Custom { branch_name, .. } => {
                format!("自定义分支 ({})", branch_name)
            }
        }
    }

    /// 从字符串解析分支来源（用于命令行参数）
    pub fn from_str(input: &str) -> Result<Self, WorktreeError> {
        match input.to_lowercase().as_str() {
            "current" => {
                // 需要在运行时获取当前分支
                let branch_name = get_current_branch()?;
                Ok(BranchSource::Current { branch_name })
            }
            "main" => {
                // 需要在运行时获取主目录和分支
                let (main_repo_path, branch_name) = get_main_repo_branch()?;
                Ok(BranchSource::Main { main_repo_path, branch_name })
            }
            custom => {
                // 自定义分支名
                let is_remote = custom.starts_with("origin/");
                Ok(BranchSource::Custom {
                    branch_name: custom.to_string(),
                    is_remote,
                })
            }
        }
    }
}
```

**验证规则**:

1. **Current**: 必须在 Git 仓库中，且不能是 detached HEAD 状态
2. **Main**: 主目录必须存在，且不能是 detached HEAD 状态
3. **Custom**: 分支必须存在于本地或远程

## 错误类型

### 2. WorktreeError 扩展

**位置**: `src/utils/errors.rs`

**描述**: 扩展现有错误枚举，添加本功能特有的错误类型

```rust
/// 在现有 WorktreeError 枚举中添加新变体
pub enum WorktreeError {
    // ... 现有错误变体 ...

    /// 主目录处于 detached HEAD 状态
    MainRepoDetachedHead {
        /// 主目录路径
        main_repo_path: String,
        /// 当前 HEAD 指向的 commit SHA
        commit_sha: String,
    },

    /// 当前目录处于 detached HEAD 状态（当选择"基于当前分支"时）
    CurrentDirDetachedHead {
        /// 当前目录路径
        current_path: String,
        /// 当前 HEAD 指向的 commit SHA
        commit_sha: String,
    },

    /// 分支不存在（本地和远程都找不到）
    BranchNotFound {
        /// 用户输入的分支名称
        branch_name: String,
        /// 可用的本地分支列表
        available_locals: Vec<String>,
        /// 可用的远程分支列表
        available_remotes: Vec<String>,
    },

    /// 无效的分支来源选项（命令行参数解析失败）
    InvalidBranchSource {
        /// 用户输入的值
        input: String,
    },
}
```

**错误展示格式**:

```text
错误: 主目录处于 detached HEAD 状态

主目录路径: /path/to/main/repo
当前 HEAD: abc1234def5678...

原因: detached HEAD 状态没有分支名称，无法作为分支来源

建议的解决方案:
1. 切换到一个分支: cd /path/to/main/repo && git checkout <branch>
2. 选择其他分支来源（当前分支、自定义分支）
3. 先在主目录创建或切换分支，然后重试
```

## Git 元数据结构

### 3. BranchInfo (新增结构体)

**位置**: `src/core/git_ops.rs`

**描述**: 封装分支的元数据信息

```rust
/// 分支信息
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchInfo {
    /// 分支名称（短格式，不含 refs/heads/）
    pub name: String,
    /// 是否为远程分支
    pub is_remote: bool,
    /// 远程名称（仅远程分支有效，如 "origin"）
    pub remote: Option<String>,
    /// 上游分支（如果已设置跟踪）
    pub upstream: Option<String>,
    /// 最后提交的 SHA
    pub head_sha: Option<String>,
}

impl BranchInfo {
    /// 从分支引用字符串解析（如 "refs/heads/main" 或 "origin/feature"）
    pub fn from_ref(ref_str: &str) -> Option<Self> {
        // 解析逻辑
        // "refs/heads/main" -> BranchInfo { name: "main", is_remote: false, ... }
        // "refs/remotes/origin/main" -> BranchInfo { name: "main", is_remote: true, remote: Some("origin"), ... }
        // "origin/feature" -> BranchInfo { name: "feature", is_remote: true, remote: Some("origin"), ... }
    }
}
```

## 配置数据

### 4. CreateOptions (扩展现有结构)

**位置**: `src/main.rs` (在 `create_command_handler` 函数中)

**描述**: 封装 `work create` 命令的所有选项

```rust
/// work create 命令的选项
struct CreateOptions {
    /// worktree 名称
    name: String,

    /// 自定义路径（可选）
    path: Option<String>,

    /// 是否启用交互模式
    interactive: bool,

    /// 分支来源（新增）
    branch_source: Option<BranchSource>,

    /// 旧版参数：基准分支（向后兼容）
    legacy_branch: Option<String>,
}
```

**字段说明**:

| 字段 | 类型 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `name` | `String` | 是 | - | 新 worktree 的名称 |
| `path` | `Option<String>` | 否 | `None` | 自定义路径，None 时使用默认路径模式 |
| `interactive` | `bool` | 否 | `false` | 是否启用交互式选择 |
| `branch_source` | `Option<BranchSource>` | 否 | `None` | 显式指定的分支来源（命令行参数） |
| `legacy_branch` | `Option<String>` | 否 | `None` | 旧版 `--branch` 参数，向后兼容 |

**解析优先级**:

1. 如果指定 `legacy_branch` (旧版 `--branch`)，直接使用该分支
2. 否则如果指定 `branch_source` (新版 `--branch-source`)，使用指定的来源
3. 否则如果启用 `interactive`，显示交互式菜单
4. 否则默认使用 `BranchSource::Current`

## 数据流图

### 创建 worktree 的数据流

```text
用户输入
   ↓
命令行解析 (clap)
   ↓
CreateOptions 结构体
   ↓
解析分支来源
   ├─ 显式指定 → 直接使用
   ├─ 交互模式 → 显示菜单 → 用户选择
   └─ 默认值 → BranchSource::Current
   ↓
验证分支来源
   ├─ Current: 检查当前目录分支状态
   ├─ Main: 获取主目录分支并检查状态
   └─ Custom: 验证分支存在性（本地/远程）
   ↓
执行 Git 命令
   ├─ git worktree add <path> <branch>
   └─ (远程分支) 自动设置跟踪
   ↓
返回结果
```

### 错误处理数据流

```text
验证失败
   ↓
生成 WorktreeError
   ├─ MainRepoDetachedHead
   ├─ CurrentDirDetachedHead
   ├─ BranchNotFound
   └─ InvalidBranchSource
   ↓
格式化错误信息
   ├─ 彩色输出 (colored 库)
   ├─ 结构化显示
   └─ 建议/提示信息
   ↓
返回错误 (anyhow::Result)
```

## 状态转换

### BranchSource 的状态转换

```text
[初始状态]
    ↓
用户选择来源
    ↓
┌───────────┬──────────┬──────────┐
│  Current  │   Main   │  Custom  │
└───────────┴──────────┴──────────┘
      ↓           ↓          ↓
  获取当前      获取主目录    验证分支
  分支信息      分支信息    存在性
      ↓           ↓          ↓
   [验证状态]
      ↓
   所有验证通过？
    ↙      ↘
  是        否
  ↓         ↓
[可用]    [错误]
  ↓         ↓
用于创建  显示错误
worktree   信息
```

## 持久化考虑

本功能**不引入**任何持久化存储：

- 无配置文件
- 无数据库
- 无缓存
- 无状态文件

所有状态都是临时的，仅在命令执行期间存在于内存中。

## 序列化

本功能**不需要**序列化支持（无 JSON/YAML 输入或输出）。

未来如果需要添加配置文件（如 `.workrc`），可以考虑为 `BranchSource` 实现 `serde::Serialize` 和 `serde::Deserialize`。

## 类型别名

```rust
/// 结果类型别名
type Result<T> = std::result::Result<T, WorktreeError>;
```

## 总结

本功能的核心数据模型包括：

1. **BranchSource 枚举**: 表示三种分支来源选项
2. **WorktreeError 扩展**: 添加 4 个新的错误变体
3. **BranchInfo 结构体**: 封装分支元数据（可选实现）
4. **CreateOptions 结构体**: 扩展现有命令选项

所有数据结构都是临时的、运行时的，无需持久化或序列化支持。设计重点在于：
- 清晰的类型语义
- 完整的错误处理
- 友好的用户反馈
- 向后兼容性
