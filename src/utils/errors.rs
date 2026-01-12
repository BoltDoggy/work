use std::path::PathBuf;
use thiserror::Error;

/// Worktree 管理工具的错误类型
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
    GitError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Worktree has uncommitted changes")]
    UncommittedChanges,

    #[error("Cannot delete current worktree")]
    CannotDeleteCurrent,

    #[error("Not a git repository: {0}")]
    NotGitRepository(PathBuf),

    #[error("Invalid branch name: {0}")]
    InvalidBranchName(String),

    #[error("Directory name conflict: '{dirname}' already exists for branch '{existing_branch}'")]
    DirNameConflict {
        dirname: String,
        existing_branch: String,
    },
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, WorktreeError>;
