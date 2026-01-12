use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;

mod cli;
mod core;
mod utils;

use cli::output::{OutputFormat, format_worktree_table, format_worktree_compact, format_worktree_json};
use core::git_ops::{list_worktrees, create_worktree, create_worktree_with_new_branch, delete_worktree, branch_exists, prune_worktrees, get_worktree_status};
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use std::path::Path;
use colored::Colorize;

/// 一个简化的 Git worktree 管理工具
#[derive(Parser, Debug)]
#[command(name = "work")]
#[command(author = "BoltDoggy <github@robolt.uu.me>")]
#[command(version = "0.1.7")]
#[command(about = "简化 Git worktree 的管理", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 列出所有 worktree
    #[command(alias = "ls")]
    List {
        /// 输出格式 (table, compact, json)
        #[arg(short = 'o', long = "output", default_value = "compact")]
        output_format: String,
    },
    /// 切换到指定的 worktree
    Switch {
        /// Worktree 名称
        name: Option<String>,
        /// 输出 worktree 路径供 shell 集成使用
        #[arg(long = "print-path")]
        print_path: bool,
    },
    /// 创建新的 worktree
    #[command(alias = "new")]
    Create {
        /// 分支名或 worktree 名称
        name: String,
        /// 基准分支（用于创建新分支）
        #[arg(short = 'b', long = "branch")]
        branch: Option<String>,
        /// 自定义路径
        #[arg(short = 'p', long = "path")]
        path: Option<String>,
        /// 交互式选择基准分支
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,
    },
    /// 删除 worktree
    #[command(alias = "rm")]
    Delete {
        /// Worktree 名称（可指定多个）
        #[arg(required = false)]
        names: Vec<String>,
        /// 强制删除（忽略未提交的更改）
        #[arg(short = 'f', long = "force")]
        force: bool,
        /// 交互式选择要删除的 worktree
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,
    },
    /// 显示 worktree 详细信息
    #[command(alias = "show")]
    Info {
        /// Worktree 名称
        name: String,
        /// 输出格式 (table 或 json)
        #[arg(short = 'o', long = "output", default_value = "table")]
        output_format: String,
    },
    /// 清理无效的 worktree
    Prune {
        /// 预览将要清理的 worktree（不实际删除）
        #[arg(long = "dry-run")]
        dry_run: bool,
    },
}

fn main() -> Result<()> {
    // 初始化日志
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    let args = Args::parse();

    info!("执行 work 命令: {:?}", args.command);

    match args.command {
        Commands::List { output_format } => {
            list_command_handler(&output_format)
        }
        Commands::Switch { name, print_path } => {
            switch_command_handler(name.as_deref(), print_path)
        }
        Commands::Create { name, branch, path, interactive } => {
            create_command_handler(&name, branch.as_deref(), path.as_deref(), interactive)
        }
        Commands::Delete { names, force, interactive } => {
            delete_command_handler(&names, force, interactive)
        }
        Commands::Info { name, output_format } => {
            info_command_handler(&name, &output_format)
        }
        Commands::Prune { dry_run } => {
            prune_command_handler(dry_run)
        }
    }
}

/// 处理 list 命令
fn list_command_handler(output_format: &str) -> Result<()> {
    let worktrees = list_worktrees()?;

    let format = OutputFormat::from_str(output_format);

    match format {
        OutputFormat::Table => {
            println!("{}", format_worktree_table(worktrees));
        }
        OutputFormat::Compact => {
            println!("{}", format_worktree_compact(worktrees));
        }
        OutputFormat::Json => {
            println!("{}", format_worktree_json(worktrees));
        }
    }

    Ok(())
}

