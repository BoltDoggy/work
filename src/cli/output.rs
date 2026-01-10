use comfy_table::{Table, Cell, Color};
use serde::{Deserialize, Serialize};
use colored::Colorize;

/// 输出格式枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Table,
    Compact,
    Json,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "compact" | "simple" | "short" => OutputFormat::Compact,
            _ => OutputFormat::Table,
        }
    }
}

/// 格式化 worktree 列表为表格
pub fn format_worktree_table(worktrees: Vec<crate::core::worktree::Worktree>) -> String {
    let mut table = Table::new();
    table
        .set_header(vec!["NAME", "BRANCH", "PATH", "CURRENT", "STATUS"])
        .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);

    for wt in worktrees {
        let current_marker = if wt.is_current { "*" } else { "" };
        let status = if wt.is_detached {
            "Detached HEAD"
        } else {
            "Healthy"
        };

        table.add_row(vec![
            Cell::new(&wt.name).fg(Color::Cyan),
            Cell::new(&wt.branch),
            Cell::new(&wt.path),
            Cell::new(current_marker).fg(Color::Green),
            Cell::new(status),
        ]);
    }

    table.to_string()
}

/// 格式化 worktree 列表为简洁格式
pub fn format_worktree_compact(worktrees: Vec<crate::core::worktree::Worktree>) -> String {
    let mut output = String::new();

    for wt in worktrees {
        // 判断是否为主目录（不在 .worktrees 中）
        let is_main = !wt.path.contains(".worktrees");

        // 当前标记：绿色
        let current_marker = if wt.is_current {
            "*".green().to_string()
        } else {
            " ".normal().to_string()
        };

        // 主目录标记：紫色粗体
        let main_marker = if is_main {
            format!("{} ", "⌂".bold().purple())  // 屋顶符号表示主目录
        } else {
            String::new()
        };

        // worktree 名称（目录名）：主目录用紫色，其他用青色
        let name = if is_main {
            wt.name.purple().bold().to_string()
        } else {
            wt.name.cyan().to_string()
        };

        // 当前分支：黄色（如果与目录名不同）
        let branch_info = if wt.branch != wt.name && wt.branch != "HEAD" {
            format!(" on {}", wt.branch.yellow())
        } else if wt.is_detached {
            format!(" on {}", "HEAD".yellow())
        } else {
            String::new()
        };

        // 构建状态标记
        let mut status_markers = Vec::new();
        if wt.has_uncommitted_changes() {
            status_markers.push("modified".red().to_string());
        }
        let status_marker = if status_markers.is_empty() {
            String::new()
        } else {
            format!(" ({})", status_markers.join(", "))
        };

        // 简化显示：目录名 + 分支 + 状态
        output.push_str(&format!(
            "{}{} {}{}{}\n",
            current_marker,
            main_marker,
            name,
            branch_info,
            status_marker
        ));
    }

    output.trim_end().to_string()
}

/// 格式化 worktree 列表为 JSON
pub fn format_worktree_json(worktrees: Vec<crate::core::worktree::Worktree>) -> String {
    serde_json::to_string_pretty(&worktrees).unwrap_or_else(|_| {
        format!("{{\"error\": \"Failed to serialize worktrees\"}}")
    })
}

/// 格式化单个 worktree 的详细信息
pub fn format_worktree_info(worktree: &crate::core::worktree::Worktree) -> String {
    format!(
        "Worktree: {}
  Branch: {}
  Path: {}
  HEAD: {}
  Current: {}
  Detached: {}
{}{}",
        worktree.name,
        worktree.branch,
        worktree.path,
        worktree.head_commit.as_ref().unwrap_or(&"N/A".to_string()),
        if worktree.is_current { "Yes" } else { "No" },
        if worktree.is_detached { "Yes" } else { "No" },
        if let Some(upstream) = &worktree.upstream_branch {
            format!("  Upstream: {}\n", upstream)
        } else {
            String::new()
        },
        format!("  Last Modified: {}\n", worktree.last_modified.format("%Y-%m-%d %H:%M:%S"))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::worktree::Worktree;

    #[test]
    fn test_format_worktree_table() {
        let worktrees = vec![
            Worktree::new(
                "main".to_string(),
                "main".to_string(),
                "/home/user/project".to_string(),
                true,
                false,
                false,
                Some("abc123".to_string()),
                Some("origin/main".to_string()),
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

        let output = format_worktree_table(worktrees);
        assert!(output.contains("main"));
        assert!(output.contains("feature-auth"));
        assert!(output.contains("*")); // current marker
    }

    #[test]
    fn test_format_worktree_json() {
        let worktrees = vec![Worktree::new(
            "main".to_string(),
            "main".to_string(),
            "/home/user/project".to_string(),
            true,
            false,
            false,
            Some("abc123".to_string()),
            Some("origin/main".to_string()),
        )];

        let json = format_worktree_json(worktrees);
        assert!(json.contains("\"name\":\"main\""));
        assert!(json.contains("\"is_current\":true"));
    }

    #[test]
    fn test_output_format_from_str() {
        assert!(matches!(OutputFormat::from_str("json"), OutputFormat::Json));
        assert!(matches!(OutputFormat::from_str("JSON"), OutputFormat::Json));
        assert!(matches!(OutputFormat::from_str("table"), OutputFormat::Table));
        assert!(matches!(OutputFormat::from_str("invalid"), OutputFormat::Table));
    }
}
