# CLI 命令契约: 分支名斜杠转换

**Feature**: 003-branch-slash-conversion
**Date**: 2026-01-12
**Version**: 0.2.0

## Overview

本文档定义了 CLI 命令的输入/输出契约，包括命令参数、退出码和输出格式。

---

## Command: `work add <branch>`

创建一个新的 worktree，自动将分支名中的 `/` 转换为 `-` 作为目录名。

### Parameters

| Parameter | Type | Required | Description | Example |
|-----------|------|----------|-------------|---------|
| `branch` | string | Yes | Git 分支名（可能包含 `/`） | `feat/feature-001` |

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-f, --force` | flag | false | 强制创建（即使有未提交的更改） |
| `-b, --branch <name>` | string | auto | 指定分支名（与位置参数二选一） |
| `-d, --detach` | flag | false | 创建分离的 HEAD |

### Exit Codes

| Code | Meaning | Condition |
|------|---------|-----------|
| 0 | Success | Worktree 创建成功 |
| 1 | General Error | 未指定的错误 |
| 2 | Invalid Argument | 分支名无效或为空 |
| 3 | Conflict | 目录名冲突 |
| 4 | Git Error | Git 命令失败 |

### Success Output (stdout)

```
Created worktree at /path/to/repo.worktrees/feat-feature-001
Branch: feat/feature-001
Directory: feat-feature-001
```

### Error Output (stderr)

**Case 1: 目录名冲突**
```
Error: Cannot create worktree - directory name conflict

The branch 'feat/new-feature' would create directory 'feat-new-feature',
which conflicts with existing worktree for branch 'feat/new-feature'.

Suggested solutions:
  1. Use a different branch name
  2. Delete the existing worktree with: work delete feat-new-feature
```

**Case 2: 无效分支名**
```
Error: Invalid branch name ''

Branch name cannot be empty or only slashes.
```

**Case 3: Git 错误**
```
Error: Git command failed: fatal: not a valid branch name: 'invalid@branch'
```

### Examples

```bash
# 创建包含斜杠的分支
$ work add feat/feature-001
Created worktree at /path/to/repo.worktrees/feat-feature-001
Branch: feat/feature-001
Directory: feat-feature-001

# 创建多级斜杠分支
$ work add feature/auth/oauth
Created worktree at /path/to/repo.worktrees/feature-auth-oauth
Branch: feature/auth/oauth
Directory: feature-auth-oauth

# 创建无斜杠分支（无转换）
$ work add main
Created worktree at /path/to/repo.worktrees/main
Branch: main
Directory: main

# 尝试创建冲突的 worktree
$ work add feat/feature-001
Error: Cannot create worktree - directory name conflict
...
```

---

## Command: `work list`

列出所有 worktree，显示目录名和分支名的对应关系。

### Parameters

无位置参数。

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-o, --output <format>` | string | compact | 输出格式: compact, table, json |
| `-p, --porcelain` | flag | false | 机器可读格式（别名 for json） |

### Exit Codes

| Code | Meaning | Condition |
|------|---------|-----------|
| 0 | Success | 列表输出成功 |
| 1 | General Error | 未指定的错误 |
| 2 | Git Error | Git 命令失败 |

### Output Formats

#### Compact Format (default)

```
*⌂  worktree on 003-branch-slash-conversion (modified)
  feat-feature-001 on feat/feature-001
  feat-auth-oauth on feature/auth/oauth
  main
```

**Legend**:
- `*` = 当前 worktree
- `⌂` = 主工作目录
- `dirname on branch` = 目录名与分支名不同时显示
- `(modified)` = 有未提交的更改（红色）
- `(detached)` = HEAD 分离（黄色）

#### Table Format

```
┌─────────────────────┬──────────────────────────────┬─────────┬──────────┐
│ Directory           │ Branch                        │ Status  │ Head     │
├─────────────────────┼──────────────────────────────┼─────────┼──────────┤
│ ⌂ worktree          │ 003-branch-slash-conversion   │ modified│ abc1234  │
│ feat-feature-001    │ feat/feature-001              │ clean   │ def5678  │
│ feat-auth-oauth     │ feature/auth/oauth            │ clean   │ ghi9012  │
│ main                │ main                         │ clean   │ jkl3456  │
└─────────────────────┴──────────────────────────────┴─────────┴──────────┘
```

**Legend**:
- `⌂` = 主工作目录
- `*` = 当前 worktree（在 Directory 列前显示）
- `Status`: clean, modified (red), detached (yellow)

#### JSON Format

