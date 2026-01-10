# Tasks: 修复 GitHub Actions Release Workflow

**Input**: Design documents from `/specs/002-fix-release/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md

**Tests**: 未请求测试 - 这是 CI/CD 配置修复，验证通过实际 tag 推送完成，而非单元测试。

**Organization**: 任务按用户故事分组，支持每个故事的独立实现和测试。

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 此任务所属的用户故事（例如 US1, US2, US3）
- 包含精确文件路径的描述

## Path Conventions

- **Single project**: `.github/workflows/` 为唯一修改目录
- **验证文件**: `specs/002-fix-release/` 用于文档和测试计划

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 准备工作环境和验证基础

- [ ] T001 切换到 main 分支并确保本地代码最新（git checkout main && git pull origin main）
- [ ] T002 验证当前 release.yml 存在并读取内容（cat .github/workflows/release.yml）
- [ ] T003 [P] 备份当前 release.yml 文件（cp .github/workflows/release.yml .github/workflows/release.yml.backup）

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 核心基础设施必须完成才能开始任何用户故事

**⚠️ CRITICAL**: 在任何用户故事工作开始之前必须完成此阶段

- [ ] T004 理解 artifacts 上传/下载问题根因（review research.md 第 7-71 行）
- [ ] T005 理解技术决策和实现方案（review research.md 第 73-323 行）
- [ ] T006 [P] 理解 YAML 语法和 GitHub Actions workflow 结构（review .github/workflows/release.yml）
- [ ] T007 [P] 准备测试 tag 名称（v0.9.0-test）和验证计划

**Checkpoint**: 基础准备完成 - 用户故事实现现在可以并行开始

---

## Phase 3: User Story 1 - 发布构建产物到 GitHub Release (Priority: P1) 🎯 MVP

**Goal**: 修复 artifacts 上传/下载问题，确保所有 5 个平台的二进制文件正确上传到 GitHub Release Downloads

**Independent Test**: 推送测试 tag v0.9.0-test，验证 GitHub Actions 完成后，在 GitHub Releases 页面能看到所有 5 个平台的二进制文件（.tar.gz 和 .zip）可供下载

### Implementation for User Story 1

- [ ] T008 [US1] 修改 upload-artifact 步骤（第 91-97 行），将 artifact 名称从 `${{ matrix.asset_name }}` 改为统一名称 `release-artifacts` 在 .github/workflows/release.yml
- [ ] T009 [US1] 修改 upload-artifact 步骤，添加 `if-no-files-found: error` 配置在 .github/workflows/release.yml
- [ ] T010 [US1] 修改 download-artifact 步骤（第 107-110 行），添加 `pattern: release-artifacts` 配置在 .github/workflows/release.yml
- [ ] T011 [US1] 修改 download-artifact 步骤，添加 `merge-multiple: true` 配置展平目录结构在 .github/workflows/release.yml
- [ ] T012 [US1] 添加验证文件结构步骤，在 download-artifact 后插入调试命令 `ls -R artifacts` 和 `find artifacts -name "*.tar.gz"` 在 .github/workflows/release.yml 第 110 行之后
- [ ] T013 [US1] 修改 softprops/action-gh-release 的 files 参数（第 120-121 行），将 `artifacts/*.*` 改为递归通配符 `artifacts/**/*.tar.gz` 和 `artifacts/**/*.zip` 在 .github/workflows/release.yml
- [ ] T014 [US1] 修改 softprops/action-gh-release 的 files 参数，添加 `artifacts/checksums.txt` 文件路径在 .github/workflows/release.yml
- [ ] T015 [US1] 添加失败处理配置，在 softprops/action-gh-release 步骤添加 `fail_on_unmatched_files: true` 在 .github/workflows/release.yml
- [ ] T016 [US1] 验证 YAML 语法正确性（使用 GitHub Actions CLI 或在线验证工具）
- [ ] T017 [US1] 提交修复到分支（git add .github/workflows/release.yml && git commit -m "fix: 修复 artifacts 上传/下载路径问题"）
- [ ] T018 [US1] 推送到远程分支（git push origin 002-fix-release）
- [ ] T019 [US1] 创建并推送测试 tag（git tag v0.9.0-test && git push origin v0.9.0-test）
- [ ] T020 [US1] 监控 GitHub Actions 运行，验证所有步骤成功（检查 Actions 页面和日志）
- [ ] T021 [US1] 验证 Release 创建成功并包含所有 5 个平台文件（访问 Releases 页面确认）
- [ ] T022 [US1] 下载并验证文件完整性（下载 tar.gz 文件并解压，运行 sha256sum -c checksums.txt）
- [ ] T023 [US1] 清理测试 tag（git tag -d v0.9.0-test && git push origin :refs/tags/v0.9.0-test && gh release delete v0.9.0-test --yes）

**Checkpoint**: 此时，User Story 1 应该完全功能且可独立测试 - 所有 5 个平台的二进制文件应正确上传到 Release Downloads

---

## Phase 4: User Story 2 - 验证构建产物的完整性 (Priority: P2)

**Goal**: 确保 checksums.txt 包含所有文件的 SHA256 哈希，用户能验证文件完整性

**Independent Test**: 下载任意平台的 tar.gz 文件和 checksums.txt，运行 `sha256sum -c checksums.txt`，验证所有文件的哈希值完全匹配

### Implementation for User Story 2

- [ ] T024 [US2] 验证 checksums 生成步骤正确（review Create checksums 步骤第 112-115 行在 .github/workflows/release.yml）
- [ ] T025 [US2] 确认 checksums.txt 在所有二进制文件之后生成（验证步骤顺序：Download all artifacts → Create checksums → Create Release）
- [ ] T026 [US2] 测试 checksums 验证流程（下载文件并运行 sha256sum -c checksums.txt）
- [ ] T027 [US2] 验证所有文件哈希匹配（确认输出显示所有文件 OK）

**Checkpoint**: 此时，User Stories 1 AND 2 都应该独立工作 - Release 包含二进制文件和可用的 checksums.txt

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: 影响多个用户故事的改进

- [ ] T028 [P] 更新 quickstart.md 文档，记录验证结果和任何发现的边缘情况在 specs/002-fix-release/quickstart.md
- [ ] T029 [P] 创建故障排除指南，记录常见问题和解决方案（参考 quickstart.md 故障排除部分）
- [ ] T030 [P] 更新 README.md 或相关文档，确认发布流程说明正确
- [ ] T031 [P] 清理备份文件（rm .github/workflows/release.yml.backup）
- [ ] T032 创建 Pull Request，包含修复说明和测试验证结果
- [ ] T033 合并 PR 到 main 分支
- [ ] T034 删除特性分支（git branch -d 002-fix-release）

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 无依赖 - 可以立即开始
- **Foundational (Phase 2)**: 依赖 Setup 完成 - 阻止所有用户故事
- **User Stories (Phase 3+)**: 所有依赖 Foundational 阶段完成
  - 用户故事按顺序执行（US1 → US2）
  - US2 可以独立测试但不依赖 US1
- **Polish (Final Phase)**: 依赖用户故事完成

### User Story Dependencies

- **User Story 1 (P1)**: Foundational 完成后可开始 - 无其他故事依赖
- **User Story 2 (P2)**: Foundational 完成后可开始 - 在 US1 完成后验证

### Within Each User Story

- YAML 修改按顺序完成（upload → download → release）
- 验证步骤在所有修改完成后执行
- 推送 tag 是最后验证步骤

### Parallel Opportunities

- Setup 中的所有任务标记为 [P] 可以并行运行
- Foundational 中的任务 T006 和 T007 可以并行运行
- Polish 中的所有任务可以并行运行

## Parallel Example: User Story 1

```bash
# 一起启动所有验证任务：
Task: "验证 YAML 语法正确性"
Task: "验证文件结构"
Task: "准备测试 tag 名称"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. 完成 Phase 1: Setup
2. 完成 Phase 2: Foundational（关键 - 阻止所有故事）
3. 完成 Phase 3: User Story 1
4. **停止并验证**: 独立测试 User Story 1（推送测试 tag）
5. 如果准备好则部署/演示

