# Research Document: Worktree Branch Source Selection

**Feature**: 005-worktree-branch-source
**Date**: 2025-01-16
**Purpose**: 记录技术决策、设计选择和实现方法

## 概述

本文档记录了为 work CLI 工具添加分支来源选择功能的技术研究和设计决策。主要目标是实现三种分支来源选项：当前目录分支、主目录分支和自定义分支。

## 技术决策

### TD-001: 交互式分支来源选择界面

**决策**: 使用 dialoguer 库的 `Select` 组件实现交互式菜单

**理由**:
- 项目已使用 dialoguer 0.11 (见 Cargo.toml 和现有代码中的交互式选择)
- 保持一致的 UI 风格和用户体验
- 避免引入新的依赖
- dialoguer 的 Select 组件支持键盘导航和可配置主题

**替代方案**:
1. 使用 inquire 库: 功能更丰富，但需要新增依赖
2. 自定义 TUI 界面: 开发成本高，过度设计
3. 命令行参数组合: 不够直观，用户体验差

**实现细节**:
```rust
use dialoguer::{theme::ColorfulTheme, Select};

let items = vec![
    "基于当前目录分支",
    "基于主目录分支",
    "自定义输入分支"
];

let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("选择分支来源")
    .items(&items)
    .default(0)
    .interact()?;
```

### TD-002: 获取主仓库分支信息

**决策**: 通过 `git rev-parse --git-common-dir` 定位主仓库，然后在主目录执行 git 命令

**理由**:
- `git-common-dir` 指向主仓库的 `.git` 目录，无论当前在哪个 worktree
- 现有代码已使用此方法 (见 main.rs:180-201)
- Git 官方推荐的方式，兼容所有 worktree 场景
- 无需解析 `.git` 文件中的 `gitdir` 引用

**实现方法**:
```rust
// 1. 获取 git-common-dir
let git_common_dir = Command::new("git")
    .args(["rev-parse", "--git-common-dir"])
    .output()?;

// 2. 定位主仓库目录
let main_repo = git_common_dir.parent(); // .git 的父目录即主仓库

// 3. 在主目录执行 git 命令获取当前分支
let current_branch = Command::new("git")
    .args(["-C", main_repo, "rev-parse", "--abbrev-ref", "HEAD"])
    .output()?;
```

**替代方案**:
1. 解析 `.git/worktrees/<name>/gitdir`: 复杂且易出错
2. 使用 `git2` crate: 违反项目设计决策（避免 OpenSSL 依赖）
3. 假设固定路径结构: 不适用于所有 Git 配置

### TD-003: 自定义分支输入界面

**决策**: 使用 dialoguer 的 `Input` 组件让用户输入分支名

**理由**:
- 与现有 UI 风格一致
- 支持输入验证和默认值
- 提供友好的错误提示
- 避免手动处理终端输入的复杂性

**实现细节**:
```rust
use dialoguer::{theme::ColorfulTheme, Input};

let branch_name = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("输入分支名称")
    .allow_empty(false)  // 不允许空输入
    .interact()?;
```

**验证逻辑**:
- 检查分支是否存在于本地或远程
- 提供清晰的错误信息（如"分支 'xxx' 不存在"）
- 显示可用分支列表帮助用户

### TD-004: 分支存在性验证

**决策**: 扩展现有 `branch_exists()` 函数，同时支持本地和远程分支验证

**理由**:
- 现有 `git_ops.rs:branch_exists()` 仅检查本地分支
- 规格要求支持远程分支（如 `origin/feature`）
- 需要统一的验证接口

**实现方法**:
```rust
pub fn branch_exists_local(branch_name: &str) -> bool {
    // 现有逻辑：检查 refs/heads/
}

pub fn branch_exists_remote(branch_name: &str) -> bool {
    // 新增：检查 refs/remotes/
    Command::new("git")
        .args(["show-ref", "--verify", "--quiet",
               &format!("refs/remotes/{}", branch_name)])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn branch_exists_any(branch_name: &str) -> bool {
    branch_exists_local(branch_name) ||
    branch_exists_remote(branch_name.trim_start_matches("origin/"))
}
```

### TD-005: 处理 Detached HEAD 状态

**决策**: 当主目录或当前目录处于 detached HEAD 状态时，显示友好的错误提示并建议解决方案

**理由**:
- 规格 FR-009 要求处理此情况
- Detached HEAD 意味着没有分支名称，无法作为基准
- 用户需要明确的指导

