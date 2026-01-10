# Tasks: Git Worktree 管理工具

**Input**: Design documents from `/specs/001-git-worktree-cli/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: 未请求测试 - 规范中未明确要求 TDD 或测试覆盖，因此不生成测试任务。

**Organization**: 任务按用户故事分组，以支持每个故事的独立实现和测试。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 此任务所属的用户故事（例如 US1, US2, US3）
- 包含精确文件路径的描述

## Path Conventions

- **Single project**: `src/`, `tests/` 位于仓库根目录
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` 或 `android/src/`
- 以下路径假设单一项目 - 根据计划中的项目结构调整

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 项目初始化和基本结构

- [X] T001 创建 Cargo.toml 配置文件，定义项目元数据和依赖项（clap, git2, inquire, anyhow, serde, serde_json, comfy-table, env_logger, log, rayon）
- [X] T002 创建项目目录结构（src/{cli,core,utils}, tests/{integration,unit}）
- [X] T003 [P] 创建 .gitignore 文件（Rust 标准：target/, Cargo.lock, .env, *.rlib, *.rmeta）
- [X] T004 [P] 创建 README.md 文件（项目描述、安装说明、基本使用示例）
- [X] T005 初始化 Git 仓库并提交初始项目结构

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 所有用户故事之前必须完成的核心基础设施

**⚠️ CRITICAL**: 在任何用户故事工作开始之前必须完成此阶段

- [X] T006 创建 src/main.rs CLI 入口点，定义基本的命令结构和错误处理
- [X] T007 [P] 创建 src/utils/mod.rs 工具模块导出文件
- [X] T008 [P] 创建 src/utils/errors.rs，定义 WorktreeError 枚举和 Result 类型别名
- [X] T009 [P] 创建 src/utils/path.rs，实现路径验证工具函数（validate_worktree_path, validate_worktree_name）
- [X] T010 创建 src/core/mod.rs 核心模块导出文件
- [X] T011 创建 src/core/repository.rs，实现 Repository 结构体和 Git 仓库检测逻辑
- [X] T012 创建 src/core/git_ops.rs，封装 git2 库的基础操作（打开仓库、查找分支、获取提交）

**Checkpoint**: 基础完成 - 用户故事实现现在可以并行开始

---

## Phase 3: User Story 1 - 列出和切换 Worktree (Priority: P1) 🎯 MVP

**Goal**: 实现列出所有 worktree 并切换到目标 worktree 的核心功能

**Independent Test**: 执行 `work list` 查看所有 worktree 列表，执行 `work switch <name>` 切换到指定 worktree 并验证路径输出正确

### Implementation for User Story 1

- [X] T013 [P] [US1] 创建 src/cli/commands.rs，使用 clap 定义命令行参数结构（Commands 枚举：List, Switch, Create, Delete, Info, Prune）
- [X] T014 [P] [US1] 创建 src/core/worktree.rs，定义 Worktree 结构体和字段（name, branch, path, is_current, is_bare, is_detached, head_commit, upstream_branch, last_modified）
- [X] T015 [P] [US1] 创建 src/cli/output.rs，实现 OutputFormat 枚举和表格输出格式化逻辑（使用 comfy-table）
- [X] T016 [US1] 在 src/core/worktree.rs 中实现 Worktree::from_git2 方法，从 git2::Worktree 转换为我们的 Worktree 结构体
- [X] T017 [US1] 在 src/core/git_ops.rs 中实现 list_worktrees 函数，使用 git2 列出所有 worktree 并返回 Vec<Worktree>
- [X] T018 [US1] 在 src/core/worktree.rs 中实现 find_current_worktree 函数，基于当前工作目录确定当前 worktree
- [X] T019 [US1] 在 src/core/repository.rs 中实现 get_repository_info 函数，返回 Repository 元数据（root_path, is_bare, worktree_count, default_branch）
- [X] T020 [US1] 在 src/cli/output.rs 中实现 format_worktree_table 函数，将 Vec<Worktree> 格式化为人类可读的表格
- [X] T021 [US1] 在 src/cli/output.rs 中实现 format_worktree_json 函数，将 Vec<Worktree> 序列化为 JSON（使用 serde_json）
- [X] T022 [US1] 在 src/main.rs 中实现 list_command_handler 函数，处理 list 命令（调用 list_worktrees 并格式化输出）
- [X] T023 [US1] 在 src/main.rs 中实现 switch_command_handler 函数，处理 switch 命令（查找 worktree 并输出路径或使用 inquire 交互选择）
- [X] T024 [US1] 在 src/main.rs 中集成命令处理器，根据 clap 解析的命令路由到相应的 handler
- [X] T025 [US1] 在 src/main.rs 中添加 --print-path 标志支持，输出 worktree 路径供 shell 集成使用

**Checkpoint**: 此时，User Story 1 应该完全功能且可独立测试

---

## Phase 4: User Story 2 - 创建和删除 Worktree (Priority: P2)

**Goal**: 实现创建新 worktree（基于现有分支或创建新分支）和删除 worktree 的功能

