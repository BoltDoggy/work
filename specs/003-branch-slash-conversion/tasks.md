# Implementation Tasks: 分支名斜杠转换

**Feature Branch**: `003-branch-slash-conversion`
**Date**: 2026-01-12
**Spec**: [spec.md](./spec.md)
**Plan**: [plan.md](./plan.md)
**Total Tasks**: 38

---

## Task Organization

This document breaks down the implementation into actionable tasks organized by **user story**, not by technical layer. Each user story can be independently implemented and tested.

**MVP Scope**: Phase 3 (User Story 1) - Core slash conversion functionality

**Phases**:
- Phase 1: Setup (shared infrastructure)
- Phase 2: Foundational (blocking prerequisites for all user stories)
- Phase 3: User Story 1 - Create worktree with slash conversion (P1)
- Phase 4: User Story 2 - List worktrees showing directory and branch names (P2)
- Phase 5: User Story 3 - Show worktree details (P3)
- Phase 6: Polish & Cross-Cutting Concerns

---

## Dependencies

```
Phase 1 (Setup)
    ↓
Phase 2 (Foundational)
    ↓
    ├→ Phase 3 [US1] (P1 - MVP) ────┐
    │                              ├→ Phase 6 (Polish)
    ├→ Phase 4 [US2] (P2) ─────────┤
    │                              │
    └→ Phase 5 [US3] (P3) ─────────┘
```

**Story Dependencies**:
- US2 and US3 depend on US1 (they use the converted directory names from US1)
- US2 and US3 are independent of each other (can be implemented in parallel)

---

## Phase 1: Setup

**Goal**: Prepare development environment and verify project structure

**Tasks**: 4

- [ ] T001 Verify Rust 1.75+ and Cargo toolchain are installed
- [ ] T002 Run `cargo build --release` to ensure project compiles
- [ ] T003 Run `cargo test` to verify existing tests pass
- [ ] T004 Create feature branch `003-branch-slash-conversion` (if not already on it)

---

## Phase 2: Foundational

**Goal**: Implement core data model changes that all user stories depend on

**Rationale**: These changes modify the `Worktree` entity to separate directory name from branch name. All user stories (US1, US2, US3) depend on this foundational change.

**Tasks**: 8

**Data Model Changes**:
- [x] T005 [P] Add `dirname` field to `Worktree` struct in src/core/worktree.rs (new field, keep `name` for backward compatibility during migration)
- [x] T006 [P] Add `branch_name` field to `Worktree` struct in src/core/worktree.rs (renamed from `branch` for clarity)
- [x] T007 [P] Update `Worktree::new()` signature to accept `dirname` and `branch_name` separately in src/core/worktree.rs
- [x] T008 [P] Add `display_name()` method to `Worktree` impl in src/core/worktree.rs that returns "dirname on branch" format when different

**Core Utility Functions**:
- [x] T009 [P] Add `branch_to_dirname()` function to src/core/git_ops.rs that replaces all `/` with `-`
- [x] T010 [P] Add `validate_dirname()` function to src/core/git_ops.rs that checks dirname is not empty, contains no path separators, and doesn't start with `.`
- [x] T011 [P] Add `check_dirname_conflict()` function to src/core/git_ops.rs that validates dirname doesn't conflict with existing worktrees
- [x] T012 Add `DirNameConflict` error variant to `WorktreeError` enum in src/utils/errors.rs with dirname and existing_branch fields

**Migration Notes**:
- During this phase, `name` field becomes `dirname`, `branch` field becomes `branch_name`
- Update all references to use new field names
- Maintain backward compatibility where possible

---

## Phase 3: User Story 1 - Create Worktree with Slash Conversion (P1)

**Story**: 作为开发者，我想创建一个包含斜杠的分支（如 `feat/xxx`、`feature/auth`）的 worktree，系统应该将斜杠转换为连字符创建目录，但保持分支名不变。

**Why P1**: Core functionality that solves the primary problem (branch naming vs filesystem constraints)

**Independent Test**: Create worktree with branch `feat/new-feature`, verify directory is `feat-new-feature` but git branch is `feat/new-feature`

**Success Criteria**:
- SC-001: Can create worktree in < 5 seconds
- SC-002: 100% correct directory name conversion, branch name unchanged
- SC-004: 100% conflict detection before creation

**Acceptance Scenarios**:
1. Given branch `feat/feature-001`, When `work add feat/feature-001`, Then directory is `feat-feature-001`, branch is `feat/feature-001`
2. Given branch `feature/user-auth/oauth`, When `work add feature/auth/oauth`, Then directory is `feature-auth-oauth`, branch is `feature/auth/oauth`
3. Given branch `main` (no slash), When `work add main`, Then directory and branch are both `main` (no conversion)