**错误提示设计**:
```text
错误: 主目录处于 detached HEAD 状态，无法作为分支来源

当前状态: HEAD 指向 abc1234

建议的解决方案:
1. 切换到一个分支: cd /path/to/main && git checkout <branch>
2. 选择其他分支来源（当前分支、自定义分支）
3. 先在主目录创建或切换分支
```

### TD-006: 命令行参数扩展

**决策**: 添加新的命令行参数 `--branch-source` (或 `-bs`) 直接指定分支来源，跳过交互式选择

**理由**:
- 规格假设 #4 提到此需求（虽非 P1）
- 提高脚本使用效率
- 保持向后兼容（可选参数）

**参数设计**:
```rust
Create {
    name: String,
    branch: Option<String>,
    path: Option<String>,
    interactive: bool,
    branch_source: Option<String>,  // 新增: "current", "main", "custom"
}

// 使用示例:
// work create feature-x --branch-source main
// work create feature-y -bs current
```

**优先级**: P2（非 MVP 必需，可在后续版本实现）

### TD-007: 默认行为

**决策**: 当用户未指定分支来源且未启用交互模式时，默认使用"基于当前目录分支"

**理由**:
- 规格假设 #6 建议此默认行为
- 保守策略：保持与现有行为一致（当前无选项时基于当前分支）
- 最少惊讶原则：用户通常期望基于当前工作上下文

**实现逻辑**:
```rust
let branch_source = if let Some(source) = args.branch_source {
    parse_branch_source(source)?
} else if args.interactive {
    prompt_branch_source()?
} else {
    BranchSource::Current  // 默认值
};
```

### TD-008: 远程分支自动跟踪

**决策**: 当用户输入远程分支名（如 `origin/feature`）时，自动创建本地同名分支并设置跟踪

**理由**:
- 规格 FR-010 和 FR-012 要求此行为
- Git worktree 原生支持：`git worktree add <path> <remote-branch>`
- 简化用户操作流程

**实现方法**:
```rust
// Git 自动处理远程分支跟踪
let output = Command::new("git")
    .args(["worktree", "add", path, "origin/feature-remote"])
    .output()?;

// Git 会自动:
// 1. 创建本地分支 "feature-remote"
// 2. 设置上游为 "origin/feature-remote"
// 3. 检出该分支
```

**注意事项**:
- 需要验证远程分支确实存在
- 错误提示需要区分"远程分支不存在"和"网络问题"

### TD-009: 用户界面流程优化

**决策**: 分支来源选择后，如果是"自定义分支"选项，立即显示分支输入提示

**理由**:
- 减少用户等待时间
- 避免额外的确认步骤
- 符合现有交互模式（见 main.rs:238-253 的交互式分支选择）

**流程设计**:
```text
用户执行: work create feature-x

1. 显示菜单:
   ❯ 基于当前目录分支 (main)
     基于主目录分支 (develop)
     自定义输入分支

2a. 用户选择"当前分支" → 直接创建 worktree

2b. 用户选择"主目录分支" → 直接创建 worktree

2c. 用户选择"自定义分支" → 显示输入提示:
   › 输入分支名称: origin/feature-remote
   → 验证分支存在 → 创建 worktree
```

### TD-010: 错误处理和用户反馈

**决策**: 使用 colored 库提供彩色、结构化的错误信息

**理由**:
- 现有代码已使用 colored (见 main.rs)
- 提高错误信息的可读性
- 保持一致的视觉风格

**错误信息格式**:
```rust
use colored::*;

eprintln!("{}", "错误: 分支不存在".red().bold());
eprintln!();
eprintln!("{}", "您输入的分支 'feature-xyz' 在本地和远程都不存在".dimmed());
eprintln!();
eprintln!("{}", "可用的本地分支:".yellow().bold());
for branch in local_branches {
    eprintln!("  {}", branch.dimmed());
}
eprintln!();
eprintln!("{}", "可用的远程分支:".yellow().bold());
for branch in remote_branches {
    eprintln!("  {}", branch.dimmed());
}
```

### TD-011: 向后兼容性

**决策**: 保持现有 `--branch` 和 `--interactive` 参数的行为不变

**理由**:
- 避免破坏现有用户的工作流程
- 现有行为已经有明确的语义
- 新功能通过新增参数提供，不修改现有参数

**兼容性矩阵**:

| 参数组合 | 旧行为 | 新行为 |
|---------|--------|--------|
| `work create name` | 基于当前分支创建新分支 | 保持不变 |
| `work create name -b main` | 基于 main 分支创建 | 保持不变 |
| `work create name -i` | 交互式选择分支 | 增强为：选择分支来源 → 选择具体分支 |
| `work create name -bs main` | N/A | 新增：直接基于主目录分支 |