```json
[
  {
    "directory": "worktree",
    "branch": "003-branch-slash-conversion",
    "path": "/path/to/repo",
    "status": "modified",
    "commit": "abc1234",
    "is_main": true,
    "is_current": true
  },
  {
    "directory": "feat-feature-001",
    "branch": "feat/feature-001",
    "path": "/path/to/repo.worktrees/feat-feature-001",
    "status": "clean",
    "commit": "def5678",
    "is_main": false,
    "is_current": false
  },
  {
    "directory": "feat-auth-oauth",
    "branch": "feature/auth/oauth",
    "path": "/path/to/repo.worktrees/feat-auth-oauth",
    "status": "clean",
    "commit": "ghi9012",
    "is_main": false,
    "is_current": false
  },
  {
    "directory": "main",
    "branch": "main",
    "path": "/path/to/repo.worktrees/main",
    "status": "clean",
    "commit": "jkl3456",
    "is_main": false,
    "is_current": false
  }
]
```

### Examples

```bash
# 默认输出
$ work list
*⌂  worktree on 003-branch-slash-conversion (modified)
  feat-feature-001 on feat/feature-001
  main

# 表格输出
$ work list -o table
┌─────────────────────┬──────────────────────────────┬─────────┬──────────┐
│ Directory           │ Branch                        │ Status  │ Head     │
├─────────────────────┼──────────────────────────────┼─────────┼──────────┤
│ ⌂ worktree          │ 003-branch-slash-conversion   │ modified│ abc1234  │
│ feat-feature-001    │ feat/feature-001              │ clean   │ def5678  │
└─────────────────────┴──────────────────────────────┴─────────┴──────────┘

# JSON 输出
$ work list -o json
[
  {
    "directory": "feat-feature-001",
    "branch": "feat/feature-001",
    ...
  }
]
```

---

## Command: `work show <directory>`

显示指定 worktree 的详细信息。

### Parameters

| Parameter | Type | Required | Description | Example |
|-----------|------|----------|-------------|---------|
| `directory` | string | Yes | Worktree 目录名 | `feat-feature-001` |

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-o, --output <format>` | string | human | 输出格式: human, json |

### Exit Codes

| Code | Meaning | Condition |
|------|---------|-----------|
| 0 | Success | 显示成功 |
| 1 | General Error | 未指定的错误 |
| 2 | Not Found | Worktree 不存在 |
| 3 | Git Error | Git 命令失败 |

### Success Output (stdout)

**Human Format**:
```
Worktree: feat-feature-001
Branch: feat/feature-001
Path: /path/to/repo.worktrees/feat-feature-001
Status: clean
HEAD: def5678 Add new feature (2 hours ago)
Is Main: No
Is Current: No
```

**JSON Format**:
```json
{
  "directory": "feat-feature-001",
  "branch": "feat/feature-001",
  "path": "/path/to/repo.worktrees/feat-feature-001",
  "status": "clean",
  "commit": "def5678",
  "commit_message": "Add new feature",
  "commit_author": "John Doe",
  "commit_date": "2026-01-12T10:30:00Z",
  "is_main": false,
  "is_current": false
}
```

### Error Output (stderr)

**Case 1: Worktree 不存在**
```
Error: Worktree not found: 'unknown-directory'

Available worktrees:
  - feat-feature-001 (branch: feat/feature-001)
  - main (branch: main)
```

### Examples

```bash
# 显示包含斜杠的 worktree
$ work show feat-feature-001
Worktree: feat-feature-001
Branch: feat/feature-001
Path: /path/to/repo.worktrees/feat-feature-001
Status: clean
HEAD: def5678 Add new feature (2 hours ago)
Is Main: No
Is Current: No

# JSON 输出
$ work show feat-feature-001 -o json
{
  "directory": "feat-feature-001",
  "branch": "feat/feature-001",
  ...
}
```

---

## Command: `work delete <directory>`

删除指定的 worktree。

### Parameters

| Parameter | Type | Required | Description | Example |
|-----------|------|----------|-------------|---------|
| `directory` | string | Yes | Worktree 目录名 | `feat-feature-001` |

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-f, --force` | flag | false | 强制删除（即使有未提交的更改） |

### Exit Codes

| Code | Meaning | Condition |
|------|---------|-----------|
| 0 | Success | 删除成功 |
| 1 | General Error | 未指定的错误 |
| 2 | Not Found | Worktree 不存在 |
| 3 | Uncommitted Changes | 有未提交的更改（未使用 --force） |
| 4 | Current Worktree | 不能删除当前 worktree |
| 5 | Git Error | Git 命令失败 |

### Success Output (stdout)

```
Deleted worktree: feat-feature-001
Branch: feat/feature-001
Path: /path/to/repo.worktrees/feat-feature-001
```

### Error Output (stderr)

**Case 1: Worktree 不存在**
```
Error: Worktree not found: 'unknown-directory'

Available worktrees:
  - feat-feature-001 (branch: feat/feature-001)
  - main (branch: main)
```

**Case 2: 未提交的更改**
```
Error: Cannot delete worktree with uncommitted changes

Worktree 'feat-feature-001' has uncommitted changes.

Please commit or stash your changes first, or use --force to delete anyway.
```

