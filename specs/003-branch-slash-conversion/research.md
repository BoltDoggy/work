# Technical Research: 分支名斜杠转换

**Feature**: 003-branch-slash-conversion
**Date**: 2026-01-12
**Status**: Complete

## Overview

本文档记录了分支名斜杠转换功能的技术决策和研究结果。所有技术"NEEDS CLARIFICATION"项已解决。

---

## Decision 1: 分支名转换算法

**决策**: 使用简单的字符串替换算法，将所有 `/` 字符替换为 `-` 字符。

**理由**:
- **简单性**: 算法简单直观，易于理解和维护
- **性能**: O(n) 时间复杂度，n 为分支名长度，性能开销可忽略
- **可预测性**: 转换结果一致且可逆（在大多数情况下）
- **无依赖**: 不需要外部库或复杂的正则表达式

**实现方案**:
```rust
fn branch_to_dirname(branch_name: &str) -> String {
    branch_name.replace('/', "-")
}
```

**考虑的替代方案**:
1. **使用正则表达式**: 引入不必要的复杂性，性能无优势
2. **使用 slugify 库**: 过度设计，引入额外依赖，且可能破坏分支名的语义
3. **仅替换第一个斜杠**: 无法处理多级分支（如 `feature/auth/oauth`）

**边界情况处理**:
- 连续多个斜杠（`feat///feature`）→ `feat---feature`（保留所有连字符）
- 斜杠在开头（`/feat`）→ `-feat`（保留前导连字符）
- 斜杠在结尾（`feat/`）→ `feat-`（保留尾随连字符）

---

## Decision 2: 冲突检测策略

**决策**: 在创建 worktree 前检测转换后的目录名是否与现有 worktree 冲突。

**理由**:
- **用户体验**: 在创建前检测冲突可以避免部分创建状态
- **安全性**: 防止覆盖现有 worktree 或 Git 元数据
- **清晰性**: 向用户提供明确的错误信息，说明冲突原因

**实现方案**:
```rust
fn check_dirname_conflict(dirname: &str, existing_worktrees: &[Worktree]) -> Result<(), WorktreeError> {
    if existing_worktrees.iter().any(|w| w.dirname == dirname) {
        return Err(WorktreeError::DirNameConflict {
            dirname: dirname.to_string(),
            existing_branch: existing_worktrees
                .iter()
                .find(|w| w.dirname == dirname)
                .map(|w| w.branch_name.clone()),
        });
    }
    Ok(())
}
```

**错误信息设计**:
```
Error: Cannot create worktree - directory name conflict

The branch 'feat/new-feature' would create directory 'feat-new-feature',
which conflicts with existing worktree for branch 'feat/new-feature'.

Suggested solutions:
  1. Use a different branch name
  2. Delete the existing worktree with: work delete feat-new-feature
```

**考虑的替代方案**:
1. **自动重命名**: 可能导致不可预测的结果，用户不知道实际目录名
2. **事后检测**: 需要回滚部分创建的 worktree，复杂且容易失败
3. **允许冲突**: 会导致 Git worktree 元数据混乱，违反 Git worktree 语义

---

## Decision 3: 输出格式化方案

**决策**: 在所有输出格式中同时显示目录名和分支名，使用 "dirname on branch" 格式。

**理由**:
- **清晰性**: 用户能清楚看到目录名和分支名的对应关系
- **一致性**: 与现有输出格式兼容，扩展而非替换
- **可调试性**: 帮助用户理解转换规则和冲突原因

**实现方案**:

### Compact 格式（默认）
```
*⌂  worktree on 003-branch-slash-conversion (modified)
  feat-feature-001 on feat/feature-001
  feat-auth-oauth on feature/auth/oauth
```

### Table 格式
```
┌─────────────────────┬──────────────────────────┬─────────┬──────────┐
│ Directory           │ Branch                    │ Status  │ Head     │
├─────────────────────┼──────────────────────────┼─────────┼──────────┤
│ ⌂ worktree          │ 003-branch-slash-conversion│ modified│ abc1234  │
│ feat-feature-001    │ feat/feature-001          │ clean   │ def5678  │
│ feat-auth-oauth     │ feature/auth/oauth        │ clean   │ ghi9012  │
└─────────────────────┴──────────────────────────┴─────────┴──────────┘
```