/// 处理 switch 命令
fn switch_command_handler(name: Option<&str>, print_path: bool) -> Result<()> {
    let worktrees = list_worktrees()?;

    let target_worktree = if let Some(n) = name {
        // 查找指定的 worktree
        worktrees
            .iter()
            .find(|wt| wt.dirname == n)
            .ok_or_else(|| anyhow::anyhow!("Worktree '{}' not found", n))?
    } else {
        // 交互式选择
        // TODO: 使用 inquire 实现交互式选择
        // 现在简单地选择第一个非当前的 worktree
        worktrees
            .iter()
            .find(|wt| !wt.is_current)
            .ok_or_else(|| anyhow::anyhow!("No other worktrees available"))?
    };

    if print_path {
        // 只输出路径供 shell 使用
        println!("{}", target_worktree.path);
    } else {
        // 输出友好的切换提示
        println!("切换到 worktree: {}", target_worktree.dirname);
        println!("路径: {}", target_worktree.path);
        println!("\n提示: 使用 eval \"$(work switch {} --print-path)\" 自动切换目录", target_worktree.dirname);
    }

    Ok(())
}

/// 处理 create 命令
fn create_command_handler(name: &str, branch: Option<&str>, path: Option<&str>, interactive: bool) -> Result<()> {
    let worktrees = list_worktrees()?;

    // 获取主仓库的 .git 目录（不是当前 worktree 的 .git 文件）
    // 使用 git rev-parse --git-common-dir 来找到主仓库
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--git-common-dir"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to find git directory: {}", e))?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Not in a git repository"));
    }

    let git_common_dir_cow = String::from_utf8_lossy(&output.stdout);
    let git_common_dir_relative = git_common_dir_cow.trim();

    // 如果是相对路径，需要转换为绝对路径
    let current_dir = std::env::current_dir()?;
    let git_dir = if git_common_dir_relative.starts_with("/") || git_common_dir_relative.starts_with(".") {
        // 绝对路径或相对路径，需要规范化
        current_dir.join(git_common_dir_relative).canonicalize()
            .unwrap_or_else(|_| current_dir.join(git_common_dir_relative))
    } else {
        // 可能是简单的 ".git"，需要基于当前目录
        current_dir.join(git_common_dir_relative)
    };

    // git_common_dir 应该指向主仓库的 .git 目录
    // 它的父目录就是主仓库所在目录
    let repo_root = git_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine repository root"))?;

    // 确定路径
    // name 参数实际上是分支名（可能包含斜杠）
    // 转换分支名得到目录名
    let dirname = core::git_ops::branch_to_dirname(name);

    let worktree_path = if let Some(custom_path) = path {
        custom_path.to_string()
    } else {
        // 获取主仓库目录的名称
        let dir_name = repo_root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("repo");

        // 在主仓库目录的同一级创建 <dirname>.worktrees 目录
        let worktrees_parent = repo_root
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine parent directory"))?;

        let worktrees_dir_name = format!("{}.worktrees", dir_name);
        worktrees_parent.join(worktrees_dir_name).join(&dirname).to_string_lossy().to_string()
    };

    // 检查 worktree 是否已存在（使用转换后的目录名）
    if worktrees.iter().any(|wt| wt.dirname == dirname) {
        return Err(anyhow::anyhow!("Worktree '{}' already exists", dirname));
    }

    // 交互式选择基准分支
    let base_branch = if interactive {
        let branches = core::git_ops::list_local_branches()?;
        if branches.is_empty() {
            return Err(anyhow::anyhow!("No branches available"));
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select base branch")
            .items(&branches)
            .default(0)
            .interact()?;

        Some(branches[selection].clone())
    } else {
        branch.map(|b| b.to_string())
    };

    // 创建 worktree
    if let Some(base) = base_branch {
        // 基于现有分支创建
        if !branch_exists(&base) {
            return Err(anyhow::anyhow!("Branch '{}' does not exist", base));
        }

        match create_worktree(&base, &worktree_path) {
            Ok(_) => {
                // T021: 显示成功消息，包含目录名和分支名
                if dirname == name {
                    // 无转换（分支名无斜杠）
                    println!("{} {} from branch {}",
                        "Created worktree".green().bold(),
                        dirname.cyan().bold(),
                        base.yellow()
                    );
                } else {
                    // 有转换（分支名有斜杠）
                    println!("{} {} (directory: {}) from branch {}",
                        "Created worktree".green().bold(),
                        name.cyan().bold(),
                        dirname.cyan().dimmed(),
                        base.yellow()
                    );
                }
            }
            Err(e) => {
                // T020: 处理 DirNameConflict 错误
                if let crate::utils::errors::WorktreeError::DirNameConflict { dirname: conflict_dirname, existing_branch } = e {
                    eprintln!("{}", "Error: Cannot create worktree - directory name conflict".red().bold());
                    eprintln!();
                    eprintln!("The branch '{}' would create directory '{}',", name, conflict_dirname);
                    eprintln!("which conflicts with existing worktree for branch '{}'.", existing_branch);
                    eprintln!();
                    eprintln!("{}", "Suggested solutions:".yellow().bold());
                    eprintln!("  1. Use a different branch name");
                    eprintln!("  2. Delete the existing worktree with: work delete {}", conflict_dirname);
                    return Err(anyhow::anyhow!("Directory name conflict"));
                }
                return Err(e.into());
            }
        }
    } else {
        // 创建新分支
        let upstream = branch.map(|b| b.to_string());
        match create_worktree_with_new_branch(name, &worktree_path, upstream.as_deref()) {
            Ok(_) => {
                // T021: 显示成功消息
                if dirname == name {
                    println!("{} {} with new branch",
                        "Created worktree".green().bold(),
                        dirname.cyan().bold()
                    );
                } else {
                    println!("{} {} (directory: {}) with new branch",
                        "Created worktree".green().bold(),
                        name.cyan().bold(),
                        dirname.cyan().dimmed()
                    );
                }
            }
            Err(e) => {
                // T020: 处理错误
                if let crate::utils::errors::WorktreeError::DirNameConflict { dirname: conflict_dirname, existing_branch } = e {
                    eprintln!("{}", "Error: Cannot create worktree - directory name conflict".red().bold());
                    eprintln!();
                    eprintln!("The branch '{}' would create directory '{}',", name, conflict_dirname);
                    eprintln!("which conflicts with existing worktree for branch '{}'.", existing_branch);
                    eprintln!();
                    eprintln!("{}", "Suggested solutions:".yellow().bold());
                    eprintln!("  1. Use a different branch name");
                    eprintln!("  2. Delete the existing worktree with: work delete {}", conflict_dirname);
                    return Err(anyhow::anyhow!("Directory name conflict"));
                }
                return Err(e.into());
            }
        }
    }

    println!("\n{}: {}", "Path".bold(), worktree_path.dimmed());
    println!("\n{}:", "Switch to this worktree".green());
    println!("  {}", format!("cd {}", worktree_path).dimmed());
    println!("  {}", format!("eval \"$(work switch {} --print-path)\"", name).dimmed());

    Ok(())
}

