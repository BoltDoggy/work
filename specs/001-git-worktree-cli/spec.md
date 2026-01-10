# Feature Specification: Git Worktree 管理工具

**Feature Branch**: `001-git-worktree-cli`
**Created**: 2026-01-10
**Status**: Draft
**Input**: User description: "使用 rust 实现一个 cli 工具,命令为 work，目的是管理 git worktree 并简化使用方式，使用方式参考 vscode 里的 git worktree 命令"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 列出和切换 Worktree (Priority: P1)

用户可以方便地查看所有可用的 worktree 并快速切换到目标 worktree 目录。

**Why this priority**: 这是核心功能，用户需要能够发现和导航现有的 worktree。没有这个功能，用户无法了解 worktree 的存在状态，也无法进行后续操作。这是工具的基础价值。

**Independent Test**: 可以通过执行列出命令查看输出，验证显示所有 worktree 信息（名称、分支、路径、状态），然后通过切换命令验证能够正确切换到目标目录。

**Acceptance Scenarios**:

1. **Given** 仓库中有多个 worktree，**When** 用户执行列出命令，**Then** 显示所有 worktree 的列表，包括分支名、路径、最后修改时间、是否为当前 worktree
2. **Given** 显示了 worktree 列表，**When** 用户选择其中一个 worktree 进行切换，**Then** 工具将用户的工作目录切换到选定的 worktree 路径
3. **Given** 用户在非 worktree 目录，**When** 用户执行列出命令，**Then** 仍然能够显示所有 worktree 并标记当前所在位置

---

### User Story 2 - 创建和删除 Worktree (Priority: P2)

用户可以创建新的 worktree（基于现有分支或创建新分支）并删除不再需要的 worktree。

**Why this priority**: 创建和删除是 worktree 管理的核心生命周期操作。优先级低于列出和切换，因为用户首先需要能够查看现有 worktree，但创建/删除是工具提供完整管理能力的关键。

**Independent Test**: 可以通过创建命令基于现有分支创建新 worktree，验证新 worktree 目录和文件正确创建；通过删除命令移除 worktree，验证目录被删除且 Git 注册信息被清理。

**Acceptance Scenarios**:

1. **Given** 用户在主仓库目录，**When** 用户执行创建命令并指定分支名，**Then** 创建新的 worktree 目录，检出指定分支，并在列表中可见
2. **Given** 用户要创建新功能分支，**When** 用户执行创建命令并指定新分支名和基准分支，**Then** 创建新的 worktree 并检出新分支
3. **Given** 存在不再需要的 worktree，**When** 用户执行删除命令并指定 worktree，**Then** 删除 worktree 目录并从 Git 的 worktree 注册中移除
4. **Given** 用户尝试删除当前正在使用的 worktree，**Then** 工具显示明确的错误信息并拒绝删除操作

---

### User Story 3 - Worktree 信息和管理 (Priority: P3)

用户可以查看特定 worktree 的详细信息，执行清理操作（移除无效的 worktree），并进行批量管理。

**Why this priority**: 这些是高级管理功能，提升用户体验但不是核心工作流所必需。用户可以在 P1 和 P2 功能下完成基本的 worktree 管理任务。

**Independent Test**: 可以通过信息命令查看单个 worktree 的详细状态（未提交的更改、分支跟踪信息等）；通过清理命令移除已不存在目录的 worktree 注册。

**Acceptance Scenarios**:

1. **Given** 用户想了解特定 worktree 的状态，**When** 用户执行信息命令并指定 worktree，**Then** 显示该 worktree 的详细信息（分支名、上游分支、未提交更改、未跟踪文件）
2. **Given** 某些 worktree 目录已被手动删除，**When** 用户执行清理命令，**Then** 工具检测并移除这些无效的 worktree 注册项
3. **Given** 用户想批量删除多个 worktree，**When** 用户执行删除命令并指定多个 worktree 或使用通配符，**Then** 工具逐一删除并显示进度和结果

---

### Edge Cases

- 当 Git 仓库没有初始化时，工具如何响应？
- 当 worktree 目录被手动删除但注册信息仍存在时，工具如何处理？
- 当用户尝试创建同名的 worktree 时，工具如何处理？
- 当 worktree 包含未提交的更改时，删除操作如何处理？
- 当磁盘空间不足无法创建新 worktree 时，工具如何提示？
- 当网络不可用时，创建基于远程分支的 worktree 如何处理？
- 当路径包含特殊字符或空格时，工具如何正确处理？

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统必须列出当前仓库的所有 worktree，显示分支名、路径、状态信息
- **FR-002**: 系统必须支持将用户工作目录切换到指定的 worktree
- **FR-003**: 系统必须支持基于现有分支创建新的 worktree
- **FR-004**: 系统必须支持创建新分支并同时创建 worktree
- **FR-005**: 系统必须支持删除指定的 worktree（删除目录和清理注册信息）
- **FR-006**: 系统必须在删除前检测 worktree 是否包含未提交的更改并提示用户
- **FR-007**: 系统必须防止删除当前正在使用的 worktree
- **FR-008**: 系统必须显示特定 worktree 的详细信息（分支、上游、未提交更改等）
- **FR-009**: 系统必须支持清理无效的 worktree（目录已不存在但注册信息存在）
- **FR-010**: 系统必须在非 Git 仓库中执行时显示明确的错误信息
- **FR-011**: 系统必须支持交互式选择（从列表中选择）和命令行参数指定 worktree
- **FR-012**: 系统必须支持人类可读的输出格式和机器可解析的输出格式（如 JSON）

### Key Entities

- **Worktree**: Git worktree 的概念表示，包含属性：分支名、路径、是否为当前 worktree、未提交更改状态、上游分支信息
- **Branch**: Git 分支引用，包含本地分支名、远程跟踪分支（如果有）、最后提交信息
- **Repository**: Git 仓库的根目录和工作目录概念
- **ChangeSet**: 未提交的更改集合，包括已修改文件、已暂存文件、未跟踪文件

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 用户可以在 5 秒内完成查看所有 worktree 和切换到目标 worktree 的操作
- **SC-002**: 用户可以在 10 秒内创建新的 worktree（基于现有分支）
- **SC-003**: 工具能够正确处理包含 20+ 个 worktree 的大型仓库，列表操作在 2 秒内完成
- **SC-004**: 95% 的用户能够在首次使用时无需查看文档成功完成基本 worktree 操作（列出、切换、创建）
- **SC-005**: 错误信息清晰明确，90% 的用户能够根据错误提示自行解决问题而无需搜索帮助
- **SC-006**: 相比直接使用 `git worktree` 命令，用户完成常见工作流（列出并切换、创建并进入）的步骤减少 50%以上