### TD-012: 测试策略

**决策**: 采用三层测试策略：单元测试、集成测试、端到端测试

**理由**:
- 确保每个函数的正确性
- 验证与 Git 的集成
- 测试完整的用户场景

**测试范围**:

1. **单元测试** (tests/unit/):
   - `get_main_repo_branch()`: 测试获取主目录分支
   - `branch_exists_remote()`: 测试远程分支验证
   - `parse_branch_source()`: 测试分支来源解析

2. **集成测试** (tests/integration/):
   - 创建 worktree 基于当前分支
   - 创建 worktree 基于主目录分支
   - 创建 worktree 基于自定义分支
   - Detached HEAD 错误处理
   - 远程分支自动跟踪

3. **端到端测试** (使用 assert_cmd):
   - 完整的 CLI 命令执行流程
   - 错误信息和用户反馈
   - 性能测试（< 2 秒响应）

## 设计模式

### 枚举类型设计

```rust
/// 分支来源选项
#[derive(Debug, Clone, PartialEq)]
pub enum BranchSource {
    /// 基于当前目录分支
    Current,
    /// 基于主目录分支
    Main,
    /// 自定义分支名称
    Custom(String),
}
```

### 错误类型扩展

```rust
/// 在 utils/errors.rs 中添加新错误类型
pub enum WorktreeError {
    // ... 现有错误类型

    /// 主目录处于 detached HEAD 状态
    MainRepoDetachedHead {
        commit: String,
    },

    /// 分支不存在（本地和远程）
    BranchNotFound {
        branch: String,
        available_locals: Vec<String>,
        available_remotes: Vec<String>,
    },

    /// 无效的分支来源选项
    InvalidBranchSource {
        input: String,
    },
}
```

## 性能考虑

### 响应时间目标

- 交互式菜单显示: < 100ms
- 分支存在性验证: < 500ms
- 完整的 worktree 创建: < 2 秒

### 优化策略

1. **缓存分支列表**: 避免多次执行 `git branch` 命令
2. **并行执行**: 当前分支和主目录分支获取可以并行（如果需要）
3. **早期验证**: 在用户输入时立即验证，减少等待时间

## 依赖更新

无需新增外部依赖，所有功能使用现有的：
- clap 4.5 (CLI 解析)
- dialoguer 0.11 (交互界面)
- anyhow 1.0 (错误处理)
- colored 2.1 (彩色输出)

## 安全性考虑

### 输入验证

1. **分支名验证**: 检查空输入、特殊字符、路径遍历
2. **路径验证**: 确保主目录路径有效
3. **Git 命令注入**: 使用参数化调用，避免字符串拼接

### 错误信息

- 不泄露敏感信息（如本地文件路径，除非用户明确操作）
- 提供有用的调试信息但不暴露内部实现

## 国际化

当前版本仅支持中文，错误信息和提示文本都是中文。未来可以考虑：
- 使用 i18n 库支持多语言
- 从配置文件读取语言偏好
- 检测系统语言设置

**当前实现**: 硬编码中文文本（符合项目现状）

## 实现优先级

### P1 (MVP 必需)

- TD-001: 交互式分支来源选择界面
- TD-002: 获取主仓库分支信息
- TD-003: 自定义分支输入界面
- TD-004: 分支存在性验证
- TD-005: Detached HEAD 处理
- TD-007: 默认行为
- TD-008: 远程分支自动跟踪
- TD-009: 用户界面流程优化
- TD-010: 错误处理和用户反馈
- TD-011: 向后兼容性

### P2 (后续版本)

- TD-006: 命令行参数 `--branch-source`
- TD-012: 完整的测试套件

### P3 (未来增强)

- 国际化支持
- 性能优化（缓存）
- 历史记录和智能推荐

## 总结

本功能的核心技术挑战在于：
1. **准确定位主仓库**: 使用 `git rev-parse --git-common-dir`
2. **友好的交互界面**: 基于 dialoguer 实现一致的用户体验
3. **完整的错误处理**: 覆盖所有边界情况并提供有用的反馈
4. **向后兼容**: 不破坏现有工作流程

所有决策都遵循项目现有的设计原则：
- 使用系统 Git 而非 `git2` crate
- 保持简单的 CLI 交互模式
- 优先考虑用户体验而非功能丰富度
- 最小化依赖和复杂度
