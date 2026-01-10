use crate::utils::errors::{Result, WorktreeError};
use std::path::Path;

/// 验证 worktree 名称
pub fn validate_worktree_name(name: &str) -> Result<()> {
    // 不能为空
    if name.is_empty() {
        return Err(WorktreeError::InvalidName(
            "Worktree name cannot be empty".to_string(),
        ));
    }

    // 不能包含特殊字符（除了 - 和 _）
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(WorktreeError::InvalidName(
            "Worktree name contains invalid characters".to_string(),
        ));
    }

    // 不能以 . 开头
    if name.starts_with('.') {
        return Err(WorktreeError::InvalidName(
            "Worktree name cannot start with '.'".to_string(),
        ));
    }

    // 不能是 Git 保留名称
    const RESERVED_NAMES: &[&str] = &["HEAD", "FETCH_HEAD", "ORIG_HEAD", "MERGE_HEAD"];
    if RESERVED_NAMES.contains(&name) {
        return Err(WorktreeError::InvalidName(format!(
            "Worktree name '{}' is reserved by Git",
            name
        )));
    }

    Ok(())
}

/// 验证 worktree 路径
pub fn validate_worktree_path(path: &Path, repo_root: &Path) -> Result<()> {
    // 必须是绝对路径
    if !path.is_absolute() {
        return Err(WorktreeError::InvalidPath(
            "Worktree path must be absolute".to_string(),
        ));
    }

    // 不能在主仓库目录内
    if path.starts_with(repo_root) {
        return Err(WorktreeError::InvalidPath(
            "Worktree cannot be inside main repository".to_string(),
        ));
    }

    // 如果路径已存在，验证是否是有效的 worktree
    if path.exists() {
        // 必须是目录
        if !path.is_dir() {
            return Err(WorktreeError::InvalidPath(
                "Worktree path must be a directory".to_string(),
            ));
        }

        // 必须包含 .git 文件（worktree 元数据）
        let git_file = path.join(".git");
        if !git_file.exists() {
            return Err(WorktreeError::InvalidPath(
                "Path is not a valid worktree".to_string(),
            ));
        }
    }

    Ok(())
}

/// 验证分支名
pub fn validate_branch_name(name: &str) -> Result<()> {
    // 不能为空
    if name.is_empty() {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot be empty".to_string(),
        ));
    }

    // 不能以 - 开头或结尾
    if name.starts_with('-') || name.ends_with('-') {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot start or end with '-'".to_string(),
        ));
    }

    // 不能包含连续的 ..
    if name.contains("..") {
        return Err(WorktreeError::InvalidBranchName(
            "Branch name cannot contain '..'".to_string(),
        ));
    }

    // 不能包含特殊字符（除了 /, -, _, .）
    for c in name.chars() {
        if !c.is_alphanumeric() && !matches!(c, '/' | '-' | '_' | '.') {
            return Err(WorktreeError::InvalidBranchName(format!(
                "Branch name contains invalid character: '{}'",
                c
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_worktree_name_valid() {
        assert!(validate_worktree_name("feature-auth").is_ok());
        assert!(validate_worktree_name("bug_fix_123").is_ok());
        assert!(validate_worktree_name("main").is_ok());
    }

    #[test]
    fn test_validate_worktree_name_invalid() {
        assert!(validate_worktree_name("").is_err());
        assert!(validate_worktree_name(".hidden").is_err());
        assert!(validate_worktree_name("feature auth").is_err());
        assert!(validate_worktree_name("HEAD").is_err());
    }

    #[test]
    fn test_validate_branch_name_valid() {
        assert!(validate_branch_name("main").is_ok());
        assert!(validate_branch_name("feature/auth").is_ok());
        assert!(validate_branch_name("bug-fix-123").is_ok());
    }

    #[test]
    fn test_validate_branch_name_invalid() {
        assert!(validate_branch_name("").is_err());
        assert!(validate_branch_name("-main").is_err());
        assert!(validate_branch_name("main-").is_err());
        assert!(validate_branch_name("feature..auth").is_err());
    }
}