### JSON 格式
```json
[
  {
    "directory": "feat-feature-001",
    "branch": "feat/feature-001",
    "status": "clean",
    "commit": "def5678"
  }
]
```

**数据模型扩展**:
```rust
struct Worktree {
    dirname: String,        // 新增：目录名
    branch_name: String,    // 重命名：Git 分支名
    path: PathBuf,
    status: WorktreeStatus,
    commit: String,
}
```

**考虑的替代方案**:
1. **仅显示目录名**: 用户无法知道实际分支名，失去信息
2. **仅显示分支名**: 用户无法知道实际目录名，无法执行命令
3. **使用分隔符**: 如 `feat-feature-001 | feat/feature-001`，不如 "on" 语义清晰

---

## Decision 4: 文件系统非法字符处理

**决策**: 仅处理斜杠（`/`）字符，其他文件系统非法字符由 Git 和操作系统处理。

**理由**:
- **职责分离**: Git 已经阻止包含非法字符的分支名
- **平台差异**: 不同操作系统的非法字符集不同，统一处理复杂
- **实际需求**: 斜杠是唯一常见的、Git 允许但文件系统不允许的字符

**Git 的分支名规则**:
- Git 允许: 字母、数字、`.`、`-`、`_`、`/`
- Git 拒绝: `:`、`?`、`[`、`]`、`@`、`{`、`}`、空格、控制字符
- Git 限制: 不能以 `.` 开头，不能以 `.lock` 结尾

**文件系统的额外限制**:
- Windows: 拒绝 `<`、`>`、`:`、`"`、`/`、`\`、`|`、`?`、`*`
- macOS/Linux: 仅拒绝 `/` 和 `\0`（空字符）

**实现方案**:
1. 依赖 Git 拒绝非法字符（无需额外验证）
2. 仅将 `/` 转换为 `-`（解决最常见问题）
3. 如果其他非法字符 somehow 存在，让操作系统返回错误（极罕见）

**错误处理**:
```rust
fn validate_dirname(dirname: &str) -> Result<(), WorktreeError> {
    // 仅检查转换后的目录名是否为空
    if dirname.is_empty() {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot be empty or only slashes".to_string(),
        ));
    }
    Ok(())
}
```

---

## Decision 5: 目录名长度限制处理

**决策**: 不强制限制目录名长度，依赖文件系统和 Git 的限制。

**理由**:
- **平台差异**: 不同文件系统的最大路径长度不同（Windows 260 字符，Linux 4096 字符）
- **实际需求**: 分支名通常不会超过合理长度（< 100 字符）
- **简洁性**: 避免引入复杂的长度验证和截断逻辑

**文件系统限制**:
- **Windows**: MAX_PATH = 260 字符（包括完整路径）
- **macOS**: 无硬限制（通常受限于 HFS+/APFS 的实现）
- **Linux**: PATH_MAX = 4096 字符

**Git 的限制**:
- Git 本身不限制分支名长度
- 但文件系统会限制 worktree 路径长度

**实现方案**:
```rust
fn create_worktree(branch: &str, dirname: &str, path: &Path) -> Result<(), WorktreeError> {
    // 尝试创建，让操作系统返回错误（如果路径过长）
    Command::new("git")
        .args(["worktree", "add", path.to_str().unwrap(), branch])
        .output()
        .map_err(|e| {
            if e.kind() io::ErrorKind::Filesystem || e.kind() == io::ErrorKind::FileNameTooLong {
                WorktreeError::PathTooLong(path.to_path_buf())
            } else {
                WorktreeError::GitError(e.to_string())
            }
        })?;
    Ok(())
}
```

**错误信息**:
```
Error: Cannot create worktree - path too long

The worktree path '/path/to/repo.worktrees/very-long-branch-name...'
exceeds the maximum path length allowed by the filesystem.

Suggested solutions:
  1. Use a shorter branch name
  2. Create the worktree in a location with a shorter base path
```

---

## Decision 6: 跨平台路径处理

**决策**: 使用 Rust 的 `std::path::Path` 和 `std::path::PathBuf` 处理所有路径操作。

**理由**:
- **跨平台**: `Path` 自动处理不同操作系统的路径分隔符（`/` vs `\`）
- **安全性**: 防止路径遍历攻击和非法路径
- **标准库**: 无需引入额外依赖

**实现方案**:
```rust
use std::path::{Path, PathBuf};

