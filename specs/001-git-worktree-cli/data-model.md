# Data Model: Git Worktree 管理工具

**Feature**: Git Worktree 管理工具
**Date**: 2026-01-10
**Phase**: Phase 1 - Design & Contracts

## Overview

本文档定义 Git Worktree 管理工具的核心数据实体和它们之间的关系。数据模型设计遵循简洁性和可扩展性原则，支持所有功能需求（FR-001 至 FR-012）。

## Core Entities

### Worktree

**描述**: Git worktree 的概念表示，包含所有必要的属性和状态信息。

**字段**:

| 字段名 | 类型 | 描述 | 可选 | 验证规则 |
|--------|------|------|------|----------|
| `name` | String | Worktree 名称（通常基于分支名） | 否 | 非空，符合 Git 引用命名规则 |
| `branch` | String | 分支名 | 否 | 有效的 Git 分支名 |
| `path` | PathBuf | Worktree 目录的绝对路径 | 否 | 必须存在且可访问 |
| `is_current` | bool | 是否为当前 worktree（shell 所在目录） | 否 | - |
| `is_bare` | bool | 是否为裸仓库（主仓库通常为 bare） | 否 | - |
| `is_detached` | bool | 是否处于分离 HEAD 状态 | 否 | - |
| `head_commit` | Option<String> | HEAD 提交的 SHA | 是 | 有效的 40 字符 SHA 或 7 字符短 SHA |
| `upstream_branch` | Option<String> | 上游跟踪分支（如 `origin/main`） | 是 | 有效的远程分支引用 |
| `uncommitted_changes` | ChangeSet | 未提交的更改集合 | 是 | - |
| `last_modified` | DateTime< Utc> | 最后修改时间（基于文件系统或 HEAD 提交时间） | 否 | - |

**示例**:

```json
{
  "name": "feature-auth",
  "branch": "feature-auth",
  "path": "/home/user/project/worktrees/feature-auth",
  "is_current": false,
  "is_bare": false,
  "is_detached": false,
  "head_commit": "a1b2c3d4e5f6789012345678901234567890abcd",
  "upstream_branch": "origin/feature-auth",
  "uncommitted_changes": {
    "modified": ["src/auth.rs", "tests/auth_tests.rs"],
    "staged": [],
    "untracked": ["notes.txt"]
  },
  "last_modified": "2026-01-10T15:30:00Z"
}
```

**关系**:
- 一个 `Worktree` 属于一个 `Repository`
- 一个 `Worktree` 关联一个 `Branch`
- 一个 `Worktree` 包含零个或多个 `ChangeSet`

### Branch

**描述**: Git 分支引用，包含本地分支、远程分支和提交信息。

**字段**:

| 字段名 | 类型 | 描述 | 可选 | 验证规则 |
|--------|------|------|------|----------|
| `name` | String | 分支名（短名称，如 `main`） | 否 | 非空，符合 Git 引用命名规则 |
| `is_remote` | bool | 是否为远程分支 | 否 | - |
| `remote_name` | Option<String> | 远程名称（如 `origin`） | 是（如果是远程分支则为否） | 有效的远程名称 |
| `ref_name` | String | 完整引用名称（如 `refs/heads/main`） | 否 | 有效的 Git 引用 |
| `head_commit` | String | HEAD 提交的 SHA | 否 | 有效的 40 字符 SHA |
| `commit_date` | DateTime<Utc> | 最后提交时间 | 否 | - |
| `commit_message` | String | 提交消息（首行） | 否 | 非空 |
| `author` | String | 提交者名称 | 否 | 非空 |

**示例**:

```json
{
  "name": "main",
  "is_remote": false,
  "remote_name": null,
  "ref_name": "refs/heads/main",
  "head_commit": "a1b2c3d4e5f6789012345678901234567890abcd",
  "commit_date": "2026-01-10T14:20:00Z",
  "commit_message": "Add initial worktree support",
  "author": "Developer Name"
}
```

