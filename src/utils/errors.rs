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

    #[error(
        "Main repository is in detached HEAD state at {main_repo_path} (commit: {commit_sha})"
    )]
    MainRepoDetachedHead {
        main_repo_path: String,
        commit_sha: String,
    },

    #[error(
        "Current directory is in detached HEAD state at {current_path} (commit: {commit_sha})"
    )]
    CurrentDirDetachedHead {
        current_path: String,
        commit_sha: String,
    },

    #[error("Branch '{branch_name}' not found. Available locals: {available_locals:?}, remotes: {available_remotes:?}")]
    BranchNotFound {
        branch_name: String,
        available_locals: Vec<String>,
        available_remotes: Vec<String>,
    },

    #[error("Invalid branch source option: {input}")]
    InvalidBranchSource { input: String },
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, WorktreeError>;
