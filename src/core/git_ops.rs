use crate::utils::errors::{Result, WorktreeError};
use crate::core::worktree::Worktree;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 运行 git 命令并返回输出
fn run_git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(WorktreeError::GitError(stderr.to_string()));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| WorktreeError::GitError(format!("Invalid UTF-8 output: {}", e)))
}

/// 获取仓库根目录
pub fn get_repository_root() -> Result<PathBuf> {
    let output = run_git(&["rev-parse", "--show-toplevel"])?;
    let path = output.trim();
    Ok(PathBuf::from(path))
}

/// 列出所有 worktree
pub fn list_worktrees() -> Result<Vec<Worktree>> {
    let output = run_git(&["worktree", "list", "--porcelain"])?;

    let mut result = Vec::new();
    let mut current_worktree: Option<WorktreeData> = None;

    // 解析 git worktree list --porcelain 输出
    // 格式示例:
    // worktree /path/to/worktree
    // HEAD abc123def456
    // branch refs/heads/main
    // detached
    for line in output.lines() {
        if line.is_empty() {
            // 空行表示新 worktree 开始
            if let Some(wt_data) = current_worktree.take() {
                if let Ok(worktree) = wt_data.to_worktree() {
                    result.push(worktree);
                }
            }
        } else if let Some((key, value)) = line.split_once(' ') {
            match key {
                "worktree" => {
                    current_worktree = Some(WorktreeData::new(value.to_string()));
                }
                "HEAD" => {
                    if let Some(ref mut wt) = current_worktree {
                        wt.head_commit = Some(value.to_string());
                    }
                }
                "branch" => {
                    if let Some(ref mut wt) = current_worktree {
                        // 分支格式: refs/heads/main or refs/remotes/origin/main
                        wt.branch = value
                            .strip_prefix("refs/heads/")
                            .or_else(|| value.strip_prefix("refs/remotes/"))
                            .unwrap_or(value)
                            .to_string();
                        wt.is_detached = false;
                    }
                }
                "detached" => {
                    if let Some(ref mut wt) = current_worktree {
                        wt.is_detached = true;
                    }
                }
                _ => {}
            }
        }
    }

    // 添加最后一个 worktree
    if let Some(wt_data) = current_worktree {
        if let Ok(worktree) = wt_data.to_worktree() {
            result.push(worktree);
        }
    }

    Ok(result)
}

