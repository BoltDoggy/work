# Data Model: 分支名斜杠转换

**Feature**: 003-branch-slash-conversion
**Date**: 2026-01-12
**Status**: Complete

## Overview

本文档描述了分支名斜杠转换功能的数据模型。主要修改了 `Worktree` 实体，添加了 `dirname` 字段以支持目录名和分支名的分离。

---

## Core Entities

### Worktree

表示一个 Git worktree 的完整信息。

**Fields**:

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `dirname` | `String` | Worktree 目录名（分支名转换后） | 必填，非空，无路径分隔符 |
| `branch_name` | `String` | Git 分支名（原始名称） | 必填，非空，符合 Git 分支名规则 |
| `path` | `PathBuf` | Worktree 完整路径 | 必填，存在且可访问 |
| `status` | `WorktreeStatus` | Worktree 状态（枚举） | 必填 |
| `commit` | `String` | HEAD 提交哈希 | 必填，有效的 Git SHA |
| `is_main` | `bool` | 是否为主工作目录 | 必填 |
| `is_current` | `bool` | 是否为当前工作目录 | 必填 |

**Relationships**:
- 一个 `Worktree` 属于一个 `Repository`
- 一个 `Repository` 包含多个 `Worktree`

**State Transitions**:

```
[不存在] → [创建中] → [clean/detached/modified]
                ↓
              [删除中] → [不存在]
```

**Rust Definition**:

```rust
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    /// Worktree 目录名（分支名中的 / 替换为 -）
    pub dirname: String,

    /// Git 分支名（原始名称，可能包含 /）
    pub branch_name: String,

    /// Worktree 完整路径
    pub path: PathBuf,

    /// Worktree 状态
    pub status: WorktreeStatus,

    /// HEAD 提交哈希
    pub commit: String,

    /// 是否为主工作目录
    pub is_main: bool,

    /// 是否为当前工作目录（通过 git rev-parse --show-prefix 检测）
    pub is_current: bool,
}

impl Worktree {
    /// 从 Git worktree porcelain 输出解析
    pub fn from_git_output(lines: &[&str]) -> Result<Vec<Self>, WorktreeError> {
        // 解析逻辑...
    }

    /// 获取显示名称（用于输出）
    pub fn display_name(&self) -> String {
        if self.dirname == self.branch_name {
            self.dirname.clone()
        } else {
            format!("{} on {}", self.dirname, self.branch_name)
        }
    }
}
```

---

### WorktreeStatus

表示 worktree 的当前状态。

**Values**:

| Value | Description | Display |
|-------|-------------|---------|
| `Clean` | 工作目录干净，无未提交更改 | "clean" |
| `Modified` | 有未提交的更改 | "modified" (红色) |
| `Detached` | HEAD 处于分离状态 | "detached" (黄色) |

**Rust Definition**:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorktreeStatus {
    Clean,
    Modified,
    Detached,
}

impl WorktreeStatus {
    /// 从 Git worktree porcelain 输出解析
    pub fn from_git_flag(flag: &str) -> Option<Self> {
        match flag {
            "" => Some(Self::Clean),
            "detached" => Some(Self::Detached),
            _ => Some(Self::Modified),
        }
    }

    /// 获取显示名称（带颜色）
    pub fn display_colored(&self) -> ColoredString {
        match self {
            Self::Clean => "clean".normal(),
            Self::Modified => "modified".red(),
            Self::Detached => "detached".yellow(),
        }
    }
}
```

---

### Repository

表示 Git 仓库的元数据。

**Fields**:

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `common_dir` | `PathBuf` | Git 通用目录（.git） | 必填，存在 |
| `worktrees_dir` | `PathBuf` | Worktrees 基础目录 | 必填，存在 |
| `repo_name` | `String` | 仓库名称 | 必填，非空 |

**Rust Definition**:

```rust
#[derive(Debug, Clone)]
pub struct Repository {
    /// Git 通用目录路径
    pub common_dir: PathBuf,

    /// Worktrees 基础目录（通常为 <repo>.worktrees）
    pub worktrees_base: PathBuf,