**关系**:
- 一个 `Branch` 可以关联多个 `Worktree`（虽然通常每个分支只有一个 worktree）

### ChangeSet

**描述**: 未提交的更改集合，包括已修改、已暂存和未跟踪的文件。

**字段**:

| 字段名 | 类型 | 描述 | 可选 | 验证规则 |
|--------|------|------|------|----------|
| `modified` | Vec<PathBuf> | 已修改但未暂存的文件 | 否 | 相对于 worktree 根目录的路径 |
| `staged` | Vec<PathBuf> | 已暂存但未提交的文件 | 否 | 相对于 worktree 根目录的路径 |
| `untracked` | Vec<PathBuf> | 未跟踪的文件 | 否 | 相对于 worktree 根目录的路径 |

**验证规则**:
- 所有路径必须相对于 worktree 根目录
- 路径必须是有效的 UTF-8 字符串
- 路径不得包含 `..` 或绝对路径（安全考虑）

**示例**:

```json
{
  "modified": ["src/auth.rs", "tests/auth_tests.rs"],
  "staged": ["README.md"],
  "untracked": ["notes.txt", ".env.local"]
}
```

**关系**:
- 一个 `ChangeSet` 属于一个 `Worktree`

### Repository

**描述**: Git 仓库的抽象，表示主仓库（bare 仓库）及其元数据。

**字段**:

| 字段名 | 类型 | 描述 | 可选 | 验证规则 |
|--------|------|------|------|----------|
| `root_path` | PathBuf | 仓库根目录的绝对路径（`.git` 目录所在位置） | 否 | 必须存在且包含 `.git` 目录 |
| `is_bare` | bool | 是否为裸仓库 | 否 | - |
| `worktree_count` | usize | 关联的 worktree 数量 | 否 | >= 0 |
| `default_branch` | String | 默认分支名（通常是 `main` 或 `master`） | 否 | 非空 |
| `current_worktree` | Option<String> | 当前 worktree 名称（基于 shell cwd） | 是 | 必须是有效的 worktree 名称 |

**示例**:

```json
{
  "root_path": "/home/user/project",
  "is_bare": false,
  "worktree_count": 5,
  "default_branch": "main",
  "current_worktree": "feature-auth"
}
```

**关系**:
- 一个 `Repository` 包含零个或多个 `Worktree`
- 一个 `Repository` 包含多个 `Branch`

## Entity Relationships

```
Repository (1) ----< (0..*) Worktree
    |                        |
    |                        +----> (1) ChangeSet
    |
    +----> (0..*) Branch
```

**说明**:
- 一个仓库可以有多个 worktree
- 一个仓库可以有多个分支
- 每个 worktree 必须关联一个分支
- 每个 worktree 可以有一个更改集合（如果有未提交的更改）

## State Transitions

### Worktree Lifecycle

```
[New]
  |
  v
[Healthy] <----> [Modified]
  |                  |
  v                  v
[Stale] ---------> [Orphaned]
  |
  v
[Deleted]
```

**状态说明**:

| 状态 | 描述 | 进入条件 | 退出条件 |
|------|------|----------|----------|
| `New` | 新创建的 worktree | `git worktree add` | 第一次查询状态 |
| `Healthy` | 无未提交更改 | 无更改或所有更改已提交 | 文件被修改 |
| `Modified` | 有未提交更改 | 文件被修改或暂存 | 更改被提交或丢弃 |
| `Stale` | 目录不存在但注册仍在 | 目录被手动删除 | 清理或重建 |
| `Orphaned` | 注册不在但目录存在 | 手动删除注册 | 清理或重新注册 |
| `Deleted` | 已删除 | `git worktree prune` | N/A |

## Validation Rules

### Worktree 验证

