# Implementation Plan: 分支名斜杠转换

**Branch**: `003-branch-slash-conversion` | **Date**: 2026-01-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-branch-slash-conversion/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

实现 worktree 创建时的分支名到目录名的自动转换功能。当分支名包含斜杠（`/`）时，系统自动将所有斜杠转换为连字符（`-`）作为 worktree 目录名，同时保持 Git 分支名不变。技术方法是在 `git_ops.rs` 中添加分支名转换函数，在创建 worktree 前调用该函数，并在输出格式化中同时显示目录名和分支名。

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: clap 4.5 (CLI), anyhow 1.0 (错误处理), thiserror 1.0 (自定义错误), serde 1.0 (序列化), comfy-table 7.0 (表格输出), colored 2.1 (颜色输出), chrono 0.4 (时间处理)
**Storage**: 无持久化存储（仅 Git worktree 元数据）
**Testing**: cargo test (单元测试), assert_cmd 2.0 (CLI 测试), predicates 3.1 (断言匹配), tempfile 3.10 (临时文件)
**Target Platform**: macOS, Linux, Windows (跨平台 CLI 工具)
**Project Type**: single (单一 CLI 项目)
**Performance Goals**: 创建 worktree < 1 秒，列表输出 < 100ms
**Constraints**: 必须兼容 Git 2.0+，不能引入 OpenSSL 依赖（使用系统 git 命令）
**Scale/Scope**: 小型 CLI 工具，< 10k LOC，核心功能 < 5 个文件

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. 规范优先开发 (Specification-First Development)

✅ **PASS** - 规范已通过 `/speckit.specify` 创建，包含：
- 清晰的用户需求（WHAT）：创建包含斜杠分支的 worktree 时自动转换目录名
- 明确的业务价值（WHY）：解决分支命名规范与文件系统限制的冲突
- 可测试的需求：10 个功能需求，每个都有明确的验收标准
- 可衡量的成功标准：5 个成功标准，均为可量化的业务指标

技术决策将在 `research.md` 中记录，包括：
- 分支名转换算法的选择
- 冲突检测策略
- 输出格式化方案

### II. 独立可交付性 (Independent Deliverability)

✅ **PASS** - 用户故事已按优先级排序（P1、P2、P3），且每个故事独立可测试：
- **P1 (核心功能)**: 创建包含斜杠分支的 worktree - 可独立交付和使用
- **P2 (用户体验)**: 列出 worktree 时显示目录名和分支名 - 依赖 P1，但可独立测试
- **P3 (辅助功能)**: 显示 worktree 详情 - 依赖 P1，但可独立测试

任务拆分将按用户故事组织，确保每个故事可独立实现和交付。没有跨故事的实现依赖。

### III. 质量前置 (Quality Upfront)

✅ **PASS** - 规范已通过完整性验证：
- 无 [NEEDS CLARIFICATION] 标记
- 所有需求都是可测试且无歧义的
- 成功标准可衡量且与技术无关
- 边界情况已识别（7 种边界情况）

规划阶段将：
- 在 `research.md` 中记录所有技术决策及理由
- 确保设计通过宪章检查后才能进入任务拆分
- 鼓励 TDD：测试先于实现编写并失败

### 质量门禁状态

| 门禁 | 状态 | 备注 |
|------|------|------|
| 规范质量门禁 | ✅ PASS | 所有必需部分完整，最多 0 个 NEEDS CLARIFICATION 标记 |
| 规划质量门禁 | ⏳ IN PROGRESS | 将在 Phase 0 和 Phase 1 后验证 |
| 任务质量门禁 | ⏳ PENDING | 将在 `/speckit.tasks` 阶段验证 |
| 实现质量门禁 | ⏳ PENDING | 将在 `/speckit.implement` 阶段验证 |

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── main.rs              # CLI 入口点，命令处理器
├── cli/
│   └── output.rs        # 输出格式化（table, compact, JSON）
├── core/
│   ├── git_ops.rs       # [MODIFY] Git 命令执行包装器（添加分支名转换函数）
│   ├── worktree.rs      # [MODIFY] Worktree 数据模型（添加目录名字段）
│   └── repository.rs    # 仓库管理
└── utils/
    ├── errors.rs        # 错误类型和处理
    └── path.rs          # 路径工具函数

tests/
├── integration/
│   └── worktree_tests.rs  # [ADD] Worktree 集成测试（包含斜杠转换场景）
└── unit/
    └── git_ops_tests.rs    # [ADD] Git 操作单元测试
```

**Structure Decision**: 选择 Option 1（单一项目结构），因为这是一个 CLI 工具，不需要前后端分离。主要修改将在 `src/core/git_ops.rs` 中添加分支名转换函数，在 `src/core/worktree.rs` 中扩展数据模型以支持目录名和分支名的分离显示。测试文件将添加到 `tests/` 目录。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