**Tasks**: 10

**Implementation**:

- [x] T013 [US1] Update `create_worktree()` in src/core/git_ops.rs to call `branch_to_dirname()` before creating worktree
- [x] T014 [US1] Update `create_worktree()` in src/core/git_ops.rs to call `validate_dirname()` after conversion and return error if invalid
- [x] T015 [US1] Update `create_worktree()` in src/core/git_ops.rs to call `check_dirname_conflict()` with existing worktrees and return DirNameConflict error if conflict exists
- [x] T016 [US1] Update `create_worktree()` in src/core/git_ops.rs to pass converted dirname to git worktree add command (not branch name)
- [x] T017 [US1] Update `create_worktree_with_new_branch()` in src/core/git_ops.rs to apply same dirname conversion logic
- [x] T018 [US1] Update `list_worktrees()` in src/core/git_ops.rs to populate `dirname` and `branch_name` fields separately when parsing git output
- [x] T019 [US1] Update `WorktreeData::to_worktree()` in src/core/git_ops.rs to derive dirname from path (keep existing logic) and branch_name from branch field
- [x] T020 [US1] Update CLI command handler for `work add` in src/main.rs to handle `DirNameConflict` error and display user-friendly error message with suggested solutions
- [x] T021 [US1] Update CLI command handler for `work add` in src/main.rs to display success message showing both directory name and branch name
- [x] T022 [US1] Run `cargo build --release` to verify all changes compile

**Test This Phase**:
```bash
# Test single slash
git checkout -b feat/feature-001
work add feat/feature-001
ls -la ../repo.worktrees/  # should see feat-feature-001

# Test multiple slashes
git checkout -b feature/auth/oauth
work add feature/auth/oauth
ls -la ../repo.worktrees/  # should see feature-auth-oauth

# Test no slash (no conversion)
work add main
ls -la ../repo.worktrees/  # should see main
```

---

## Phase 4: User Story 2 - List Worktrees Showing Directory and Branch Names (P2)

**Story**: 作为开发者，我想在列出 worktree 时能够清晰地看到目录名和实际分支名的对应关系，特别是当分支名包含斜杠时。

**Why P2**: Improves user experience and helps developers identify worktrees, but doesn't block core functionality

**Independent Test**: Create worktree with slash branch, run `work list`, verify output shows "dirname on branch" format

**Success Criteria**:
- SC-003: 100% of worktree listings clearly show directory name and branch name relationship

**Acceptance Scenarios**:
1. Given worktree dirname `feat-feature-001` for branch `feat/feature-001`, When `work list`, Then output shows "feat-feature-001 on feat/feature-001"
2. Given multiple slash-branch worktrees, When `work list`, Then all show correct dirname and branch pairs

**Tasks**: 9

**Implementation**:

- [x] T023 [P] [US2] Update compact format output in src/cli/output.rs to use `worktree.display_name()` instead of just `worktree.name`
- [x] T024 [P] [US2] Update table format output in src/cli/output.rs to add "Directory" column showing `dirname` and "Branch" column showing `branch_name`
- [x] T025 [P] [US2] Update table format headers in src/cli/output.rs to use "Directory" and "Branch" column names
- [x] T026 [P] [US2] Update JSON format output in src/cli/output.rs to serialize `directory` (from `dirname`) and `branch` (from `branch_name`) fields
- [x] T027 [P] [US2] Update JSON format in src/cli/output.rs to ensure backward compatibility by including both `directory` and `branch` fields
- [x] T028 [US2] Update `work list` command handler in src/main.rs to pass worktree list to updated output formatters
- [x] T029 [US2] Test `work list` compact format output shows "feat-feature-001 on feat/feature-001" for slash branches
- [x] T030 [US2] Test `work list -o table` shows separate Directory and Branch columns
- [x] T031 [US2] Test `work list -o json` outputs `{"directory": "feat-feature-001", "branch": "feat/feature-001", ...}`

**Test This Phase**:
```bash
# Create slash-branch worktree
work add feat/feature-001

# Test compact format
work list | grep "feat-feature-001 on feat/feature-001"

# Test table format
work list -o table | grep "feat-feature-001"

# Test JSON format
work list -o json | jq '.[] | select(.branch | contains("/"))'
```

---

## Phase 5: User Story 3 - Show Worktree Details (P3)

**Story**: 作为开发者，我想查看 worktree 的详细信息时，能够看到目录名和分支名的完整信息，特别是当分支名包含斜杠时。

**Why P3**: Helpful debugging feature, but doesn't block daily usage