/// 检查路径是否在 Git 仓库中
pub fn is_inside_repository<P: AsRef<Path>>(path: P) -> bool {
    Command::new("git")
        .args(["-C", path.as_ref().to_str().unwrap_or("."), "rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 检查是否为裸仓库
pub fn is_bare_repository() -> Result<bool> {
    let output = run_git(&["rev-parse", "--is-bare-repository"])?;
    Ok(output.trim() == "true")
}

/// 获取当前分支名
pub fn get_current_branch() -> Result<String> {
    let output = run_git(&["rev-parse", "--abbrev-ref", "HEAD"])?;
    let branch = output.trim();
    if branch == "HEAD" {
        Ok("HEAD".to_string()) // detached HEAD
    } else {
        Ok(branch.to_string())
    }
}

/// 获取 upstream 分支
pub fn get_upstream_branch() -> Result<Option<String>> {
    let output = run_git(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"]);
    if output.is_err() {
        return Ok(None);
    }
    let upstream = output?.trim().to_string();
    if upstream.is_empty() || upstream.contains("@{u}") {
        Ok(None)
    } else {
        Ok(Some(upstream))
    }
}

/// 创建新的 worktree（基于现有分支）
pub fn create_worktree(branch_name: &str, path: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["worktree", "add", path, branch_name])
        .output()
        .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(WorktreeError::GitError(stderr.to_string()));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| WorktreeError::GitError(format!("Invalid UTF-8 output: {}", e)))
}

/// 创建新分支并同时创建 worktree
pub fn create_worktree_with_new_branch(branch_name: &str, path: &str, upstream: Option<&str>) -> Result<String> {
    let mut args = vec!["worktree", "add", "-b", branch_name, path];

    if let Some(upstream_branch) = upstream {
        args.push(upstream_branch);
    } else {
        // 如果没有指定上游，使用当前 HEAD
        args.push("HEAD");
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(WorktreeError::GitError(stderr.to_string()));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| WorktreeError::GitError(format!("Invalid UTF-8 output: {}", e)))
}

/// 删除 worktree
pub fn delete_worktree(path: &str, force: bool) -> Result<String> {
    let mut args = vec!["worktree", "remove"];

    if force {
        args.push("--force");
    }

    args.push(path);

    let output = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(WorktreeError::GitError(stderr.to_string()));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| WorktreeError::GitError(format!("Invalid UTF-8 output: {}", e)))
}

/// 获取所有本地分支列表
pub fn list_local_branches() -> Result<Vec<String>> {
    let output = run_git(&["branch", "--format=%(refname:short)"])?;

    let branches: Vec<String> = output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(branches)
}

/// 检查 worktree 路径是否有未提交的更改
pub fn has_uncommitted_changes(path: &Path) -> Result<bool> {
    let output = Command::new("git")
        .args(["-C", path.to_str().ok_or_else(|| WorktreeError::InvalidPath(path.to_string_lossy().to_string()))?,
               "status", "--porcelain"])
        .output()
        .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

    if !output.status.success() {
        // 如果 git status 失败，可能不是一个有效的 worktree
        return Ok(false);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(!stdout.trim().is_empty())
}

/// 检查分支是否存在
pub fn branch_exists(branch_name: &str) -> bool {
    Command::new("git")
        .args(["show-ref", "--verify", "--quiet", &format!("refs/heads/{}", branch_name)])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 清理无效的 worktree
pub fn prune_worktrees(dry_run: bool) -> Result<Vec<String>> {
    let worktrees = list_worktrees()?;
    let mut pruned = Vec::new();

    for wt in worktrees {
        let path = Path::new(&wt.path);

        // 检查 worktree 目录是否存在
        if !path.exists() {
            if dry_run {
                pruned.push(format!("Would prune: {} (directory not found)", wt.name));
            } else {
                // 使用 git worktree prune 清理无效的 worktree
                let output = Command::new("git")
                    .args(["worktree", "prune"])
                    .output()
                    .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

                if output.status.success() {
                    pruned.push(format!("Pruned: {} (directory not found)", wt.name));
                }
            }
        }
    }

    Ok(pruned)
}

/// 获取 worktree 的详细状态信息
pub fn get_worktree_status(path: &Path) -> Result<WorktreeStatusInfo> {
    // 检查未提交的更改
    let output = Command::new("git")
        .args(["-C", path.to_str().ok_or_else(|| WorktreeError::InvalidPath(path.to_string_lossy().to_string()))?,
               "status", "--porcelain=v1"])
        .output()
        .map_err(|e| WorktreeError::GitError(format!("Failed to execute git: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut modified = Vec::new();
    let mut staged = Vec::new();
    let mut untracked = Vec::new();

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }

        let status = &line[..2];
        let file_path = line[3..].to_string();

        // 第一个字符：暂存区状态
        // 第二个字符：工作区状态
        match status.chars().next().unwrap_or(' ') {
            'M' => staged.push(file_path.clone()),
            'A' => staged.push(file_path.clone()),
            'D' => staged.push(file_path.clone()),
            'R' => staged.push(file_path.clone()),
            _ => {}
        }

        match status.chars().nth(1).unwrap_or(' ') {
            'M' => modified.push(file_path.clone()),
            'D' => modified.push(file_path.clone()),
            _ => {}
        }

        if status.starts_with("??") {
            untracked.push(file_path);
        }
    }

    Ok(WorktreeStatusInfo {
        modified,
        staged,
        untracked,
    })
}

/// Worktree 状态详细信息
#[derive(Debug, Clone)]
pub struct WorktreeStatusInfo {
    pub modified: Vec<String>,
    pub staged: Vec<String>,
    pub untracked: Vec<String>,
}

/// Worktree 构建数据结构
struct WorktreeData {
    path: String,
    branch: String,
    head_commit: Option<String>,
    is_detached: bool,
}

impl WorktreeData {
    fn new(path: String) -> Self {
        WorktreeData {
            path,
            branch: "HEAD".to_string(),
            head_commit: None,
            is_detached: false,
        }
    }

    /// 转换为 Worktree 结构体
    fn to_worktree(self) -> Result<Worktree> {
        // 检查是否为当前 worktree
        let current_path = std::env::current_dir()
            .map_err(|e| WorktreeError::IoError(e))?;
        let is_current = current_path.starts_with(Path::new(&self.path));

        // 从路径推断 worktree 名称
        let name = self.derive_worktree_name();

        // 检查是否为裸仓库
        let is_bare = is_bare_repository().unwrap_or(false);

        // 获取上游分支
        let upstream_branch = self.get_upstream_for_worktree(&name)?;

        Ok(Worktree::new(
            name,
            self.branch,
            self.path,
            is_current,
            is_bare,
            self.is_detached,
            self.head_commit,
            upstream_branch,
        ))
    }

    /// 从路径推断 worktree 名称（基于目录名，不是分支名）
    fn derive_worktree_name(&self) -> String {
        let path = Path::new(&self.path);

        // 检查是否在 .worktrees 目录中
        if let Some(parent) = path.parent() {
            if let Some(dir_name) = parent.file_name() {
                let dir_name_str = dir_name.to_string_lossy();
                if dir_name_str.ends_with(".worktrees") {
                    // 在 worktrees 目录中，使用当前目录名作为 worktree 名称
                    if let Some(name) = path.file_name() {
                        return name.to_string_lossy().to_string();
                    }
                }
            }
        }

        // 检查是否在主仓库中（不包含 .worktrees）
        if !self.path.contains(".worktrees") {
            // 主仓库，使用目录名
            if let Some(name) = path.file_name() {
                return name.to_string_lossy().to_string();
            }
        }

        // 兜底：使用路径的最后一部分
        if let Some(name) = path.file_name() {
            name.to_string_lossy().to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// 获取 worktree 的上游分支
    fn get_upstream_for_worktree(&self, _name: &str) -> Result<Option<String>> {
        // 如果不是当前 worktree，暂时返回 None
        // TODO: 可以通过切换到该 worktree 并运行 git rev-parse @{u} 来获取
        let current_path = std::env::current_dir()
            .map_err(|e| WorktreeError::IoError(e))?;
        let is_current = current_path.starts_with(Path::new(&self.path));

        if is_current {
            get_upstream_branch()
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_get_repository_root() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // 初始化 Git 仓库
        Command::new("git")
            .args(["init"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // 更改当前目录到临时目录
        std::env::set_current_dir(repo_path).unwrap();

        // 获取仓库根目录
        let root = get_repository_root();
        assert!(root.is_ok());
    }

    #[test]
    fn test_is_inside_repository() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // 初始化 Git 仓库
        Command::new("git")
            .args(["init"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // 在仓库中
        assert!(is_inside_repository(repo_path));

        // 不在仓库中（临时目录的父目录）
        assert!(!is_inside_repository(temp_dir.path().parent().unwrap()));
    }

    #[test]
    fn test_get_current_branch() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // 初始化 Git 仓库并设置默认分支为 main
        Command::new("git")
            .args(["init", "-b", "main"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        std::env::set_current_dir(repo_path).unwrap();

        // 配置用户信息（Git 需要）
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // 创建初始提交
        std::fs::write(repo_path.join("test.txt"), "test content").unwrap();
        Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // 获取当前分支
        let branch = get_current_branch();
        assert!(branch.is_ok());
        assert_eq!(branch.unwrap(), "main");
    }
}
