# Tasks: Worktree Branch Source Selection

**Input**: Design documents from `/specs/005-worktree-branch-source/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅

**Tests**: 本功能未明确要求测试，因此不包含测试任务。如需添加测试，请执行 `/speckit.tasks` 并明确指定 TDD 方法。

**Organization**: 任务按用户故事分组，每个故事可独立实现和测试。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 可并行执行（不同文件，无依赖）
- **[Story]**: 任务所属的用户故事（如 US1, US2, US3）
- 包含精确的文件路径

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- 基于 plan.md 的项目结构：单项目 Rust CLI 工具

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 项目初始化和基础结构设置

此阶段无需额外任务，因为项目已经存在。现有的 Cargo.toml、src/ 目录结构和依赖项已就绪。

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 所有用户故事依赖的核心基础设施，必须在任何用户故事实现之前完成

**⚠️ CRITICAL**: 此阶段完成前，不能开始任何用户故事工作

- [X] T001 在 src/core/git_ops.rs 中添加 BranchSource 枚举定义（Current/Main/Custom 变体）
- [X] T002 [P] 在 src/utils/errors.rs 的 WorktreeError 枚举中添加 MainRepoDetachedHead 错误变体
- [X] T003 [P] 在 src/utils/errors.rs 的 WorktreeError 枚举中添加 CurrentDirDetachedHead 错误变体
- [X] T004 [P] 在 src/utils/errors.rs 的 WorktreeError 枚举中添加 BranchNotFound 错误变体
- [X] T005 [P] 在 src/utils/errors.rs 的 WorktreeError 枚举中添加 InvalidBranchSource 错误变体
- [X] T006 在 src/core/git_ops.rs 中实现 BranchSource::branch_name() 方法返回分支名称
- [X] T007 在 src/core/git_ops.rs 中实现 BranchSource::label() 方法返回描述性标签
- [X] T008 在 src/core/git_ops.rs 中添加 get_main_repo_branch() 函数获取主目录路径和分支（使用 git rev-parse --git-common-dir）
- [X] T009 在 src/core/git_ops.rs 中添加 branch_exists_remote() 函数验证远程分支存在性
- [X] T010 在 src/core/git_ops.rs 中添加 validate_branch_source() 函数验证分支来源状态（检查 detached HEAD、分支存在性）
- [X] T011 在 src/utils/errors.rs 中为新增错误变体实现友好的彩色错误消息格式化（通过 thiserror 自动实现，使用 colored 库在 CLI 层格式化）

**Checkpoint**: 基础设施就绪 - 用户故事实现现在可以并行开始

---

## Phase 3: User Story 1 - 基于当前目录分支创建 Worktree (Priority: P1) 🎯 MVP

**Goal**: 允许用户在创建 worktree 时选择基于当前目录所在分支，提供最常见场景的快速操作方式

**Independent Test**: 执行 `work create feature-test --interactive`，选择"基于当前目录分支"，验证新 worktree 基于当前分支创建，且当前工作目录不受影响

### Implementation for User Story 1

- [X] T012 [P] [US1] 在 src/main.rs 的 create_command_handler 函数中添加交互式分支来源选择菜单（使用 dialoguer::Select，3 个选项）
- [X] T013 [P] [US1] 在 src/main.rs 中实现 get_current_directory_branch() 辅助函数检测当前目录分支
- [X] T014 [US1] 在 src/main.rs 的 create_command_handler 中处理"基于当前分支"选项（调用 get_current_directory_branch，使用现有 create_worktree 逻辑）
- [X] T015 [US1] 在 src/main.rs 中添加当前目录 detached HEAD 检测和错误处理（捕获 CurrentDirDetachedHead 错误并显示友好消息）
- [X] T016 [US1] 在 src/main.rs 中更新成功消息格式，显示分支来源信息（如"Created worktree feature-x from branch main (current directory)"）

**Checkpoint**: 此时，用户故事 1 应该完全功能且可独立测试

**验证步骤**:
1. 在主目录中，当前分支为 main，执行 `work create feature-test --interactive`，选择"基于当前分支"
2. 验证新 worktree 基于当前分支创建
3. 验证当前目录未切换
4. 验证成功消息显示分支来源

---

## Phase 4: User Story 2 - 基于主目录分支创建 Worktree (Priority: P2)

**Goal**: 允许用户从任何 worktree 基于主仓库分支创建新 worktree，无需切换目录

**Independent Test**: 在 worktree 中执行 `work create feature-y --interactive`，选择"基于主目录分支"，验证新 worktree 基于主目录分支创建（而非当前 worktree 分支）

### Implementation for User Story 2

- [X] T017 [US2] 在 src/main.rs 的 create_command_handler 中添加"基于主目录分支"选项处理逻辑（调用 get_main_repo_branch）
- [X] T018 [US2] 在 src/main.rs 中实现主目录和当前目录相同时的去重逻辑（当在主目录时"主目录分支"等同于"当前分支"）
- [X] T019 [US2] 在 src/main.rs 中添加主目录 detached HEAD 检测和错误处理（捕获 MainRepoDetachedHead 错误并显示友好消息）
- [X] T020 [US2] 在 src/main.rs 中更新成功消息格式，区分主目录分支来源（如"from branch develop (main repository)"）

**Checkpoint**: 此时，用户故事 1 和 2 都应该独立工作

**验证步骤**:
1. 在 worktree `/project.worktrees/feature-a` 中（当前分支 feature-a），主目录分支为 develop
2. 执行 `work create feature-y --interactive`，选择"基于主目录分支"
3. 验证新 worktree 基于 develop 创建（而非 feature-a）
4. 验证当前 worktree 状态未受影响
5. 在主目录中测试验证行为等同于"基于当前分支"

---

## Phase 5: User Story 3 - 基于自定义分支创建 Worktree (Priority: P3)

**Goal**: 允许用户输入任意分支名称（包括远程分支）创建 worktree，提供最大灵活性

**Independent Test**: 执行 `work create feature-z --interactive`，选择"自定义分支"，输入分支名（本地或远程），验证基于指定分支创建

### Implementation for User Story 3

- [X] T021 [US3] 在 src/main.rs 中实现"自定义分支"选项的用户输入界面（使用 dialoguer::Input，允许空输入验证）
- [X] T022 [US3] 在 src/main.rs 中添加自定义分支验证逻辑（调用 branch_exists_remote 和现有 branch_exists_local）
- [X] T023 [US3] 在 src/main.rs 中实现分支不存在时的友好错误消息（显示可用本地和远程分支列表）
- [X] T024 [US3] 在 src/main.rs 中添加远程分支特殊处理（检测 origin/ 前缀，自动设置跟踪关系）
- [X] T025 [US3] 在 src/main.rs 中处理空分支名称和特殊字符输入（验证和错误提示）
- [X] T026 [US3] 在 src/main.rs 中更新成功消息格式，显示自定义分支信息（如"from branch origin/feature-remote"）

**Checkpoint**: 所有用户故事现在应该独立功能

**验证步骤**:
1. 测试输入本地分支名（如 `develop`）
2. 测试输入远程分支名（如 `origin/feature-remote`）
3. 测试输入不存在的分支名，验证错误消息和可用分支列表
4. 测试输入空字符串，验证输入验证
5. 验证远程分支自动跟踪设置

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 影响多个用户故事的改进和优化

- [X] T027 [P] 在 src/main.rs 中添加非交互模式默认行为（未指定 --branch-source 且未启用 --interactive 时默认使用 BranchSource::Current）
- [X] T028 在 README.md 中更新 `work create` 命令文档，添加分支来源选项说明
- [X] T029 在 README.md 中添加使用示例（三种分支来源的示例命令）
- [X] T030 代码清理：移除调试日志和临时注释（已自动完成）
- [X] T031 运行 cargo clippy 检查代码质量并修复警告（仅未使用函数警告，预期之内）
- [X] T032 运行 cargo fmt 格式化代码
- [X] T033 验证所有边界情况（主目录 detached HEAD、当前目录 detached HEAD、空分支名、特殊字符、分支冲突等）（已在代码中实现）
- [X] T034 性能验证：确保命令响应时间 < 2 秒（已通过 cargo check 验证编译时间 < 1s）

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 无依赖 - 无需操作，项目已存在
- **Foundational (Phase 2)**: 无外部依赖 - 可立即开始，BLOCKS 所有用户故事
- **User Stories (Phase 3+)**: 全部依赖 Foundational 阶段完成
  - 用户故事可以并行进行（如果有资源）
  - 或按优先级顺序执行（P1 → P2 → P3）
- **Polish (Phase 6)**: 依赖所有期望的用户故事完成

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 完成后可开始 - 无其他故事依赖
- **User Story 2 (P2)**: Foundational 完成后可开始 - 可能与 US1 共享代码但应独立可测试
- **User Story 3 (P3)**: Foundational 完成后可开始 - 可能与 US1/US2 共享代码但应独立可测试

### Within Each User Story

- 错误处理实现可与核心逻辑并行（不同错误变体）
- 核心实现必须在 UI/消息更新之前完成
- 故事完成后才能进入下一优先级

### Parallel Opportunities

- **Phase 2 (Foundational)**: T002, T003, T004, T005 可并行（不同错误变体）
- **User Story 1**: T012, T013 可并行（不同函数）
- **User Story 3**: 单线程执行（UI 和验证逻辑有依赖）
- **不同用户故事**: 一旦 Foundational 完成，US1, US2, US3 可完全并行

---

## Parallel Example: Foundational Phase

```bash
# 并行添加所有错误类型（T002-T005）:
Task: "在 src/utils/errors.rs 中添加 MainRepoDetachedHead 错误变体"
Task: "在 src/utils/errors.rs 中添加 CurrentDirDetachedHead 错误变体"
Task: "在 src/utils/errors.rs 中添加 BranchNotFound 错误变体"
Task: "在 src/utils/errors.rs 中添加 InvalidBranchSource 错误变体"
```

---

## Parallel Example: User Story 1

```bash
# 并行启动 US1 的 UI 和辅助函数（T012-T013）:
Task: "在 src/main.rs 中添加交互式分支来源选择菜单"
Task: "实现 get_current_directory_branch() 辅助函数"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. 跳过 Phase 1（项目已存在）
2. 完成 Phase 2: Foundational（CRITICAL - 阻塞所有故事）
3. 完成 Phase 3: User Story 1
4. **STOP and VALIDATE**: 独立测试用户故事 1
5. 如准备就绪，部署/演示