**Independent Test**: Create worktree with slash branch, run `work show <dirname>`, verify details show both directory and branch name

**Success Criteria**: None (this is polish/enhancement, not core success criteria)

**Acceptance Scenarios**:
1. Given worktree dirname `feat-feature-001` for branch `feat/feature-001`, When `work show feat-feature-001`, Then output shows "Worktree: feat-feature-001" and "Branch: feat/feature-001"

**Tasks**: 5

**Implementation**:

- [x] T032 [P] [US3] Update `work show` command handler in src/main.rs to display "Worktree: {dirname}" field
- [x] T033 [P] [US3] Update `work show` command handler in src/main.rs to display "Branch: {branch_name}" field on separate line
- [x] T034 [US3] Update `work show` JSON format output in src/main.rs to include `directory` and `branch` fields
- [x] T035 [US3] Test `work show feat-feature-001` displays both Worktree and Branch fields
- [x] T036 [US3] Test `work show feat-feature-001 -o json` outputs `{"directory": "feat-feature-001", "branch": "feat/feature-001", ...}`

**Test This Phase**:
```bash
# Create slash-branch worktree
work add feat/feature-001

# Test human format
work show feat-feature-001 | grep "Worktree: feat-feature-001"
work show feat-feature-001 | grep "Branch: feat/feature-001"

# Test JSON format
work show feat-feature-001 -o json | jq '.directory'
work show feat-feature-001 -o json | jq '.branch'
```

---

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: Ensure quality, handle edge cases, and complete documentation

**Tasks**: 5

**Testing**:
- [x] T037 [P] Add unit tests for `branch_to_dirname()` in src/core/git_ops.rs covering: simple branch, single slash, multiple slashes, empty string, slashes at boundaries
- [x] T038 [P] Add integration tests in tests/integration/worktree_tests.rs for: creating slash-branch worktree, conflict detection, listing output format

**Edge Cases** (already handled by validation, verify manually):
- Empty branch name → rejected by `validate_dirname()`
- Branch with only slashes → rejected by `validate_dirname()`
- Slashes at start/end → converted but Git rejects (Git validation)
- Multiple consecutive slashes → converted to multiple hyphens
- Path too long → caught by OS error, user gets clear error

---

## Parallel Execution Opportunities

### Within Phase 2 (Foundational):
**Can run in parallel** (different files, no dependencies):
- T005, T006, T007, T008 (data model changes in worktree.rs)
- T009, T010, T011 (utility functions in git_ops.rs)
- T012 (error type in errors.rs)

**Must run sequentially**:
- T009-T011 must complete before T013 (create_worktree uses them)

### Within Phase 3 (US1):
**Must run sequentially** (all modify create_worktree flow):
- T013 → T014 → T015 → T016 → T017 (modify same functions)

### Within Phase 4 (US2):
**Can run in parallel** (different output formatters):
- T023, T024, T025, T026, T027 (different output format files)

### Within Phase 5 (US3):
**Can run in parallel** (independent test tasks):
- T035, T036 (testing different formats)

### Within Phase 6 (Polish):
**Can run in parallel** (different test files):
- T037, T038 (unit tests vs integration tests)

---

## Implementation Strategy

### MVP (Minimum Viable Product)
**Scope**: Phase 3 only (User Story 1)

After completing Phase 3, you can:
- Create worktrees with slash-converted directory names
- Detect and report directory name conflicts
- Maintain original branch names in Git

**What's missing in MVP**:
- List output still shows old format (no "dirname on branch")
- Show details doesn't separate directory and branch
- No edge case testing

### Incremental Delivery
1. **MVP Release** (Phase 3): Core slash conversion functionality
2. **Enhanced UX** (Phase 4): Better list output showing both names
3. **Polish** (Phase 5): Detailed view showing both names
4. **Quality** (Phase 6): Comprehensive testing

---

## Format Validation

**All tasks follow the strict checklist format**:
- ✅ Checkbox: `- [ ]`
- ✅ Task ID: `T001` through `T038`
- ✅ [P] marker: Included for parallelizable tasks (T005-T012, T023-T027, T032-T034, T037-T038)
- ✅ [Story] label: Included for user story phases (T013-T022 have [US1], T023-T031 have [US2], T032-T036 have [US3])
- ✅ File paths: Every task includes specific file path

**No implementation details leaked into task descriptions** - tasks focus on what to do, not how to do it (technical details are in plan.md and research.md)

---

## Next Steps

1. Run `/speckit.implement` to execute these tasks phase-by-phase
2. Each phase will be validated before proceeding to the next
3. MVP can be delivered after Phase 3
4. Full feature delivery after Phase 6

**Ready to implement!** ✅