1. **名称验证**:
   ```rust
   fn validate_worktree_name(name: &str) -> Result<()> {
       // 不能为空
       if name.is_empty() {
           return Err(anyhow!("Worktree name cannot be empty"));
       }

       // 不能包含特殊字符（除了 - 和 _）
       if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
           return Err(anyhow!("Worktree name contains invalid characters"));
       }

       // 不能以 . 开头
       if name.starts_with('.') {
           return Err(anyhow!("Worktree name cannot start with '.'"));
       }

       // 不能是 Git 保留名称（如 HEAD, FETCH_HEAD 等）
       const RESERVED_NAMES: &[&str] = &["HEAD", "FETCH_HEAD", "ORIG_HEAD", "MERGE_HEAD"];
       if RESERVED_NAMES.contains(&name) {
           return Err(anyhow!("Worktree name is reserved by Git"));
       }

       Ok(())
   }
   ```

2. **路径验证**:
   ```rust
   fn validate_worktree_path(path: &Path, repo_root: &Path) -> Result<()> {
       // 必须是绝对路径
       if !path.is_absolute() {
           return Err(anyhow!("Worktree path must be absolute"));
       }

       // 不能在主仓库目录内
       if path.starts_with(repo_root) {
           return Err(anyhow!("Worktree cannot be inside main repository"));
       }

       // 必须存在（如果已创建）
       if path.exists() {
           // 必须是目录
           if !path.is_dir() {
               return Err(anyhow!("Worktree path must be a directory"));
           }

           // 必须包含 `.git` 文件（worktree 元数据）
           let git_file = path.join(".git");
           if !git_file.exists() {
               return Err(anyhow!("Path is not a valid worktree"));
           }
       }

       Ok(())
   }
   ```

### Branch 验证

```rust
fn validate_branch_name(name: &str) -> Result<()> {
    // 不能为空
    if name.is_empty() {
        return Err(anyhow!("Branch name cannot be empty"));
    }

    // 不能以 - 开头或结尾
    if name.starts_with('-') || name.ends_with('-') {
        return Err(anyhow!("Branch name cannot start or end with '-'"));
    }

    // 不能包含连续的 ..
    if name.contains("..") {
        return Err(anyhow!("Branch name cannot contain '..'"));
    }

    // 不能包含特殊字符（除了 /, -, _, .）
    for c in name.chars() {
        if !c.is_alphanumeric() && !matches!(c, '/' | '-' | '_' | '.') {
            return Err(anyhow!("Branch name contains invalid character: '{}'", c));
        }
    }

    Ok(())
}
```

## Error Handling

### 错误类型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorktreeError {
    #[error("Worktree not found: {0}")]
    NotFound(String),

    #[error("Worktree already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid worktree name: {0}")]
    InvalidName(String),

    #[error("Invalid worktree path: {0}")]
    InvalidPath(String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Worktree has uncommitted changes")]
    UncommittedChanges,

    #[error("Cannot delete current worktree")]
    CannotDeleteCurrent,

    #[error("Not a git repository: {0}")]
    NotGitRepository(PathBuf),
}

pub type Result<T> = std::result::Result<T, WorktreeError>;
```

## Performance Considerations

1. **延迟加载**: 只在需要时查询 `ChangeSet`（文件操作较慢）
2. **并行查询**: 使用 `rayon` 并行化多个 worktree 的状态查询
3. **缓存**: 缓存 `Repository` 对象，避免重复打开

```rust
use rayon::prelude::*;

// 并行查询所有 worktree 状态
let worktree_statuses: Vec<(String, WorktreeStatus)> = worktrees
    .into_par_iter()
    .map(|name| {
        let status = query_worktree_status(&repo, &name)?;
        Ok((name, status))
    })
    .collect::<Result<Vec<_>>>()?;
```

## Next Steps

1. 实现 Rust 结构体和枚举
2. 编写单元测试（验证规则）
3. 集成 git2 库进行 Git 操作
4. 编写集成测试（使用临时 Git 仓库）