**Independent Test**: 执行 `work create <branch>` 创建新 worktree 并验证目录和文件存在，执行 `work delete <name>` 删除 worktree 并验证目录和注册都被清理

### Implementation for User Story 2

- [X] T026 [P] [US2] 在 src/core/git_ops.rs 中实现 create_worktree 函数，基于现有分支创建新 worktree（使用 git 命令）
- [X] T027 [P] [US2] 在 src/core/git_ops.rs 中实现 create_worktree_with_new_branch 函数，创建新分支并同时创建 worktree
- [X] T028 [P] [US2] 在 src/core/git_ops.rs 中实现 delete_worktree 函数，删除 worktree 目录并清理 Git 注册（使用 git worktree remove）
- [X] T029 [P] [US2] 在 src/core/worktree.rs 中实现 has_uncommitted_changes 函数，检测 worktree 是否有未提交的更改
- [X] T030 [P] [US2] 在 src/utils/path.rs 中实现 validate_branch_name 函数，验证分支名符合 Git 规则
- [X] T031 [US2] 在 src/main.rs 中实现 create_command_handler 函数，处理 create 命令（支持 --branch 和 --path 参数）
- [X] T032 [US2] 在 src/main.rs 中实现 delete_command_handler 函数，处理 delete 命令（检查未提交更改，确认提示，支持 --force）
- [X] T033 [US2] 在 src/main.rs 中添加交互式创建支持，使用 dialoguer 选择基准分支（--interactive 标志）
- [X] T034 [US2] 在 src/main.rs 中添加交互式删除支持，使用 dialoguer 从列表中选择要删除的 worktree
- [X] T035 [US2] 在 src/main.rs 中集成 create 和 delete 命令处理器到主命令路由
- [X] T036 [US2] 在 src/utils/errors.rs 中添加删除相关错误变体（UncommittedChanges, CannotDeleteCurrent）
- [X] T037 [US2] 在 src/main.rs 中实现确认提示逻辑，删除前显示 worktree 信息并请求用户确认（除非 --force）

**Checkpoint**: 此时，User Stories 1 AND 2 都应该独立工作

---

## Phase 5: User Story 3 - Worktree 信息和管理 (Priority: P3)

**Goal**: 实现查看特定 worktree 详细信息、清理无效 worktree 和批量管理的功能

**Independent Test**: 执行 `work info <name>` 查看详细状态，执行 `work prune` 清理无效的 worktree 注册

### Implementation for User Story 3

- [X] T038 [P] [US3] 在 src/core/git_ops.rs 中定义 WorktreeStatusInfo 结构体（modified, staged, untracked 字段）
- [X] T039 [P] [US3] 在 src/main.rs 中实现 info_command_handler 函数，获取单个 worktree 的详细信息（包括未提交更改）
- [X] T040 [P] [US3] 在 src/core/git_ops.rs 中实现 get_worktree_status 函数，查询 worktree 的未提交更改（已修改、已暂存、未跟踪文件）
- [X] T041 [P] [US3] 在 src/core/git_ops.rs 中实现 prune_worktrees 函数，检测目录不存在但注册仍在的 worktree
- [X] T042 [P] [US3] 在 src/core/git_ops.rs 中实现 prune_worktrees 函数，清理所有无效的 worktree 注册
- [X] T043 [US3] 在 src/cli/output.rs 中实现 format_worktree_info 函数，格式化单个 worktree 的详细信息（包括提交消息、作者、未提交更改）
- [X] T044 [US3] 在 src/main.rs 的 info_command_handler 中实现格式化 ChangeSet 为可读文本
- [X] T045 [US3] 在 src/main.rs 中实现 info_command_handler 函数，处理 info 命令（显示详细信息和 JSON 格式支持）
- [X] T046 [US3] 在 src/main.rs 中实现 prune_command_handler 函数，处理 prune 命令（支持 --dry-run）
- [X] T047 [US3] 在 src/main.rs 中集成 info 和 prune 命令处理器到主命令路由
- [X] T048 [US3] 在 src/main.rs 中实现批量删除支持，允许 delete 命令接受多个 worktree 名称（Vec<String>）
- [X] T049 [US3] 在 src/main.rs 的 delete_command_handler 中实现批量操作结果显示，显示删除结果摘要

**Checkpoint**: 所有用户故事现在应该都独立功能

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 影响多个用户故事的改进

