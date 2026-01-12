# Feature Specification: 分支名斜杠转换

**Feature Branch**: `003-branch-slash-conversion`
**Created**: 2026-01-12
**Status**: Draft
**Input**: User description: "创建 worktree 时的分支名中存在 / 时,创建的目录需要转换为 -,但分支名保持不变,如 feat/xxx 的 worktree 目录需要为 feat-xxx"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 创建包含斜杠的分支的 worktree (Priority: P1)

作为开发者，我想创建一个包含斜杠的分支（如 `feat/xxx`、`feature/auth`）的 worktree，系统应该将斜杠转换为连字符创建目录，但保持分支名不变。

**Why this priority**: 这是核心功能，解决了分支命名规范（使用斜杠组织分支）与文件系统目录命名限制之间的冲突。这是最常见和最重要的使用场景。

**Independent Test**: 可以通过创建一个包含斜杠的分支（如 `feat/new-feature`）的 worktree，验证目录名使用连字符（`feat-new-feature`）而分支名保持原样（`feat/new-feature`）来独立测试。

**Acceptance Scenarios**:

1. **Given** 分支名为 `feat/feature-001`，**When** 执行 `work add feat/feature-001`，**Then** 创建的 worktree 目录名为 `feat-feature-001`，且分支名保持为 `feat/feature-001`
2. **Given** 分支名为 `feature/user-auth/oauth`（多个斜杠），**When** 执行 `work add feature/user-auth/oauth`，**Then** 创建的 worktree 目录名为 `feature-user-auth-oauth`，且分支名保持为 `feature/user-auth/oauth`
3. **Given** 分支名不包含斜杠（如 `main`），**When** 执行 `work add main`，**Then** 创建的 worktree 目录名和分支名均为 `main`（无转换）

---

### User Story 2 - 列出包含斜杠分支的 worktree (Priority: P2)

作为开发者，我想在列出 worktree 时能够清晰地看到目录名和实际分支名的对应关系，特别是当分支名包含斜杠时。

**Why this priority**: 虽然不影响核心功能，但能提供更好的用户体验，帮助开发者快速识别 worktree 对应的实际分支。

**Independent Test**: 可以通过创建包含斜杠分支的 worktree，然后执行 `work list` 命令，验证输出中目录名使用连字符而分支名显示原始斜杠来独立测试。

**Acceptance Scenarios**:

1. **Given** 存在一个 worktree 目录名为 `feat-feature-001` 对应分支 `feat/feature-001`，**When** 执行 `work list`，**Then** 输出显示目录名 `feat-feature-001` 和分支名 `feat/feature-001`，清晰展示两者的对应关系
2. **Given** 存在多个包含斜杠的分支 worktree，**When** 执行 `work list`，**Then** 所有 worktree 的输出都正确显示目录名和分支名

---

### User Story 3 - 显示包含斜杠分支的 worktree 详情 (Priority: P3)

作为开发者，我想查看 worktree 的详细信息时，能够看到目录名和分支名的完整信息，特别是当分支名包含斜杠时。

**Why this priority**: 这是辅助功能，提供更详细的信息用于调试和管理，但不影响日常使用。

**Independent Test**: 可以通过创建包含斜杠分支的 worktree，然后执行 `work show <directory>` 命令，验证详情输出中同时显示转换后的目录名和原始分支名来独立测试。

**Acceptance Scenarios**:

1. **Given** 存在一个 worktree 目录名为 `feat-feature-001` 对应分支 `feat/feature-001`，**When** 执行 `work show feat-feature-001`，**Then** 详细输出中包含 "目录名: feat-feature-001" 和 "分支名: feat/feature-001"

---

### Edge Cases

- 分支名只包含斜杠（如 `/` 或 `//`）时的处理
- 分支名以斜杠开头或结尾（如 `/feat` 或 `feat/`）时的处理
- 分支名包含连续多个斜杠（如 `feat///feature`）时的处理
- 分支名转换后与现有 worktree 目录名冲突时的处理
- 分支名包含其他文件系统不允许的字符（如 `:`、`*`、`?` 等）时的处理
- 分支名转换为目录名后超过文件系统最大长度限制时的处理
- 不同操作系统（Windows、Linux、macOS）对目录名的大小写敏感性差异

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 当创建 worktree 时，系统必须自动检测分支名中是否包含斜杠（`/`）字符
- **FR-002**: 系统必须将分支名中的所有斜杠（`/`）转换为连字符（`-`）作为 worktree 目录名
- **FR-003**: 系统必须保持原始分支名不变，不修改 Git 分支的实际名称
- **FR-004**: 对于不包含斜杠的分支名，系统必须使用原始分支名作为目录名，不进行任何转换
- **FR-005**: 系统必须在列出 worktree 时清晰显示目录名和对应的实际分支名
- **FR-006**: 当分支名转换为目录名后，系统必须确保目录名符合文件系统命名规范（不包含非法字符）
- **FR-007**: 系统必须在显示 worktree 详情时同时提供目录名和分支名信息
- **FR-008**: 当转换后的目录名与现有 worktree 冲突时，系统必须向用户报告明确的错误信息
- **FR-009**: 系统必须支持分支名中包含多个连续斜杠的场景（将所有 `/` 转换为 `-`）
- **FR-010**: 系统必须处理分支名以斜杠开头或结尾的场景（转换后仍保留连字符）

### Key Entities

- **Branch Name**: Git 分支的完整名称，可能包含斜杠（如 `feat/feature-001`、`feature/auth/oauth`）
- **Worktree Directory**: 文件系统中的实际目录路径，由分支名转换而来（斜杠替换为连字符）
- **Worktree Metadata**: 包含目录名和对应分支名的映射关系

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 开发者可以在 5 秒内成功创建包含斜杠分支的 worktree，无需手动处理目录命名问题
- **SC-002**: 100% 的包含斜杠的分支名都能正确转换为合法的目录名，且分支名保持不变
- **SC-003**: worktree 列表输出中 100% 清晰显示目录名和分支名的对应关系
- **SC-004**: 目录名转换导致的冲突检测率达到 100%，所有命名冲突都能在创建前被识别并报告
- **SC-005**: 开发者反馈表明此功能解决了 90% 以上因分支命名规范导致的 worktree 创建问题
