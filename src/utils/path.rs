use crate::utils::errors::{Result, WorktreeError};
use std::path::{Path, PathBuf};

use std::path::Component;

/// 规范化路径以用于 Git 命令
///
/// Windows 可能返回扩展路径语法 (//?/C:/...),Git 可能无法正确处理。
/// 此函数将路径转换为标准格式。
///
/// # Arguments
/// * `path` - 要规范化的路径
///
/// # Returns
/// 规范化后的 PathBuf
pub fn normalize_path_for_git(path: &Path) -> PathBuf {

    // 转换为 PathBuf 以便操作
    let path_buf = path.to_path_buf();

    // 在 Windows 上,检查是否为扩展路径语法 (//?/C:/...)
    if cfg!(windows) {
        let path_str = path_buf.to_string_lossy();

        // 检查是否以 //?/ 开头 (Windows 扩展长度路径前缀)
        if path_str.starts_with("//?/") {
            // 移除 //?/ 前缀，返回标准路径
            if let Some(stripped) = path_str.strip_prefix("//?/") {
                return PathBuf::from(stripped);
            }
        }

        // 处理 //?/UNC/server/share 形式
        if path_str.starts_with("//?/UNC/") {
            if let Some(stripped) = path_str.strip_prefix("//?/UNC/") {
                return PathBuf::from(format!("//{}", stripped));
            }
        }
    }

    // 非 Windows 或已处理的路径，尝试规范化
    // 注意：canonicalize() 要求路径存在，所以我们只在路径存在时调用
    if path.exists() {
        path_buf.canonicalize().unwrap_or(path_buf)
    } else {
        // 路径不存在，只做基本清理（移除 . 和 ..）
        // 这不会处理符号链接，但足以满足我们的需求
        clean_path(&path_buf)
    }
}

/// 清理路径中的 . 和 .. 组件（不要求路径存在）
fn clean_path(path: &Path) -> PathBuf {
    let mut result = Vec::new();

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => {
                // 保留前缀 (如 C: 或 \\server\share)
                result.push(component);
            }
            Component::RootDir => {
                result.push(component);
            }
            Component::CurDir => {
                // 跳过当前目录 (.)
            }
            Component::ParentDir => {
                // 回退到上一级 (..)
                if result.len() > 1 {
                    // 不移除前缀和根目录
                    result.pop();
                }
            }
            Component::Normal(normal) => {
                result.push(Component::Normal(normal));
            }
        }
    }

    // 重新组合路径
    let mut path_buf = PathBuf::new();
    for component in result {
        path_buf.push(component);
    }

    path_buf
}

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

    #[test]
    fn test_normalize_path_for_git_basic() {
        // 基本路径应该保持不变
        let path = Path::new("/usr/local/bin");
        let normalized = normalize_path_for_git(path);
        assert_eq!(normalized, PathBuf::from("/usr/local/bin"));
    }

    #[test]
    fn test_normalize_path_for_git_with_dots() {
        // 包含 . 和 .. 的路径应该被清理
        let path = Path::new("/usr/local/../bin/./test");
        let normalized = normalize_path_for_git(path);
        assert_eq!(normalized, PathBuf::from("/usr/bin/test"));
    }

    #[test]
    fn test_normalize_path_for_git_windows_extended_path() {
        // Windows 扩展路径语法
        // 这个测试只在 Windows 上有效，因为非 Windows 系统的 Path 实现不同
        let path = Path::new("//?/C:/Users/test");
        let normalized = normalize_path_for_git(path);

        // 在非 Windows 系统上，路径保持不变
        // 在 Windows 上，应该被规范化为 C:/Users/test
        if cfg!(windows) {
            assert_eq!(normalized, PathBuf::from("C:/Users/test"));
        } else {
            // 在 macOS/Linux 上，//?/ 被视为普通路径组件
            assert_eq!(normalized, PathBuf::from("/?/C:/Users/test"));
        }
    }

    #[test]
    fn test_normalize_path_for_git_windows_unc_path() {
        // Windows UNC 路径
        // 这个测试只在 Windows 上有效
        let path = Path::new("//?/UNC/server/share");
        let normalized = normalize_path_for_git(path);

        if cfg!(windows) {
            assert_eq!(normalized, PathBuf::from("//server/share"));
        } else {
            // 在 macOS/Linux 上，路径保持不变
            assert_eq!(normalized, PathBuf::from("/?/UNC/server/share"));
        }
    }

    #[test]
    fn test_clean_path_simple() {
        let path = Path::new("/a/b/c");
        let cleaned = clean_path(path);
        assert_eq!(cleaned, PathBuf::from("/a/b/c"));
    }

    #[test]
    fn test_clean_path_with_double_dot() {
        let path = Path::new("/a/b/../c");
        let cleaned = clean_path(path);
        assert_eq!(cleaned, PathBuf::from("/a/c"));
    }

    #[test]
    fn test_clean_path_with_single_dot() {
        let path = Path::new("/a/./b/./c");
        let cleaned = clean_path(path);
        assert_eq!(cleaned, PathBuf::from("/a/b/c"));
    }

    #[test]
    fn test_clean_path_cannot_escape_root() {
        // .. 不能超过根目录
        let path = Path::new("/../a");
        let cleaned = clean_path(path);
        assert_eq!(cleaned, PathBuf::from("/a"));
    }
}