### Incremental Delivery

1. 完成 Setup + Foundational → 基础就绪
2. 添加 User Story 1 → 独立测试 → 部署/演示（MVP！）
3. 添加 User Story 2 → 独立测试 → 部署/演示
4. 每个故事在不破坏前一个故事的情况下增加价值

### Sequential Strategy

此功能是单一配置文件修复，建议顺序执行：

1. Team 一起完成 Setup + Foundational
2. 按顺序完成用户故事（US1 → US2）
3. 最后完成 Polish

---

## Notes

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以实现可追溯性
- 每个用户故事应该可以独立完成和测试
- 在每个任务或逻辑组之后提交
- 在任何检查点停止以独立验证故事
- 避免：模糊的任务、同一文件冲突、跨故事依赖

## Task Summary

- **Total Tasks**: 34
- **Setup Phase**: 3 tasks
- **Foundational Phase**: 4 tasks
- **User Story 1 (P1)**: 16 tasks
- **User Story 2 (P2)**: 4 tasks
- **Polish Phase**: 7 tasks

**Parallel Opportunities**: 7 tasks marked with [P] can be executed in parallel within their phases

**Suggested MVP**: Phase 1 + Phase 2 + Phase 3 (Tasks T001-T023) = 23 tasks for a functional MVP that publishes release artifacts correctly

**Independent Test Criteria**:
- **US1**: 推送 v0.9.0-test tag 后，GitHub Release 包含所有 5 个平台的二进制文件
- **US2**: 下载文件并运行 sha256sum -c checksums.txt 验证完整性
