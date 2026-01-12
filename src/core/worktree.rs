use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::core::git_ops;

/// Git worktree 的概念表示
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    /// Worktree 目录名（分支名中的 / 替换为 -）
    pub dirname: String,
    /// Git 分支名（原始名称，可能包含 /）
    pub branch_name: String,
    /// Worktree 名称（用于向后兼容，等同于 dirname）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 分支名（用于向后兼容，等同于 branch_name）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Worktree 目录的绝对路径
    pub path: String,
    /// 是否为当前 worktree（shell 所在目录）
    pub is_current: bool,
    /// 是否为裸仓库
    pub is_bare: bool,
    /// 是否处于分离 HEAD 状态
    pub is_detached: bool,
    /// HEAD 提交的 SHA
    pub head_commit: Option<String>,
    /// 上游跟踪分支（如 `origin/main`）
    pub upstream_branch: Option<String>,
    /// 最后修改时间
    pub last_modified: DateTime<Utc>,
}

impl Worktree {
    /// 创建一个新的 Worktree 实例
    pub fn new(
        dirname: String,
        branch_name: String,
        path: String,
        is_current: bool,
        is_bare: bool,
        is_detached: bool,
        head_commit: Option<String>,
        upstream_branch: Option<String>,
    ) -> Self {
        Worktree {
            dirname: dirname.clone(),
            branch_name: branch_name.clone(),
            name: Some(dirname.clone()),
            branch: Some(branch_name.clone()),
            path,
            is_current,
            is_bare,
            is_detached,
            head_commit,
            upstream_branch,
            last_modified: Utc::now(),
        }
    }

    /// 基于当前工作目录查找当前的 worktree
    pub fn find_current_worktree(worktrees: &[Worktree]) -> Option<&Worktree> {
        worktrees.iter().find(|wt| wt.is_current)
    }

    /// 获取显示名称（用于输出）
    /// 如果目录名和分支名不同，返回 "dirname on branch" 格式
    /// 如果相同，只返回目录名
    pub fn display_name(&self) -> String {
        if self.dirname == self.branch_name {
            self.dirname.clone()
        } else {
            format!("{} on {}", self.dirname, self.branch_name)
        }
    }

    /// 检查 worktree 是否有未提交的更改
    pub fn has_uncommitted_changes(&self) -> bool {
        let path = Path::new(&self.path);
        git_ops::has_uncommitted_changes(path).unwrap_or(false)
    }

    /// 获取 worktree 的 Git 状态
    pub fn get_status(&self) -> WorktreeStatus {
        // TODO: 实现状态检测
        // 返回 Healthy/Modified/Conflict 等状态
        if self.is_detached {
            WorktreeStatus::Detached
        } else if self.has_uncommitted_changes() {
            WorktreeStatus::Modified
        } else {
            WorktreeStatus::Healthy
        }
    }
}

/// Worktree 状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorktreeStatus {
    /// 健康状态（无未提交更改）
    Healthy,
    /// 有未提交的更改
    Modified,
    /// 处于分离 HEAD 状态
    Detached,
    /// 有冲突
    Conflict,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worktree_creation() {
        let worktree = Worktree::new(
            "feature-auth".to_string(),
            "feature-auth".to_string(),
            "/home/user/project/worktrees/feature-auth".to_string(),
            false,
            false,
            false,
            Some("abc123".to_string()),
            Some("origin/feature-auth".to_string()),
        );

        assert_eq!(worktree.dirname, "feature-auth");
        assert_eq!(worktree.branch_name, "feature-auth");
        assert_eq!(worktree.is_current, false);
        assert_eq!(worktree.head_commit, Some("abc123".to_string()));
    }

    #[test]
    fn test_find_current_worktree() {
        let worktrees = vec![
            Worktree::new(
                "main".to_string(),
                "main".to_string(),
                "/home/user/project".to_string(),
                true,
                false,
                false,
                Some("abc123".to_string()),
                None,
            ),
            Worktree::new(
                "feature-auth".to_string(),
                "feature-auth".to_string(),
                "/home/user/project/worktrees/feature-auth".to_string(),
                false,
                false,
                false,
                Some("def456".to_string()),
                None,
            ),
        ];

        let current = Worktree::find_current_worktree(&worktrees);
        assert!(current.is_some());
        assert_eq!(current.unwrap().dirname, "main");
    }

    #[test]
    fn test_worktree_status() {
        let worktree = Worktree::new(
            "feature-auth".to_string(),
            "feature-auth".to_string(),
            "/home/user/project/worktrees/feature-auth".to_string(),
            false,
            false,
            false,
            Some("abc123".to_string()),
            None,
        );

        let status = worktree.get_status();
        assert_eq!(status, WorktreeStatus::Healthy);
    }

    #[test]
    fn test_detached_worktree_status() {
        let worktree = Worktree::new(
            "detached-head".to_string(),
            "HEAD".to_string(),
            "/home/user/project/worktrees/detached".to_string(),
            false,
            false,
            true,
            Some("abc123".to_string()),
            None,
        );

        let status = worktree.get_status();
        assert_eq!(status, WorktreeStatus::Detached);
    }

    #[test]
    fn test_display_name_no_slash() {
        let worktree = Worktree::new(
            "main".to_string(),
            "main".to_string(),
            "/home/user/project".to_string(),
            false,
            false,
            false,
            Some("abc123".to_string()),
            None,
        );

        assert_eq!(worktree.display_name(), "main");
    }

    #[test]
    fn test_display_name_with_slash() {
        let worktree = Worktree::new(
            "feat-feature-001".to_string(),
            "feat/feature-001".to_string(),
            "/home/user/project.worktrees/feat-feature-001".to_string(),
            false,
            false,
            false,
            Some("abc123".to_string()),
            None,
        );

        assert_eq!(worktree.display_name(), "feat-feature-001 on feat/feature-001");
    }

    #[test]
    fn test_display_name_multiple_slashes() {
        let worktree = Worktree::new(
            "feature-auth-oauth".to_string(),
            "feature/auth/oauth".to_string(),
            "/home/user/project.worktrees/feature-auth-oauth".to_string(),
            false,
            false,
            false,
            Some("abc123".to_string()),
            None,
        );

        assert_eq!(worktree.display_name(), "feature-auth-oauth on feature/auth/oauth");
    }
}