fn get_worktree_path(base: &Path, dirname: &str) -> PathBuf {
    // 自动使用正确的路径分隔符
    base.join(dirname)
}
```

**目录名验证**:
```rust
fn validate_dirname(dirname: &str) -> Result<(), WorktreeError> {
    // 检查是否包含路径分隔符（防止创建子目录）
    if dirname.contains('/') || dirname.contains('\\') {
        return Err(WorktreeError::InvalidBranchName(
            "Directory name cannot contain path separators".to_string(),
        ));
    }

    // 检查是否为相对路径（防止路径遍历）
    if dirname.starts_with('.') {
        return Err(WorktreeError::InvalidBranchName(
            "Directory name cannot start with a dot".to_string(),
        ));
    }

    Ok(())
}
```

**考虑的替代方案**:
1. **手动拼接字符串**: 容易出错，不跨平台
2. **使用路径库**: 如 `path-clean`，但标准库已足够

---

## 技术风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|----------|
| 转换后的目录名冲突 | 中 | 中 | 创建前检测，提供清晰的错误信息 |
| 路径过长导致创建失败 | 低 | 低 | 捕获操作系统错误，提供友好的错误信息 |
| 不同操作系统的路径差异 | 中 | 中 | 使用 `std::path::Path` 确保跨平台兼容性 |
| 用户对转换规则不理解 | 低 | 中 | 在错误信息和文档中清晰说明转换规则 |

---

## 依赖关系

**无新增依赖** - 所有功能使用现有 Rust 标准库和现有依赖（clap, anyhow, thiserror）。

---

## 性能考虑

- **分支名转换**: O(n) 时间复杂度，n < 100 典型，可忽略
- **冲突检测**: O(m) 时间复杂度，m = worktree 数量（通常 < 100），可忽略
- **输出格式化**: 现有性能不受影响

---

## 测试策略

### 单元测试
- `branch_to_dirname()`: 测试各种分支名格式（简单、多级、边界情况）
- `check_dirname_conflict()`: 测试冲突检测逻辑
- `validate_dirname()`: 测试目录名验证

### 集成测试
- 创建包含斜杠的 worktree，验证目录名和分支名
- 尝试创建冲突的 worktree，验证错误信息
- 列出 worktree，验证输出格式
- 删除 worktree，验证目录名和分支名的正确性

### 边界测试
- 空分支名、仅斜杠的分支名
- 超长分支名（1000+ 字符）
- 特殊字符（`///`、`/feat`、`feat/`）
- 多级分支（`a/b/c/d/e`）

---

## 后续优化

1. **可配置的转换规则**: 允许用户自定义转换规则（如移除前导/尾随连字符）
2. **冲突解决策略**: 提供自动重命名选项（如 `feat-feature-001-1`）
3. **转换预览**: 在创建前显示转换后的目录名，确认后再创建

---

---

## Decision 7: 主目录路径显示增强 (2026-01-16 补充)

**用户需求**: "work list 需要输出主目录的完整路径"

**决策**: 扩展现有的 `format_worktree_compact()` 函数，为主目录（非 .worktrees 目录）在行尾添加完整路径显示。

**理由**:
1. **用户价值**: 在复杂的多 worktree 环境中，开发者需要快速定位主仓库位置
2. **最小改动**: 现有代码已支持识别主目录（通过 `!wt.path.contains(".worktrees")` 判断）
3. **一致性**: 与现有设计语言保持一致（使用 `.dimmed()` 灰色显示辅助信息）

**实现方案**:

### 修改 Compact 格式
```rust
// 现有输出：
// *⌂  worktree on 003-branch-slash-conversion (modified)

// 修改后输出：
// *⌂  worktree on 003-branch-slash-conversion (modified) at /Volumes/code/demos/worktree
```