- [ ] T050 [P] 在 src/main.rs 中添加环境变量支持（WORK_OUTPUT_FORMAT, WORK_CONFIRM_DELETE, RUST_LOG）
- [ ] T051 [P] 在 src/main.rs 中添加配置文件支持（~/.workconfig.toml），使用 toml crate 解析
- [X] T052 [P] 在 src/main.rs 中实现命令别名支持（ls -> list, rm -> delete, new -> create, show -> info）
- [ ] T053 [P] 在 src/main.rs 中添加 Shell 自动补全生成（completion 命令，支持 bash/zsh/fish）
- [ ] T054 [P] 在 src/cli/output.rs 中优化并行 worktree 状态查询，使用 rayon 并行化（目标是 < 2 秒处理 20+ worktree）
- [ ] T055 [P] 在 src/core/git_ops.rs 中实现 Repository 对象缓存，避免重复打开同一仓库
- [X] T056 [P] 在 src/utils/errors.rs 和 src/main.rs 中改进错误消息，确保用户可自行解决
- [X] T057 [P] 在 src/main.rs 中添加颜色输出支持（终端友好，使用 colored crate）
- [X] T058 [P] 在 README.md 中添加完整的使用示例、故障排除和 Shell 集成说明
- [ ] T059 添加性能基准测试，验证启动时间 < 100ms 和列出 20+ worktree < 2 秒
- [X] T060 优化二进制大小，使用 cargo-strip 和 lto 减小最终可执行文件大小（已在 Cargo.toml 中配置）
- [X] T061 添加 --version 和 --help 命令，显示版本信息和详细帮助（clap 自动生成）
- [ ] T062 运行 quickstart.md 中的所有示例，验证工具行为与文档一致

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 无依赖 - 可以立即开始
- **Foundational (Phase 2)**: 依赖 Setup 完成 - 阻止所有用户故事
- **User Stories (Phase 3+)**: 所有依赖 Foundational 阶段完成
  - 用户故事可以随后并行进行（如果有人力）
  - 或按优先级顺序执行（P1 → P2 → P3）
- **Polish (Final Phase)**: 依赖所有期望的用户故事完成

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 完成后可开始 - 无其他故事依赖
- **User Story 2 (P2)**: Foundational 完成后可开始 - 可能与 US1 集成但应独立测试
- **User Story 3 (P3)**: Foundational 完成后可开始 - 可能与 US1/US2 集成但应独立测试

### Within Each User Story

- 模型/实体在服务/逻辑之前
- 逻辑/处理在 CLI 命令处理之前
- 命令处理在主集成之前
- 故事在移动到下一个优先级之前完成

### Parallel Opportunities

- Setup 中的所有任务标记为 [P] 可以并行运行
- US1 中的 T013, T014, T015 可以一起启动（命令定义、Worktree 实体、输出格式化）
- US2 中的 T026, T027, T028, T029, T030 可以一起启动（创建/删除逻辑）
- US3 中的 T038, T039, T040, T041, T042 可以一起启动（详情和清理逻辑）
- US3 中的 T043, T044 可以一起启动（输出格式化）
- Polish 中的所有任务可以并行运行
- 不同用户故事可以由不同团队成员并行工作

---

## Parallel Example: User Story 1

```bash
# 一起启动所有模型/实体任务：
Task: "创建 src/cli/commands.rs，使用 clap 定义命令行参数结构"
Task: "创建 src/core/worktree.rs，定义 Worktree 结构体和字段"
Task: "创建 src/cli/output.rs，实现 OutputFormat 枚举和表格输出格式化逻辑"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. 完成 Phase 1: Setup
2. 完成 Phase 2: Foundational（关键 - 阻止所有故事）
3. 完成 Phase 3: User Story 1
4. **停止并验证**: 独立测试 User Story 1
5. 如准备好则部署/演示

### Incremental Delivery

1. 完成 Setup + Foundational → 基础就绪
2. 添加 User Story 1 → 独立测试 → 部署/演示（MVP！）
3. 添加 User Story 2 → 独立测试 → 部署/演示
4. 添加 User Story 3 → 独立测试 → 部署/演示
5. 每个故事在不破坏前一个故事的情况下增加价值

### Parallel Team Strategy

如果有多个开发人员：

1. 团队一起完成 Setup + Foundational
2. 一旦 Foundational 完成：
   - 开发者 A: User Story 1
   - 开发者 B: User Story 2
   - 开发者 C: User Story 3
3. 故事独立完成并集成

---

## Notes

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以实现可追溯性
- 每个用户故事应该可以独立完成和测试
- 在每个任务或逻辑组之后提交
- 在任何检查点停止以独立验证故事
- 避免：模糊的任务、同一文件冲突、破坏独立性的跨故事依赖

## Task Summary

- **Total Tasks**: 62
- **Setup Phase**: 5 tasks
- **Foundational Phase**: 7 tasks
- **User Story 1 (P1)**: 13 tasks
- **User Story 2 (P2)**: 12 tasks
- **User Story 3 (P3)**: 12 tasks
- **Polish Phase**: 13 tasks

**Parallel Opportunities**: 27 tasks marked with [P] can be executed in parallel within their phases

**Suggested MVP**: Phase 1 + Phase 2 + Phase 3 (Tasks T001-T025) = 25 tasks for a functional MVP that can list and switch worktrees

**Independent Test Criteria**:
- **US1**: Can run `work list` and `work switch <name>` successfully
- **US2**: Can run `work create <branch>` and `work delete <name>` successfully
- **US3**: Can run `work info <name>` and `work prune` successfully