    /// 仓库名称
    pub repo_name: String,
}

impl Repository {
    /// 从当前目录检测仓库
    pub fn detect() -> Result<Self, WorktreeError> {
        // 使用 git rev-parse --git-common-dir
    }

    /// 获取 worktree 完整路径
    pub fn get_worktree_path(&self, dirname: &str) -> PathBuf {
        self.worktrees_base.join(dirname)
    }
}
```

---

## Data Flow

### 创建 Worktree

```
[用户输入分支名]
       ↓
[branch_to_dirname()] → [目录名]
       ↓
[validate_dirname()] → [验证通过/失败]
       ↓
[check_dirname_conflict()] → [无冲突/冲突错误]
       ↓
[git worktree add <path> <branch>] → [创建成功/失败]
       ↓
[解析 git worktree list] → [Worktree 实体]
```

### 列出 Worktree

```
[gf worktree list --porcelain]
       ↓
[解析输出] → [提取 dirname, branch_name, path, status, commit]
       ↓
[构建 Worktree 实体列表]
       ↓
[格式化输出] → [compact/table/json]
```

### 删除 Worktree

```
[用户输入目录名]
       ↓
[查找 Worktree] → [找到/未找到错误]
       ↓
[检查未提交更改] → [有更改/确认删除]
       ↓
[检查是否为当前 worktree] → [是/错误]
       ↓
[git worktree remove <path>] → [删除成功/失败]
```

---

## Validation Rules

### Branch Name Validation

```rust
fn validate_branch_name(branch: &str) -> Result<(), WorktreeError> {
    // Git 的分支名规则
    if branch.is_empty() {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot be empty".to_string(),
        ));
    }

    // 不能以 . 开头
    if branch.starts_with('.') {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot start with a dot".to_string(),
        ));
    }

    // 不能以 .lock 结尾
    if branch.ends_with(".lock") {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot end with .lock".to_string(),
        ));
    }

    // 不能包含控制字符或空格
    if branch.contains(char::is_control) || branch.contains(' ') {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot contain spaces or control characters".to_string(),
        ));
    }

    // 不能包含 Git 保留字符: : ? [ ] @ { }
    let invalid_chars = ['?', '[', ']', '@', '{', '}'];
    if branch.contains(|c| invalid_chars.contains(&c)) {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name contains invalid characters".to_string(),
        ));
    }

    // 不能包含两个连续的 ..
    if branch.contains("..") {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot contain '..'".to_string(),
        ));
    }

    Ok(())
}
```

**注意**: 实际创建时，Git 会执行这些验证，所以此函数主要用于提前验证和提供友好的错误信息。

### Directory Name Validation

```rust
fn validate_dirname(dirname: &str) -> Result<(), WorktreeError> {
    // 不能为空
    if dirname.is_empty() {
        return Err(WorktreeError::InvalidBranchName(
            "Directory name cannot be empty".to_string(),
        ));
    }

    // 不能包含路径分隔符（防止创建子目录）
    if dirname.contains('/') || dirname.contains('\\') {
        return Err(WorktreeError::InvalidBranchName(
            "Directory name cannot contain path separators".to_string(),
        ));
    }

    // 不能以 . 开头（防止隐藏目录）
    if dirname.starts_with('.') {
        return Err(WorktreeError::InvalidBranchName(
            "Directory name cannot start with a dot".to_string(),
        ));
    }

    Ok(())
}
```

### Conflict Detection

```rust
fn check_dirname_conflict(
    dirname: &str,
    existing_worktrees: &[Worktree],
) -> Result<(), WorktreeError> {
    if let Some(existing) = existing_worktrees.iter().find(|w| w.dirname == dirname) {
        Err(WorktreeError::DirNameConflict {
            dirname: dirname.to_string(),
            existing_branch: existing.branch_name.clone(),
        })
    } else {
        Ok(())
    }
}
```

---

## Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum WorktreeError {
    #[error("Invalid branch name: {0}")]
    InvalidBranchName(String),

    #[error("Directory name conflict: '{dirname}' already exists for branch '{existing_branch}'")]
    DirNameConflict {
        dirname: String,
        existing_branch: String,
    },

    #[error("Worktree not found: {0}")]
    WorktreeNotFound(String),

    #[error("Git command failed: {0}")]
    GitError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Path too long: {0}")]
    PathTooLong(PathBuf),
}
```

