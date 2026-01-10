use crate::utils::errors::{Result, WorktreeError};
use crate::core::git_ops;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git 仓库的抽象表示
#[derive(Debug, Clone)]
pub struct Repository {
    /// 仓库根目录的绝对路径（`.git` 目录所在位置）
    pub root_path: PathBuf,
    /// 是否为裸仓库
    pub is_bare: bool,
    /// 关联的 worktree 数量
    pub worktree_count: usize,
    /// 默认分支名
    pub default_branch: String,
    /// 当前 worktree 名称（基于 shell cwd）
    pub current_worktree: Option<String>,
}

impl Repository {
    /// 从路径打开 Git 仓库并返回 Repository 信息
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // 尝试获取仓库根目录
        let output = Command::new("git")
            .args(["-C", path.to_str().ok_or_else(|| WorktreeError::InvalidPath(path.to_string_lossy().to_string()))?,
                   "rev-parse", "--show-toplevel"])
            .output()
            .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

        if !output.status.success() {
            return Err(WorktreeError::NotGitRepository(path.to_path_buf()));
        }

        let root_path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());

        // 检查是否为裸仓库
        let is_bare = git_ops::is_bare_repository().unwrap_or(false);

        // 获取 worktree 数量
        let worktree_count = Self::count_worktrees(&root_path)?;

        // 确定默认分支名
        let default_branch = Self::detect_default_branch(&root_path)?;

        // 确定当前 worktree
        let current_worktree = Self::detect_current_worktree(&root_path, path)?;

        Ok(Repository {
            root_path,
            is_bare,
            worktree_count,
            default_branch,
            current_worktree,
        })
    }

    /// 计算关联的 worktree 数量
    fn count_worktrees(root_path: &Path) -> Result<usize> {
        let output = Command::new("git")
            .args(["-C", root_path.to_str().ok_or_else(|| WorktreeError::InvalidPath(root_path.to_string_lossy().to_string()))?,
                   "worktree", "list"])
            .output()
            .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

        if !output.status.success() {
            return Ok(0);
        }

        // 计算行数，每行代表一个 worktree
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().count())
    }

    /// 检测仓库的默认分支名
    fn detect_default_branch(root_path: &Path) -> Result<String> {
        let root_path_str = root_path.to_str().ok_or_else(|| WorktreeError::InvalidPath(root_path.to_string_lossy().to_string()))?;

        // 尝试获取 HEAD 引用
        let output = Command::new("git")
            .args(["-C", root_path_str, "symbolic-ref", "--short", "HEAD"])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !branch.is_empty() {
                    return Ok(branch);
                }
            }
            _ => {
                // HEAD 可能是分离的或仓库为空
            }
        }

        // 如果 HEAD 是分离的，尝试查找常见的默认分支名
        let common_defaults = ["main", "master", "develop"];
        for branch_name in common_defaults {
            // 检查分支是否存在
            let output = Command::new("git")
                .args(["-C", root_path_str, "show-ref", "--verify", "--quiet", &format!("refs/heads/{}", branch_name)])
                .output();

            if output.map(|o| o.status.success()).unwrap_or(false) {
                return Ok(branch_name.to_string());
            }
        }

        Ok("main".to_string())
    }

    /// 检测当前 worktree（基于当前工作目录）
    fn detect_current_worktree(root_path: &Path, current_path: &Path) -> Result<Option<String>> {
        // 如果当前路径就是仓库根目录，则为主 worktree
        if current_path == root_path {
            return Ok(Some("main".to_string()));
        }

        // 获取当前 worktree 路径
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

        if output.status.success() {
            let worktree_path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());

            // 如果当前路径不是主仓库，尝试从路径中提取 worktree 名称
            if worktree_path != root_path {
                if let Some(name) = worktree_path
                    .file_name()
                    .and_then(|n| n.to_str())
                {
                    return Ok(Some(name.to_string()));
                }
            }
        }

        // 无法确定当前 worktree，返回 main
        Ok(Some("main".to_string()))
    }

    /// 获取仓库信息（作为辅助函数，主要用于 CLI 显示）
    pub fn get_repository_info(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_repository_from_path() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // 初始化 Git 仓库
        Command::new("git")
            .args(["init"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // 创建 Repository
        let repo = Repository::from_path(repo_path).unwrap();

        // 验证基本信息
        assert_eq!(repo.is_bare, false);
        assert_eq!(repo.worktree_count, 1); // main worktree
    }

    #[test]
    fn test_repository_from_invalid_path() {
        // 创建临时目录（不是 Git 仓库）
        let temp_dir = TempDir::new().unwrap();
        let non_repo_path = temp_dir.path();

        // 尝试创建 Repository 应该失败
        let result = Repository::from_path(non_repo_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_repository_with_initial_commit() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // 初始化 Git 仓库
        Command::new("git")
            .args(["init"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        std::env::set_current_dir(repo_path).unwrap();

        // 创建测试文件
        fs::write(repo_path.join("test.txt"), "test content").unwrap();

        // 添加文件到暂存区
        Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // 创建初始提交
        Command::new("git")
            .args(["-c", "user.name=Test", "-c", "user.email=test@example.com",
                   "commit", "-m", "Initial commit"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // 创建 Repository
        let repository = Repository::from_path(repo_path).unwrap();
        // 验证可以成功创建
        assert_eq!(repository.is_bare, false);
    }
}