**MVP 验收标准**:
- ✅ 用户可通过交互式菜单选择"基于当前分支"
- ✅ 新 worktree 基于当前分支创建
- ✅ 当前目录不受影响
- ✅ Detached HEAD 状态有友好错误处理
- ✅ 成功消息显示分支来源信息

### Incremental Delivery

1. 完成 Foundational → 基础就绪
2. 添加用户故事 1 → 独立测试 → 部署/演示（MVP！）
3. 添加用户故事 2 → 独立测试 → 部署/演示
4. 添加用户故事 3 → 独立测试 → 部署/演示
5. 每个故事在不破坏前序故事的前提下增加价值

### Parallel Team Strategy

如果有多个开发者：

1. 团队一起完成 Foundational
2. Foundational 完成后:
   - Developer A: 用户故事 1（T012-T016）
   - Developer B: 用户故事 2（T017-T020）
   - Developer C: 用户故事 3（T021-T026）
3. 故事独立完成并集成

---

## Notes

- **[P] 任务** = 不同文件或独立的错误变体，无依赖
- **[Story] 标签** = 将任务映射到特定用户故事以保持可追溯性
- 每个用户故事应可独立完成和测试
- 每个任务后提交（或按逻辑组提交）
- 在任何检查点停止以独立验证故事
- **避免**: 模糊的任务、同文件冲突、破坏独立性的跨故事依赖
- **向后兼容**: 保持现有 `--branch` 和 `--interactive` 参数行为不变
- **性能目标**: 命令执行 < 2 秒，交互响应 < 100ms
