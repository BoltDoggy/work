# Implementation Plan: Worktree Branch Source Selection

**Branch**: `005-worktree-branch-source` | **Date**: 2025-01-16 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-worktree-branch-source/spec.md`

## Summary

为 work CLI 工具的 `create` 命令添加分支来源选择功能，允许用户在创建新 worktree 时选择三种分支来源：1）基于当前目录分支，2）基于主目录分支，3）自定义输入分支。这将通过扩展现有交互式选择界面和添加新的 Git 操作函数来实现，使用 Rust 和 dialoguer 库提供友好的用户体验。

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: clap 4.5 (CLI), dialoguer 0.11 (交互式界面), anyhow 1.0 (错误处理)
**Storage**: 无持久化存储（仅 Git worktree 元数据）
**Testing**: tempfile 3.10, assert_cmd 2.0, predicates 3.1
**Target Platform**: macOS, Linux, Unix-like systems (通过 `std::process::Command` 调用系统 Git)
**Project Type**: 单项目 CLI 工具 (single)
**Performance Goals**: 命令执行时间 < 2 秒（SC-005），交互式选择响应 < 100ms
**Constraints**: 必须与现有代码向后兼容，保持现有 `--branch` 和 `--interactive` 参数行为
**Scale/Scope**: 中小型 CLI 工具，~500 LOC 预期增量

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. 规范优先开发 ✅ PASS

- **Compliance**: 功能规格完整，包含所有必需章节（用户场景、需求、成功标准）
- **WHAT vs WHY**: 规范明确描述了用户需要什么（选择分支来源）和为什么（提升便捷性和灵活性），没有涉及实现细节
- **Testable Requirements**: 所有 12 个功能需求（FR-001 至 FR-012）都是可测试的
- **Technical Decisions**: 本计划文档将记录所有技术决策

### II. 独立可交付性 ✅ PASS

- **User Story Independence**:
  - P1 (基于当前分支) 可独立测试和交付
  - P2 (基于主目录分支) 可独立测试和交付
  - P3 (自定义分支) 可独立测试和交付
- **No Cross-Story Dependencies**: 每个用户故事都可以单独实现和测试，不依赖其他用户故事
- **Prioritization Clear**: P1 → P2 → P3 优先级明确，支持 MVP 交付策略

### III. 质量前置 ✅ PASS

- **Spec Completeness**: 规范通过质量验证，无 [NEEDS CLARIFICATION] 标记
- **Max Clarifications**: 0 个（远低于 3 个限制）
- **Gate Requirements**: 规划阶段将解决所有技术细节，任务拆分将反映用户故事边界

**Overall Gate Status**: ✅ **ALL PASS** - 可以继续 Phase 0 研究

## Project Structure

### Documentation (this feature)

```text
specs/005-worktree-branch-source/
├── plan.md              # 本文件 (/speckit.plan 命令输出)
├── research.md          # Phase 0 输出 (/speckit.plan 命令)
├── data-model.md        # Phase 1 输出 (/speckit.plan 命令)
├── quickstart.md        # Phase 1 输出 (/speckit.plan 命令)
├── contracts/           # Phase 1 输出 (本功能不需要 API 契约)
└── tasks.md             # Phase 2 输出 (/speckit.tasks 命令 - 非本命令创建)
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI 入口，命令处理器 (需要修改 create_command_handler)
├── cli/
│   └── output.rs        # 输出格式化 (无需修改)
├── core/
│   ├── git_ops.rs       # Git 操作包装器 (需要添加新函数)
│   ├── worktree.rs      # Worktree 数据模型 (无需修改)
│   └── repository.rs    # 仓库管理 (可能需要添加函数)
└── utils/
    ├── errors.rs        # 错误类型定义 (可能需要添加新错误类型)
    └── path.rs          # 路径工具 (无需修改)

tests/
├── contract/            # 契约测试 (不适用)
├── integration/         # 集成测试 (需要添加新测试)
└── unit/                # 单元测试 (需要添加新测试)
```

**Structure Decision**: 选择 Option 1: 单项目结构。work CLI 工具是一个独立的 Rust 项目，所有源代码在 `src/` 目录中，按层次组织（CLI、Core、Utils）。这是现有项目结构，本功能将遵循相同的模式。

## Complexity Tracking

> **无宪章违规需要记录** - 所有检查项都通过，本功能不需要额外的复杂度。

## Phase 1 Completion Summary

**Status**: ✅ Phase 1 完成

### 生成的工件

1. **research.md**: 技术研究和设计决策文档
   - 12 个技术决策（TD-001 至 TD-012）
   - 涵盖交互界面、Git 操作、错误处理、测试策略
   - 所有决策基于项目现有架构和依赖

2. **data-model.md**: 数据模型设计文档
   - `BranchSource` 枚举：3 种分支来源选项
   - `WorktreeError` 扩展：4 个新错误变体
   - `BranchInfo` 结构体：分支元数据封装
   - `CreateOptions` 扩展：命令行选项
   - 完整的数据流图和状态转换图

3. **quickstart.md**: 集成测试场景文档
   - 7 大类测试场景
   - 每个场景包含前置条件、操作步骤、预期结果
   - 单元测试、集成测试、端到端测试示例
   - 性能测试标准（< 2 秒响应）

4. **contracts/**: API 契约目录（空，含 README 说明）
   - 本功能不涉及 API 契约

5. **CLAUDE.md**: AI 上下文文件已更新
   - 添加了本功能的技术栈信息
   - 包含 Rust 1.75+, clap 4.5, dialoguer 0.11, anyhow 1.0
   - 项目类型：单项目 CLI 工具
   - 无持久化存储

### 重新评估宪章检查

**Phase 1 后重新评估**: ✅ **所有检查项仍然通过**

#### I. 规范优先开发 ✅

- 所有技术决策已记录在 research.md
- 每个决策都有明确的理由和替代方案分析
- 数据模型清晰分离了 WHAT 和 HOW

#### II. 独立可交付性 ✅

- 用户故事边界清晰反映在数据模型中
- `BranchSource` 枚举支持三种独立的使用场景
- 每个场景可以独立测试和交付

#### III. 质量前置 ✅

- 测试策略覆盖所有场景（quickstart.md）
- 错误处理完整（4 种新错误类型）
- 性能目标明确（< 2 秒响应）

### 设计原则遵循

本功能的设计严格遵循项目现有原则：

1. **使用系统 Git**: 所有 Git 操作通过 `std::process::Command`，避免 `git2` crate
2. **保持简单**: 不引入新依赖，使用现有库（dialoguer, clap, colored）
3. **向后兼容**: 保持现有 `--branch` 和 `--interactive` 参数行为
4. **用户友好**: 彩色输出、清晰错误提示、交互式菜单

### 下一步

Phase 1 和 Phase 0 已完成。可以执行：

```bash
/speckit.tasks
```

这将基于 design artifacts（research.md, data-model.md, quickstart.md）生成可执行的任务列表（tasks.md）。