**代码实现** (src/cli/output.rs):
```rust
pub fn format_worktree_compact(worktrees: Vec<Worktree>) -> String {
    let mut output = String::new();

    for wt in worktrees {
        let is_main = !wt.path.contains(".worktrees");

        // ... 现有的标记和颜色逻辑 ...

        // 新增：主目录路径显示
        let path_info = if is_main {
            format!(" at {}", wt.path.dimmed())
        } else {
            String::new()  // 非 worktree 目录不显示路径（避免信息过载）
        };

        output.push_str(&format!(
            "{}{} {}{}{}\n",
            current_marker, main_marker, name, branch_info, status_marker, path_info
        ));
    }

    output.trim_end().to_string()
}
```

### Table 格式（无需修改）
Table 格式已包含 PATH 列，显示完整路径：
```text
┌─────────────────────┬──────────────────────────┬─────────────────────────┬───┬────────┐
│ NAME                │ BRANCH                    │ PATH                    │ … │ STATUS │
├─────────────────────┼──────────────────────────┼─────────────────────────┼───┼────────┤
│ ⌂ worktree          │ 003-branch-slash-conversion│ /Volumes/code/demos/…  │ * │ Modified│
│ feat-feature-001    │ feat/feature-001          │ …/worktrees/feat-…     │   │ Healthy │
└─────────────────────┴──────────────────────────┴─────────────────────────┴───┴────────┘
```

### JSON 格式（无需修改）
JSON 已包含 `path` 字段：
```json
{
  "dirname": "worktree",
  "branch_name": "003-branch-slash-conversion",
  "path": "/Volumes/code/demos/worktree",
  ...
}
```

**设计决策**:

| 决策点 | 选择 | 理由 |
|--------|------|------|
| 显示位置 | Compact 格式行尾 | 不干扰主要信息（目录名、分支名、状态） |
| 路径颜色 | 灰色 (dimmed) | 与现有辅助信息（状态标记）保持一致 |
| 显示范围 | 仅主目录 | 避免信息过载，worktree 路径通常不重要 |
| 路径格式 | 完整绝对路径 | 明确无歧义，用户可自行缩写（如 shell 别名） |
| 前缀文本 | " at " | 与 " on "（分支）语义对称，清晰自然 |

**考虑的替代方案**:

1. **为所有 worktree 显示路径**: 被拒绝，会导致信息过载和行过长
2. **使用 `--verbose` 标志控制**: 被拒绝，增加命令复杂度，与"简化管理"理念冲突
3. **新增 `work path` 子命令**: 被拒绝，过度设计，用户希望在 list 中直接看到
4. **缩写路径为 `~` 或 `$HOME`**: 被拒绝，增加实现复杂度且不跨平台（Windows 无 `~`）

**边界情况处理**:
- **超长路径**: 依赖终端自动换行，不手动截断
- **特殊字符路径**: Rust `std::path` 自动处理，无需额外逻辑
- **Windows 路径**: 显示完整路径（如 `C:\Users\...\project`）

**错误处理**: 无需新增错误处理，路径信息直接来自 `Worktree.path` 字段（已验证有效）

**性能影响**:
- **字符串拼接**: 每个主目录 worktree 增加 1 次 `format!()` 调用，开销可忽略
- **路径长度**: 典型路径 < 100 字符，对输出性能无影响

**测试策略**:
- **单元测试**: 验证格式化输出包含 " at /path" 后缀（主目录）
- **集成测试**: 验证 `work list` 输出正确显示主目录路径
- **手动测试**: 验证颜色和布局视觉效果

**向后兼容性**:
- ✅ JSON 格式不变（已包含 path 字段）
- ✅ Table 格式不变（PATH 列已存在）
- ⚠️  Compact 格式变更（新增信息，不破坏现有脚本）

**用户文档更新**:
```markdown
## work list - 列出所有 worktree

**输出格式**:
- Compact（默认）: 简洁格式，主目录显示完整路径
- Table: 表格格式，所有 worktree 显示完整路径
- JSON: 机器可读格式

**示例**:
```bash
$ work list
*⌂  worktree on 003-branch-slash-conversion (modified) at /Volumes/code/demos/worktree
  feat-feature-001 on feat/feature-001
```
```

**实施优先级**: P2（User Story 2 的增强，不阻塞 P1 功能）

---

## 结论

所有技术决策已明确，包括新增的主目录路径显示功能。无遗留问题，可以进入 Phase 1（设计与契约）阶段。
