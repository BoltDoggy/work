# Feature Specification: Worktree Branch Source Selection

**Feature Branch**: `005-worktree-branch-source`
**Created**: 2025-01-16
**Status**: Draft
**Input**: User description: "创建 worktree 时添加选项: 1. 基于当前目录所在分支 2. 基于主目录所在分支 3. 自定义输入分支"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 基于当前目录分支创建 Worktree (Priority: P1)

用户在某个 worktree 目录中工作时，想要基于当前所在的分支创建一个新的 worktree，以便快速进行并行的开发工作。

**Why this priority**: 这是最常见的使用场景，用户在当前分支上开发新功能时，经常需要基于当前分支创建修复分支或实验分支。

**Independent Test**: 可以独立测试通过执行创建命令并选择"当前分支"选项，验证新 worktree 是否正确基于当前分支创建，且不切换当前工作目录。

**Acceptance Scenarios**:

1. **Given** 用户在 `/project.worktrees/feature-a` 目录中，且当前分支是 `feature-a`，**When** 用户执行 `work create feature-b` 并选择"基于当前分支"，**Then** 创建新 worktree 在 `/project.worktrees/feature-b`，且基于 `feature-a` 分支
2. **Given** 用户在主目录 `/project` 中，且当前分支是 `main`，**When** 用户执行 `work create feature-x` 并选择"基于当前分支"，**Then** 创建新 worktree 在 `/project.worktrees/feature-x`，且基于 `main` 分支
3. **Given** 用户当前在 `feature-123` 分支上，**When** 用户选择"基于当前分支"创建 `feature-123-fix`，**Then** 新 worktree 正确创建且初始状态与 `feature-123` 完全一致

---

### User Story 2 - 基于主目录分支创建 Worktree (Priority: P2)

用户在任何 worktree 中工作时，想要基于主仓库（main repository）的分支创建新的 worktree，而不需要先切换到主目录。

**Why this priority**: 提供便捷性，避免用户需要先切换到主目录才能基于主分支创建 worktree，提升了多分支开发场景下的工作效率。

**Independent Test**: 可以独立测试通过在任意 worktree 目录中执行创建命令并选择"主目录分支"选项，验证新 worktree 是否基于主目录的当前分支创建。

**Acceptance Scenarios**:

1. **Given** 用户在 `/project.worktrees/feature-a` 目录中（当前分支 `feature-a`），主目录当前分支是 `develop`，**When** 用户执行 `work create feature-b` 并选择"基于主目录分支"，**Then** 创建新 worktree 基于 `develop` 分支而非 `feature-a`
2. **Given** 用户在 `/project.worktrees/bugfix-1` 中，主目录在 `main` 分支，**When** 用户选择"基于主目录分支"创建 `hotfix-2`，**Then** 新 worktree 基于 `main` 分支创建
3. **Given** 主目录和当前 worktree 在不同分支，**When** 用户选择"基于主目录分支"，**Then** 新 worktree 正确基于主目录分支且不影响当前 worktree 状态

---

### User Story 3 - 基于自定义分支创建 Worktree (Priority: P3)

用户想要基于任意指定的分支名称创建新的 worktree，包括远程分支或本地其他分支，提供最大的灵活性。

**Why this priority**: 提供最高级别的灵活性，适用于高级用户需要基于特定分支（如远程分支、历史分支、其他功能分支）创建 worktree 的场景。

**Independent Test**: 可以独立测试通过执行创建命令并输入自定义分支名称，验证新 worktree 是否基于指定的分支创建，包括远程分支和本地分支。

**Acceptance Scenarios**:

1. **Given** 用户想要基于远程分支 `origin/feature-remote` 创建本地 worktree，**When** 用户执行 `work create feature-local` 并输入分支名 `origin/feature-remote`，**Then** 创建新 worktree 并跟踪远程分支
2. **Given** 用户想要基于本地存在的 `feature-old` 分支创建新 worktree，**When** 用户输入自定义分支名 `feature-old`，**Then** 新 worktree 基于 `feature-old` 创建
3. **Given** 用户输入的分支名称不存在，**When** 用户执行创建命令，**Then** 系统提示错误信息并说明分支不存在，不创建 worktree
4. **Given** 用户输入的分支名称包含空格或特殊字符，**When** 系统处理分支名，**Then** 正确解析分支名称或提供明确的错误提示

---

### Edge Cases

