use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::core::git_ops;

/// Git worktree 的概念表示
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    /// Worktree 名称（通常基于分支名）
    pub name: String,
    /// 分支名
    pub branch: String,
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
        name: String,
        branch: String,
        path: String,
        is_current: bool,
        is_bare: bool,
        is_detached: bool,
        head_commit: Option<String>,
        upstream_branch: Option<String>,
    ) -> Self {
        Worktree {
            name,
            branch,
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

        assert_eq!(worktree.name, "feature-auth");
        assert_eq!(worktree.branch, "feature-auth");
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
        assert_eq!(current.unwrap().name, "main");
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
}
