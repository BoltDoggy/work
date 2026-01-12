// T038: 集成测试 - 分支名斜杠转换功能
//
// 测试场景：
// 1. 创建包含斜杠的分支的 worktree
// 2. 检测目录名冲突
// 3. 验证列表输出格式
//
// 注意：这些测试需要已编译的 work 二进制文件
// 运行前请确保执行: cargo build --release

use std::process::Command;
use tempfile::TempDir;
use std::path::{Path, PathBuf};

/// 辅助函数：查找 work 二进制文件路径
fn find_work_binary() -> Option<PathBuf> {
    // 尝试多个可能的位置
    let possible_paths = vec![
        PathBuf::from("./target/release/work"),
        PathBuf::from("../target/release/work"),
        PathBuf::from("../../target/release/work"),
    ];

    for path in possible_paths {
        if path.exists() {
            return Some(path.canonical().ok()?);
        }
    }

    None
}

/// 辅助函数：执行 work 命令
fn run_work_command(args: &[&str], repo_path: &Path) -> Result<String, String> {
    let work_binary = find_work_binary()
        .ok_or_else(|| "work binary not found. Run 'cargo build --release' first.".to_string())?;

    let output = Command::new(&work_binary)
        .args(args)
        .current_dir(repo_path)
        .output()
        .map_err(|e| format!("Failed to execute work command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(format!("Work command failed: {}\nStderr: {}", stdout, stderr));
    }

    Ok(stdout)
}

/// 集成测试 1: 创建包含斜杠的分支的 worktree
#[test]
fn integration_test_create_slash_branch_worktree() {
    // 创建临时目录作为 Git 仓库
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo_path = temp_dir.path();

    // 初始化 Git 仓库
    Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to init git repo");

    // 配置 Git 用户
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to config git user");

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to config git email");

    // 创建初始提交
    std::fs::write(repo_path.join("test.txt"), "test content").expect("Failed to write test file");
    Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git add");

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git commit");

    // 测试场景 1: 创建包含单个斜杠的分支的 worktree
    let result = run_work_command(&["create", "feat/test-feature"], repo_path);
    assert!(result.is_ok(), "Should create worktree for feat/test-feature");

    // 验证目录名被转换为 feat-test-feature
    let worktree_path = repo_path.join("repo.worktrees").join("feat-test-feature");
    assert!(worktree_path.exists(), "Worktree directory should be feat-test-feature");

    // 验证 Git 分支名仍然是 feat/test-feature
    let output = Command::new("git")
        .args(["branch", "--list", "feat/test-feature"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to list branches");

    let branch_list = String::from_utf8_lossy(&output.stdout);
    assert!(branch_list.contains("feat/test-feature"), "Git branch should be feat/test-feature");

    // 测试场景 2: 创建包含多个斜杠的分支的 worktree
    let result = run_work_command(&["create", "feature/auth/oauth"], repo_path);
    assert!(result.is_ok(), "Should create worktree for feature/auth/oauth");

    // 验证目录名被转换为 feature-auth-oauth
    let worktree_path = repo_path.join("repo.worktrees").join("feature-auth-oauth");
    assert!(worktree_path.exists(), "Worktree directory should be feature-auth-oauth");

    // 验证 Git 分支名仍然是 feature/auth/oauth
    let output = Command::new("git")
        .args(["branch", "--list", "feature/auth/oauth"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to list branches");

    let branch_list = String::from_utf8_lossy(&output.stdout);
    assert!(branch_list.contains("feature/auth/oauth"), "Git branch should be feature/auth/oauth");
}

/// 集成测试 2: 列表输出格式验证
#[test]
fn integration_test_list_output_format() {
    // 创建临时目录作为 Git 仓库
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo_path = temp_dir.path();

    // 初始化 Git 仓库
    Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to init git repo");

    // 配置 Git 用户
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to config git user");

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to config git email");

    // 创建初始提交
    std::fs::write(repo_path.join("test.txt"), "test content").expect("Failed to write test file");
    Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git add");

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git commit");

    // 创建包含斜杠的分支的 worktree
    run_work_command(&["create", "feat/feature-001"], repo_path).expect("Failed to create worktree");

    // 测试 compact 格式输出
    let output = run_work_command(&["list"], repo_path).expect("Failed to run work list");
    assert!(output.contains("feat-feature-001"), "Should show dirname feat-feature-001");
    assert!(output.contains("feat/feature-001"), "Should show branch feat/feature-001");

    // 测试 table 格式输出
    let output = run_work_command(&["list", "-o", "table"], repo_path).expect("Failed to run work list -o table");
    assert!(output.contains("feat-feature-001"), "Table should show dirname");

    // 测试 JSON 格式输出
    let output = run_work_command(&["list", "-o", "json"], repo_path).expect("Failed to run work list -o json");
    assert!(output.contains("\"dirname\": \"feat-feature-001\""), "JSON should include dirname");
    assert!(output.contains("\"branch_name\": \"feat/feature-001\""), "JSON should include branch_name");
}

/// 集成测试 3: 目录名冲突检测
#[test]
fn integration_test_dirname_conflict_detection() {
    // 创建临时目录作为 Git 仓库
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo_path = temp_dir.path();

    // 初始化 Git 仓库
    Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to init git repo");

    // 配置 Git 用户
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to config git user");

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to config git email");

    // 创建初始提交
    std::fs::write(repo_path.join("test.txt"), "test content").expect("Failed to write test file");
    Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git add");

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output()
        .expect("Failed to git commit");

    // 创建第一个 worktree: feat/feature-001 -> feat-feature-001
    run_work_command(&["create", "feat/feature-001"], repo_path).expect("Failed to create first worktree");

    // 尝试创建第二个 worktree: feat/feature-001 (冲突)
    // 由于目录名相同，应该被拒绝
    let result = run_work_command(&["create", "feat/feature-001"], repo_path);

    // 第二次创建应该失败（目录名冲突）
    assert!(result.is_err(), "Should reject duplicate dirname");

    // 错误消息应该包含冲突信息
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("conflict") || error_msg.contains("already exists"),
            "Error message should mention conflict");
}