- 当用户在主目录（非 worktree）中选择"基于主目录分支"时，应该与"基于当前分支"行为相同
- 当主目录是 detached HEAD 状态时，选择"基于主目录分支"应该如何处理
- 当用户输入的自定义分支名称与现有分支完全相同时（包括本地和远程）
- 当工作目录有未提交的更改时，创建新 worktree 不应受到影响
- 当指定的分支名称包含特殊字符（如 `~`, `^`, `:` 等 Git 保留字符）时的处理
- 当用户输入空分支名称或仅包含空格时的错误处理
- 当远程分支名称包含 `origin/` 前缀时，是否需要自动处理跟踪关系
- 当基于当前目录分支创建时，但当前目录本身就是主目录（非 worktree）的行为

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统必须在执行 `work create <name>` 命令时提供分支来源选择机制
- **FR-002**: 系统必须支持"基于当前目录所在分支"选项，能够检测并使用当前工作目录的 Git 分支
- **FR-003**: 系统必须支持"基于主目录所在分支"选项，能够定位并使用主仓库（通过 `git rev-parse --git-common-dir`）的当前分支
- **FR-004**: 系统必须支持"自定义输入分支"选项，允许用户输入任意有效的分支名称（包括本地和远程分支）
- **FR-005**: 系统必须在用户选择分支来源后，基于指定的分支创建新的 worktree
- **FR-006**: 系统必须验证用户输入的自定义分支名称是否存在，不存在时提供清晰的错误信息
- **FR-007**: 系统必须在创建 worktree 时保持新 worktree 的初始状态与所选分支完全一致
- **FR-008**: 系统必须确保创建新 worktree 不会影响当前工作目录的状态或未提交的更改
- **FR-009**: 系统必须处理主目录为 detached HEAD 状态的情况，提供适当的错误提示或行为选项
- **FR-010**: 系统必须正确解析包含 `origin/` 前缀的远程分支名称，并建立正确的跟踪关系
- **FR-011**: 系统必须为分支来源选择提供友好的交互界面（如选项菜单、命令行参数等）
- **FR-012**: 系统必须在基于远程分支（如 `origin/feature`）创建 worktree 时，自动创建对应的本地分支并设置跟踪关系

### Key Entities

- **Worktree**: Git 工作树，包含独立的工作目录和分支状态
- **分支来源 (Branch Source)**: 创建 worktree 时所基于的分支引用，可以是当前分支、主目录分支或自定义分支
- **主目录 (Main Repository)**: 通过 `git rev-parse --git-common-dir` 定位的主仓库目录
- **当前分支 (Current Branch)**: 当前工作目录（可能是主目录或某个 worktree）中 Git 检出的分支

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 用户可以在 30 秒内完成基于当前分支创建新 worktree 的操作（从执行命令到创建完成）
- **SC-002**: 用户可以在 45 秒内完成基于主目录分支创建新 worktree 的操作，无需切换目录
- **SC-003**: 用户可以在 60 秒内完成基于自定义分支创建新 worktree 的操作，包括输入分支名称
- **SC-004**: 95% 的用户能够在首次使用时成功理解并选择正确的分支来源选项
- **SC-005**: 系统对不存在的分支名称提供错误反馈的时间不超过 2 秒
- **SC-006**: 用户报告的因分支来源选择错误导致的问题减少 80%（相比手动切换目录创建 worktree 的方式）
- **SC-007**: 所有三种分支来源选项（当前分支、主目录分支、自定义分支）的功能测试通过率达到 100%

## Assumptions

1. 用户熟悉基本的 Git 概念，包括分支、worktree、主目录等术语
2. 用户的系统已正确配置 Git 并且可以正常使用 `git worktree` 命令
3. 默认情况下，系统应该提供交互式选项菜单让用户选择分支来源
4. 用户可能希望通过命令行参数（如 `--branch` 或 `-b`）直接指定分支来源，但这不是 P1 优先级
5. 当基于远程分支（如 `origin/feature`）创建 worktree 时，系统应该自动创建对应的本地分支并设置跟踪关系
6. 如果用户不选择分支来源（例如通过非交互模式），系统应该有合理的默认行为（建议默认为"基于当前分支"）
7. 错误信息应该清晰指导用户如何解决问题，例如显示可用的分支列表

## Out of Scope *(optional)*

- 批量创建多个 worktree 的功能
- 创建 worktree 时同时切换到新分支的功能
- 创建 worktree 时自动执行初始化脚本或命令的功能
- 在创建 worktree 后自动打开新目录的功能
- 与其他 Git 工具（如 GUI 客户端）的集成
- Worktree 模板功能（基于预定义的目录结构或配置创建）