**Case 3: 当前 worktree**
```
Error: Cannot delete current worktree

You are currently in the worktree 'feat-feature-001'.
Please switch to another worktree before deleting this one.
```

### Examples

```bash
# 删除 worktree
$ work delete feat-feature-001
Deleted worktree: feat-feature-001
Branch: feat/feature-001
Path: /path/to/repo.worktrees/feat-feature-001

# 强制删除
$ work delete feat-feature-001 --force
Deleted worktree: feat-feature-001 (forced)
Branch: feat/feature-001
Path: /path/to/repo.worktrees/feat-feature-001
```

---

## Core Functions (Internal API)

### `branch_to_dirname(branch_name: &str) -> String`

将分支名转换为目录名（将 `/` 替换为 `-`）。

**Input**: `feat/feature-001`
**Output**: `feat-feature-001`

**Test Cases**:
- `main` → `main`
- `feat/feature-001` → `feat-feature-001`
- `feature/auth/oauth` → `feature-auth-oauth`
- `/feat` → `-feat` (Git 拒绝此分支名)
- `feat/` → `feat-` (Git 拒绝此分支名)
- `feat///feature` → `feat---feature` (Git 可能允许)

### `validate_dirname(dirname: &str) -> Result<(), WorktreeError>`

验证目录名是否合法。

**Rules**:
- 不能为空
- 不能包含路径分隔符（`/` 或 `\`）
- 不能以 `.` 开头

**Error Cases**:
- `` → `InvalidBranchName("Directory name cannot be empty")`
- `feat/feature` → `InvalidBranchName("Directory name cannot contain path separators")`
- `.hidden` → `InvalidBranchName("Directory name cannot start with a dot")`

### `check_dirname_conflict(dirname: &str, existing_worktrees: &[Worktree]) -> Result<(), WorktreeError>`

检查目录名是否与现有 worktree 冲突。

**Success**: 无冲突
**Error**: `DirNameConflict { dirname, existing_branch }`

**Example**:
```rust
check_dirname_conflict("feat-feature-001", &worktrees)
// Ok(())

check_dirname_conflict("feat-feature-001", &existing_worktrees)
// Err(DirNameConflict {
//     dirname: "feat-feature-001",
//     existing_branch: "feat/feature-001"
// })
```

---

## Version Compatibility

| Feature Version | CLI Version | Breaking Changes |
|-----------------|-------------|------------------|
| 0.1.x | 0.1.x | N/A |
| 0.2.0 | 0.2.0 | **Yes**: `work list` 输出格式变更（添加 `on` 分隔符） |

---

## Migration Guide

### From 0.1.x to 0.2.0

**Breaking Changes**:
1. `work list` 输出格式变更：`dirname on branch` 替代仅 `dirname`
2. JSON 输出添加 `directory` 字段（替代 `name`）

**Action Required**:
- 更新解析 JSON 输出的脚本：使用 `directory` 替代 `name`
- 更新期望 compact 格式的测试：添加 `on` 分隔符处理

**Backward Compatibility**:
- 对于无斜杠的分支，`dirname == branch_name`，输出格式保持不变
- Table 格式添加了 Branch 列，但信息更清晰

---

## Testing Contracts

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_to_dirname_simple() {
        assert_eq!(branch_to_dirname("main"), "main");
    }

    #[test]
    fn test_branch_to_dirname_with_slash() {
        assert_eq!(branch_to_dirname("feat/feature-001"), "feat-feature-001");
    }

    #[test]
    fn test_branch_to_dirname_multiple_slashes() {
        assert_eq!(branch_to_dirname("feature/auth/oauth"), "feature-auth-oauth");
    }

    #[test]
    fn test_validate_dirname_empty() {
        assert!(validate_dirname("").is_err());
    }

    #[test]
    fn test_validate_dirname_with_slash() {
        assert!(validate_dirname("feat/feature").is_err());
    }
}
```

### Integration Tests

```bash
#!/bin/bash
# tests/integration/worktree_add.sh

setup_test_repo() {
    git init test_repo
    cd test_repo
    git config user.email "test@example.com"
    git config user.name "Test User"
    echo "test" > README.md
    git add README.md
    git commit -m "Initial commit"
}

test_add_slash_branch() {
    work add feat/feature-001
    assert_equal $? 0

    # 验证目录创建
    assert [ -d "../test_repo.worktrees/feat-feature-001" ]

    # 验证分支正确
    cd ../test_repo.worktrees/feat-feature-001
    assert_equal "$(git branch --show-current)" "feat/feature-001"
}

test_add_conflict() {
    work add feat/feature-001
    work add feat/feature-001 2>&1 | grep "directory name conflict"
    assert_equal $? 0
}
```

---

## Conclusion

所有命令契约已定义，包括输入参数、输出格式和错误处理。实现时应严格遵守这些契约，确保向后兼容性和可预测性。