---

## JSON Schema

用于 JSON 输出格式的 schema：

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "array",
  "items": {
    "type": "object",
    "required": ["directory", "branch", "status", "commit"],
    "properties": {
      "directory": {
        "type": "string",
        "description": "Worktree directory name (with / replaced by -)"
      },
      "branch": {
        "type": "string",
        "description": "Git branch name (may contain /)"
      },
      "status": {
        "type": "string",
        "enum": ["clean", "modified", "detached"],
        "description": "Worktree status"
      },
      "commit": {
        "type": "string",
        "description": "HEAD commit SHA"
      },
      "path": {
        "type": "string",
        "description": "Full path to worktree"
      }
    }
  }
}
```

---

## Migration from Existing Data Model

### Before

```rust
pub struct Worktree {
    pub name: String,     // 目录名或分支名（混合）
    pub path: PathBuf,
    pub status: WorktreeStatus,
    pub commit: String,
    pub is_main: bool,
    pub is_current: bool,
}
```

### After

```rust
pub struct Worktree {
    pub dirname: String,        // 目录名（明确）
    pub branch_name: String,    // 分支名（明确）
    pub path: PathBuf,
    pub status: WorktreeStatus,
    pub commit: String,
    pub is_main: bool,
    pub is_current: bool,
}
```

### Migration Strategy

1. **更新解析逻辑**: 从 `git worktree list` 输出中同时提取目录名和分支名
2. **更新显示逻辑**: 使用 `display_name()` 方法格式化输出
3. **更新命令逻辑**: 创建时使用 `branch_to_dirname()` 转换
4. **向后兼容**: 对于现有 worktree（无斜杠），`dirname == branch_name`

---

## Testing Data

### Test Case 1: Simple Branch

**Input**: `main`
**Expected**:
- `dirname = "main"`
- `branch_name = "main"`

### Test Case 2: Branch with Single Slash

**Input**: `feat/feature-001`
**Expected**:
- `dirname = "feat-feature-001"`
- `branch_name = "feat/feature-001"`

### Test Case 3: Branch with Multiple Slashes

**Input**: `feature/auth/oauth`
**Expected**:
- `dirname = "feature-auth-oauth"`
- `branch_name = "feature/auth/oauth"`

### Test Case 4: Edge Case - Leading Slash

**Input**: `/feat`
**Expected**:
- `dirname = "-feat"`
- `branch_name = "/feat"`
- **Error**: Git 拒绝此分支名

### Test Case 5: Edge Case - Trailing Slash

**Input**: `feat/`
**Expected**:
- `dirname = "feat-"`
- `branch_name = "feat/"`
- **Error**: Git 拒绝此分支名

### Test Case 6: Edge Case - Multiple Consecutive Slashes

**Input**: `feat///feature`
**Expected**:
- `dirname = "feat---feature"`
- `branch_name = "feat///feature"`
- **Note**: Git 可能允许此分支名（取决于版本）

---

## Performance Considerations

- **内存**: 每个 `Worktree` 实体约 200-300 字节（取决于路径长度）
- **解析时间**: O(n) 其中 n = worktree 数量（通常 < 100）
- **转换时间**: O(m) 其中 m = 分支名长度（通常 < 100）

---

## Security Considerations

- **路径遍历**: 通过 `validate_dirname()` 防止 `..` 和相对路径
- **命令注入**: 使用 `std::process::Command` 而非 shell 命令字符串
- **信息泄露**: JSON 输出包含完整路径，可选的敏感路径过滤

---

## Conclusion

数据模型已完整定义，支持目录名和分支名的分离。核心修改是添加 `dirname` 字段到 `Worktree` 实体，并更新相关验证和转换逻辑。