/// 处理 delete 命令
fn delete_command_handler(names: &[String], force: bool, interactive: bool) -> Result<()> {
    let worktrees = list_worktrees()?;

    // 如果没有指定名称且是交互式模式，显示选择列表
    let targets = if names.is_empty() && interactive {
        let items: Vec<String> = worktrees.iter().map(|wt| wt.dirname.clone()).collect();

        if items.is_empty() {
            return Err(anyhow::anyhow!("No worktrees to delete"));
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select worktree to delete")
            .items(&items)
            .interact()?;

        vec![items[selection].clone()]
    } else if names.is_empty() {
        return Err(anyhow::anyhow!("No worktree names provided. Use --interactive or specify names"));
    } else {
        names.to_vec()
    };

    // 删除每个指定的 worktree
    for name in &targets {
        let worktree = worktrees
            .iter()
            .find(|wt| wt.dirname == *name)
            .ok_or_else(|| anyhow::anyhow!("Worktree '{}' not found", name))?;

        // 检查是否为当前 worktree
        if worktree.is_current {
            return Err(anyhow::anyhow!("Cannot delete current worktree '{}'. Switch to another worktree first.", name));
        }

        // 检查未提交的更改
        if !force && worktree.has_uncommitted_changes() {
            println!("Worktree '{}' has uncommitted changes:", name);

            if !interactive || !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Delete anyway?")
                .default(false)
                .interact()?
            {
                println!("Skipped '{}'", name);
                continue;
            }
        }

        // 确认删除
        if interactive && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(&format!("Delete worktree '{}'?", name))
            .default(false)
            .interact()?
        {
            println!("Cancelled deletion of '{}'", name);
            continue;
        }

        // 执行删除
        delete_worktree(&worktree.path, force)?;
        println!("{} {}", "Deleted worktree".red().bold(), name.cyan());
    }

    Ok(())
}

/// 处理 info 命令
fn info_command_handler(name: &str, output_format: &str) -> Result<()> {
    let worktrees = list_worktrees()?;

    let worktree = worktrees
        .iter()
        .find(|wt| wt.dirname == name)
        .ok_or_else(|| anyhow::anyhow!("Worktree '{}' not found", name))?;

    let format = OutputFormat::from_str(output_format);

    match format {
        OutputFormat::Json => {
            // T034: 输出 JSON 格式，包含 directory 和 branch 字段
            let json_output = format_worktree_json(vec![worktree.clone()]);
            println!("{}", json_output);
        }
        _ => {
            // 输出带颜色的基本信息
            println!("{}: {}", "Worktree".bold().green(), worktree.dirname.cyan().bold());
            println!("  {}: {}", "Branch".bold(), worktree.branch_name.yellow());
            println!("  {}: {}", "Path".bold(), worktree.path.dimmed());
            println!("  {}: {}", "HEAD".bold(), worktree.head_commit.as_ref().unwrap_or(&"N/A".to_string()).dimmed());
            println!("  {}: {}", "Current".bold(), if worktree.is_current { "Yes".green() } else { "No".dimmed() });
            println!("  {}: {}", "Detached".bold(), if worktree.is_detached { "Yes".yellow() } else { "No".dimmed() });
            if let Some(upstream) = &worktree.upstream_branch {
                println!("  {}: {}", "Upstream".bold(), upstream.cyan());
            }
            println!("  {}: {}", "Last Modified".bold(), worktree.last_modified.format("%Y-%m-%d %H:%M:%S").to_string().dimmed());

            // 显示未提交的更改
            let path = Path::new(&worktree.path);
            if let Ok(status) = get_worktree_status(path) {
                if !status.modified.is_empty() || !status.staged.is_empty() || !status.untracked.is_empty() {
                    println!("\n{}:", "Uncommitted Changes".red().bold());

                    if !status.staged.is_empty() {
                        println!("  {}:", "Staged".green());
                        for file in &status.staged {
                            println!("    {} {}", "✓".green(), file.dimmed());
                        }
                    }

                    if !status.modified.is_empty() {
                        println!("  {}:", "Modified".yellow());
                        for file in &status.modified {
                            println!("    {} {}", "M".yellow(), file.dimmed());
                        }
                    }

                    if !status.untracked.is_empty() {
                        println!("  {}:", "Untracked".dimmed());
                        for file in &status.untracked {
                            println!("    {} {}", "?".dimmed(), file.dimmed());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// 处理 prune 命令
fn prune_command_handler(dry_run: bool) -> Result<()> {
    let pruned = prune_worktrees(dry_run)?;

    if pruned.is_empty() {
        println!("{}", "没有需要清理的无效 worktree".dimmed());
    } else {
        if dry_run {
            println!("{}:", "预览模式 - 将要清理的无效 worktree".yellow());
        } else {
            println!("{}:", "已清理以下无效 worktree".green());
        }

        for item in &pruned {
            println!("  {}", item.dimmed());
        }
    }

    Ok(())
}
