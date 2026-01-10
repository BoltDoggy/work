# Implementation Plan: Git Worktree 管理工具

**Branch**: `001-git-worktree-cli` | **Date**: 2026-01-10 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-git-worktree-cli/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

本功能旨在创建一个命令行工具 `work`，简化 Git worktree 的管理操作。工具将提供直观的界面来列出、切换、创建和删除 worktree，相比直接使用 `git worktree` 命令减少 50% 的操作步骤。工具将支持交互式选择和命令行参数两种操作方式，并输出人类可读和机器可解析（JSON）两种格式。

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**:
- CLI 框架：clap 4.x (参数解析和命令行界面)
- Git 操作：git2 0.18.x (Rust 的 Git 库绑定)
- 交互式选择：inquire 0.7.x (终端交互式选择器)
- 输出格式：serde_json 1.x (JSON 序列化)
- 错误处理：anyhow 1.x (错误上下文)
- 日志：env_logger 0.11.x (灵活日志记录)

**Storage**: N/A (工具直接操作 Git worktree，无持久化存储)
**Testing**: cargo test (Rust 内置测试框架)
**Target Platform**:
- 主要：Linux, macOS (Unix-like 系统)
- 次要：Windows (Git Bash/WSL 环境)
- 构建目标：x86_64-unknown-linux-gnu, x86_64-apple-darwin, x86_64-pc-windows-msvc

**Project Type**: single (单一 CLI 工具项目)
**Performance Goals**:
- 列出 20+ worktree: < 2 秒
- 创建/切换 worktree: < 5 秒
- 启动时间: < 100ms (冷启动)

**Constraints**:
- 必须在 Git 仓库环境中运行
- 需要足够的磁盘空间创建新 worktree
- 网络：可选（创建基于远程分支的 worktree 时）
- 终端：支持 ANSI 转义序列（用于交互式界面）

**Scale/Scope**:
- 单用户工具（本地运行）
- 支持 100+ worktree 的大型仓库
- 单个可执行文件，无外部依赖（除了 Git 本身）

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. 规范优先开发

✅ **PASS** - 规范已完整定义用户需求（WHAT）和理由（WHY），本计划将记录技术实现决策（HOW）

### II. 独立可交付性

✅ **PASS** - 用户故事按优先级排序（P1/P2/P3），P1 可独立作为 MVP：
- P1 (列出和切换) 可以独立交付和使用
- P2 (创建和删除) 在 P1 基础上添加生命周期管理
- P3 (信息和管理) 提供高级功能

### III. 质量前置

✅ **PASS** - 规范已通过完整性验证（所有需求明确、无 NEEDS CLARIFICATION 标记）

**复杂度追踪**: 无违规 - 工具保持简洁，使用 Rust 标准库和成熟第三方库，无过度设计

## Project Structure

### Documentation (this feature)

```text
specs/001-git-worktree-cli/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
└── contracts/           # N/A (CLI tool, no API contracts)
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI 入口点和命令路由
├── cli/
│   ├── mod.rs           # CLI 模块导出
│   ├── commands.rs      # 命令定义和参数解析 (clap)
│   └── output.rs        # 输出格式化（表格、JSON）
├── core/
│   ├── mod.rs           # 核心模块导出
│   ├── worktree.rs      # Worktree 实体和操作
│   ├── repository.rs    # Git 仓库抽象
│   └── git_ops.rs       # Git 操作封装（基于 git2）
└── utils/
    ├── mod.rs           # 工具模块导出
    ├── errors.rs        # 错误类型和处理
    └── path.rs          # 路径处理工具

tests/
├── integration/
│   ├── list_tests.rs    # 列出 worktree 集成测试
│   ├── switch_tests.rs  # 切换 worktree 集成测试
│   └── create_tests.rs  # 创建 worktree 集成测试
└── unit/
    ├── worktree_tests.rs    # Worktree 实体单元测试
    └── git_ops_tests.rs     # Git 操作单元测试

Cargo.toml               # Rust 项目配置
README.md                # 用户文档
```

**Structure Decision**: 选择单一项目结构（Option 1），因为这是一个独立的 CLI 工具。代码组织采用分层架构：
- `cli/` 层：处理用户交互和命令行界面
- `core/` 层：核心业务逻辑（worktree 管理、Git 操作）
- `utils/` 层：共享工具函数（错误处理、路径处理）

这种分层确保关注点分离，便于测试和维护。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

无违规 - 项目遵循简洁原则，使用成熟的标准库和第三方库，无不必要的复杂度。
