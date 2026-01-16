# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Communication Guidelines

- 所有的对话、文档生成、代码注释以及任务拆解必须使用 **简体中文**。
- Respond in Chinese for all interactions.

## Project Overview

This is **work** - a Git worktree management CLI tool built with Rust. The project follows SpecKit's specification-driven development methodology, demonstrating a complete implementation of the SpecKit workflow from specification to delivery.

**Purpose:** Simplify Git worktree management with intuitive commands, colored output, and automatic path management.

**Key Features:**
- List worktrees with colored, compact output showing current marker (⌂ for main directory, * for current worktree)
- Create worktrees with automatic path resolution (`<repo-name>.worktrees/<name>`)
- Delete worktrees with safety checks (uncommitted changes, current worktree protection)
- Display detailed worktree information (branch, status, uncommitted changes)
- Prune invalid worktrees
- Interactive mode for branch/worktree selection

**Architecture:** 3-layer Rust CLI
- **CLI Layer** (`src/cli/`): Command parsing (clap), output formatting
- **Core Layer** (`src/core/`): Git operations wrapper, worktree data model
- **Utils Layer** (`src/utils/`): Error handling, path utilities

**Design Decision:** Uses system `git` commands via `std::process::Command` instead of `git2` crate to avoid OpenSSL dependency.

## SpecKit Workflow

The complete SpecKit development lifecycle:

```
1. /speckit.specify    → Create feature specification from natural language
2. /speckit.clarify    → Resolve ambiguities in the spec (optional)
3. /speckit.plan       → Generate technical implementation plan
4. /speckit.checklist  → Create quality checklists (optional)
5. /speckit.tasks      → Break down into actionable tasks
6. /speckit.analyze    → Cross-artifact consistency check (optional)
7. /speckit.implement  → Execute the implementation
8. /speckit.taskstoissues → Convert tasks to GitHub issues (optional)
```

**Always follow this order** - each command requires artifacts from the previous phases.

## Key Commands

### `/speckit.specify` - Feature Specification

Creates numbered feature branches and specifications from natural language descriptions.

**Usage:**
```
/speckit.specify Add user authentication with OAuth2
```

**What it does:**
- Generates a 2-4 word short name (e.g., "user-auth", "oauth2-integration")
- Finds the highest existing feature number across local branches, remote branches, and specs directories
- Creates a new branch: `###-short-name` (e.g., `001-user-auth`)
- Creates spec directory: `specs/###-short-name/`
- Generates `spec.md` from template with:
  - User stories with priorities (P1, P2, P3...)
  - Functional requirements
  - Success criteria (measurable and technology-agnostic)
  - Edge cases
- Creates quality checklist at `specs/###-short-name/checklists/requirements.md`
- Validates spec completeness (max 3 [NEEDS CLARIFICATION] markers)

**Branch naming:**
- Format: `###-short-name` (3-digit padded number, hyphen, 2-4 words)
- Numbers auto-increment based on existing branches/specs
- Example: `001-user-auth`, `002-oauth2-api`, `003-analytics-dashboard`

### `/speckit.plan` - Technical Planning

Creates implementation plans and design artifacts from specifications.

**Prerequisites:** Feature spec must exist (`specs/###-feature/spec.md`)

**What it does:**
- Generates `plan.md` with:
  - Technical context (language, dependencies, storage, testing)
  - Constitution check (validates against project principles)
  - Project structure decision
- **Phase 0:** Research - resolves all "NEEDS CLARIFICATION" items
- **Phase 1:** Design - generates:
  - `research.md` - technical decisions and rationale
  - `data-model.md` - entities, fields, relationships
  - `contracts/` - API contracts (OpenAPI/GraphQL schemas)
  - `quickstart.md` - integration test scenarios
- Updates `CLAUDE.md` with new technology stack information

**Setup script:**
```bash
.specify/scripts/bash/setup-plan.sh --json
```
Returns JSON with paths: `FEATURE_SPEC`, `IMPL_PLAN`, `SPECS_DIR`, `BRANCH`

### `/speckit.tasks` - Task Breakdown

Creates actionable, dependency-ordered task lists from design artifacts.

**Prerequisites:** `plan.md` and `spec.md` required; `data-model.md`, `contracts/`, `research.md` optional

**What it does:**
- Generates `tasks.md` organized by **user story** (not by technical layer)
- Each user story is independently implementable and testable
- Tasks follow strict format: `- [ ] [TaskID] [P?] [Story?] Description with file path`
- Identifies parallelizable tasks with [P] marker
- Maps tasks to user stories with [US1], [US2], [US3] labels
- Organizes into phases:
  - Phase 1: Setup (shared infrastructure)
  - Phase 2: Foundational (blocking prerequisites)
  - Phase 3+: User stories in priority order (P1, P2, P3...)
  - Final Phase: Polish & cross-cutting concerns

**Task format examples:**
- ✅ `- [ ] T001 Create project structure per implementation plan`
- ✅ `- [ ] T005 [P] Implement authentication middleware in src/middleware/auth.py`
- ✅ `- [ ] T012 [P] [US1] Create User model in src/models/user.py`
- ✅ `- [ ] T014 [US1] Implement UserService in src/services/user_service.py`

**Check prerequisites script:**
```bash
.specify/scripts/bash/check-prerequisites.sh --json
```
Returns JSON with: `FEATURE_DIR`, `AVAILABLE_DOCS` (list of available artifacts)

**Tests are OPTIONAL** - only generate test tasks if explicitly requested in spec.md

### `/speckit.implement` - Execute Implementation

Executes tasks from `tasks.md` phase-by-phase.

**What it does:**
- Validates checklist completion (prompts if incomplete)
- Verifies/creates ignore files (.gitignore, .dockerignore, etc.)
- Executes tasks in order, respecting dependencies
- Marks tasks as [X] completed in tasks.md
- Follows TDD: tests before implementation (if tests requested)
- Validates all requirements met before completion

**Key rules:**
- Phase 1 (Setup) must complete before Phase 2 (Foundational)
- Phase 2 (Foundational) must complete before ANY user story work
- User stories can be implemented independently or in parallel
- Stop at checkpoints to validate each user story independently

### `/speckit.clarify` - Requirements Clarification

Resolves ambiguities in feature specifications.

**What it does:**
- Scans spec for 10 categories of ambiguity (scope, data model, UX, NFRs, etc.)
- Asks up to 5 targeted questions (one at a time, sequential)
- Each question provides recommended answer + options table
- Immediately integrates answers into spec
- Maintains Clarifications section with session date

### `/speckit.checklist` - Quality Checklists

Creates domain-specific quality checklists.

**What it does:**
- Asks up to 3 contextual questions about checklist intent
- Generates checklist items organized by quality dimensions:
  - Requirement Completeness
  - Requirement Clarity
  - Requirement Consistency
  - Acceptance Criteria Quality
  - Scenario Coverage
  - Edge Case Coverage
  - Non-Functional Requirements
- Each item references spec section: `[Spec §X.Y]` or `[Gap]`
- Creates NEW file each run (e.g., `ux.md`, `security.md`)

**Critical:** Checklist items test requirement quality, NOT implementation verification.

### `/speckit.analyze` - Consistency Analysis

Validates cross-artifact consistency across spec.md, plan.md, tasks.md, and constitution.md.

**Detection passes:**
- Duplication Detection
- Ambiguity Detection
- Underspecification
- Constitution Alignment
- Coverage Gaps (requirements with no tasks)
- Inconsistency (terminology drift, conflicting requirements)

Reports findings by severity: CRITICAL, HIGH, MEDIUM, LOW

### `/speckit.constitution` - Project Constitution

Defines/amends core development principles for the project.

**What it does:**
- Updates `.specify/memory/constitution.md`
- Version tracking (MAJOR.MINOR.PATCH)
- Propagates changes to dependent templates
- Generates Sync Impact Report showing all changes

### `/speckit.taskstoissues` - GitHub Integration

Converts tasks from `tasks.md` to GitHub issues.

**Requirements:** GitHub remote URL, `tasks.md` must exist

## Directory Structure

```
/Volumes/code/demos/worktree/
├── .claude/                  # Claude Code agent command definitions
│   └── commands/             # SpecKit command definitions (.md files)
│       ├── speckit.specify.md
│       ├── speckit.plan.md
│       ├── speckit.tasks.md
│       ├── speckit.implement.md
│       ├── speckit.clarify.md
│       ├── speckit.checklist.md
│       ├── speckit.analyze.md
│       ├── speckit.constitution.md
│       └── speckit.taskstoissues.md
├── .specify/                 # Core SpecKit infrastructure
│   ├── memory/               # Project-wide persistent memory
│   │   └── constitution.md   # Project constitution (v1.0.1)
│   ├── scripts/              # Bash automation scripts
│   │   └── bash/
│   │       ├── common.sh                   # Shared utilities
│   │       ├── create-new-feature.sh       # Branch + spec creation
│   │       ├── check-prerequisites.sh      # Artifact validation
│   │       ├── setup-plan.sh               # Plan initialization
│   │       └── update-agent-context.sh     # AI agent file updates
│   └── templates/            # Template files for artifacts
│       ├── spec-template.md
│       ├── plan-template.md
│       ├── tasks-template.md
│       ├── checklist-template.md
│       └── agent-file-template.md
├── specs/                    # Feature specifications
│   └── 001-git-worktree-cli/ # Initial implementation (MVP complete)
│       ├── spec.md           # Feature specification
│       ├── plan.md           # Implementation plan
│       ├── tasks.md          # Task breakdown (47/62 complete)
│       ├── research.md       # Technical research
│       └── IMPLEMENTATION_REPORT.md  # MVP status
├── src/                      # Rust source code
│   ├── main.rs               # CLI entry point, command handlers
│   ├── cli/
│   │   └── output.rs         # Output formatting (table, compact, JSON)
│   ├── core/
│   │   ├── git_ops.rs        # Git command execution wrapper
│   │   ├── worktree.rs       # Worktree data model
│   │   └── repository.rs     # Repository management
│   └── utils/
│       ├── errors.rs         # Error types and handling
│       └── path.rs           # Path utilities
├── Cargo.toml                # Rust project configuration
├── Cargo.lock                # Dependency lock file
├── README.md                 # User documentation
└── CLAUDE.md                 # This file
```

## Build & Development Commands

```bash
# Build the project
cargo build --release

# Install globally (after build)
cargo install --path .

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- list

# Format code
cargo fmt

# Check lints
cargo clippy
```

## Bash Scripts

All bash scripts in `.specify/scripts/bash/` are executable and support `--json` output mode:

### `create-new-feature.sh`

Creates numbered feature branches and spec directories.

```bash
# Usage with JSON output for parsing
.specify/scripts/bash/create-new-feature.sh --json "Add user authentication"

# With explicit number and short-name
.specify/scripts/bash/create-new-feature.sh --json --number 5 --short-name "user-auth" "Add user authentication"

# Output: {"BRANCH_NAME":"001-user-auth","SPEC_FILE":"/path/to/specs/001-user-auth/spec.md"}
```

**Features:**
- Auto-detects next available number from local/remote branches and specs directories
- Creates git branch and checks it out
- Creates spec directory
- Initializes spec.md from template
- Supports non-git repos via `SPECIFY_FEATURE` environment variable

### `check-prerequisites.sh`

Validates required artifacts exist and returns available documents.

```bash
# Usage
.specify/scripts/bash/check-prerequisites.sh --json

# Output: {"FEATURE_DIR":"/path/to/specs/001-feature","AVAILABLE_DOCS":["spec.md","plan.md",...]}
```

### `setup-plan.sh`

Initializes plan.md from template for current feature.

```bash
# Usage
.specify/scripts/bash/setup-plan.sh --json

# Output: {"FEATURE_SPEC":"/path/to/spec.md","IMPL_PLAN":"/path/to/plan.md","SPECS_DIR":"/path/to/specs","BRANCH":"001-feature"}
```

### `update-agent-context.sh`

Updates AI agent context files (CLAUDE.md, etc.) with technology stack from plan.md.

```bash
# Usage for specific agent
.specify/scripts/bash/update-agent-context.sh claude
.specify/scripts/bash/update-agent-context.sh gemini
.specify/scripts/bash/update-agent-context.sh copilot

# Supports 15+ AI coding tools
```

**What it does:**
- Reads `plan.md` Technical Context section
- Extracts language, frameworks, storage, testing info
- Updates agent-specific file with new tech stack
- Preserves manual additions between `<!-- MANUAL ADDITIONS START -->` markers

## Architecture Principles

### Feature-First Organization

Each feature gets its own numbered branch and spec directory. Numbering is global across local branches, remote branches, and specs directories.

### Artifact Separation

Clear separation between:
- **Specification** (spec.md) - WHAT users need and WHY
- **Planning** (plan.md, research.md, data-model.md) - HOW it will be built
- **Tasks** (tasks.md) - WHAT needs to be done, in order
- **Implementation** - The actual code

### AI-Agent Agnostic

Works with multiple AI coding assistants:
- Claude (CLAUDE.md)
- GitHub Copilot (.github/agents/copilot-instructions.md)
- Cursor (.cursor/rules/specify-rules.mdc)
- Gemini (GEMINI.md)
- And 10+ others

### Git-Native Workflow

Deep integration with git:
- Auto-creates numbered branches
- Checks for existing branches to prevent conflicts
- Supports non-git repos via environment variable

### Template-Driven

All artifacts generated from well-defined templates in `.specify/templates/`

## Quality Gates

### Specification Quality

Automatic validation during `/speckit.specify`:
- Maximum 3 [NEEDS CLARIFICATION] markers
- All mandatory sections completed
- Requirements testable and unambiguous
- Success criteria measurable and technology-agnostic
- Creates checklist at `checklists/requirements.md`

### Planning Quality

Automatic validation during `/speckit.plan`:
- Constitution check gates (must pass or justify violations)
- All "NEEDS CLARIFICATION" resolved in research phase
- Data model, contracts, quickstart artifacts generated

### Task Quality

Automatic validation during `/speckit.tasks`:
- Every task follows strict checklist format
- Each user story independently testable
- File paths included in all task descriptions
- Parallel opportunities identified with [P] markers

### Implementation Quality

Automatic validation during `/speckit.implement`:
- Checklist validation before starting
- Ignore file verification/creation
- Phase-by-phase validation checkpoints
- Tests fail before implementation (if TDD)

## Conventions

### Naming Conventions

- **Branches:** `###-short-name` (3-digit number, hyphen, 2-4 words)
- **Spec Directories:** Same as branch names in `specs/`
- **Feature Numbers:** Auto-incrementing, padded to 3 digits (001, 002, ...)
- **Task IDs:** Sequential with T prefix (T001, T002, ...)
- **Checklist Items:** Sequential with CHK prefix (CHK001, CHK002, ...)
- **User Stories:** US1, US2, US3 mapped to priorities P1, P2, P3

### File Organization

- **Mandatory Artifacts:** spec.md, plan.md, tasks.md
- **Optional Artifacts:** research.md, data-model.md, quickstart.md, contracts/
- **Checklists:** `checklists/[domain].md` (e.g., ux.md, security.md, api.md)
- **AI Context:** Auto-generated files (CLAUDE.md, etc.) at repo root

### Quality Principles

1. **Technology-Agnostic Specs:** No implementation details in requirements
2. **Measurable Success Criteria:** Quantifiable outcomes, not "fast/intuitive"
3. **Independent User Stories:** Each story deliverable as standalone MVP increment
4. **Traceability:** Every task traced to user story, every requirement to spec section
5. **Constitution Compliance:** All development must respect core principles

## Workflow Rules

1. **No Skipping:** Follow command order (can't implement without tasks)
2. **Validation First:** Quality gates must pass before proceeding
3. **Incremental Delivery:** MVP = User Story 1 only
4. **Parallel When Possible:** Mark tasks [P] for parallel execution
5. **Test Before Code:** TDD approach (if tests requested)

## Common Patterns

### Creating a New Feature

```bash
# 1. Create specification
/speckit.specify Add user authentication with OAuth2

# 2. (Optional) Clarify ambiguities
/speckit.clarify

# 3. Generate technical plan
/speckit.plan

# 4. (Optional) Create quality checklists
/speckit.checklist

# 5. Break down into tasks
/speckit.tasks

# 6. (Optional) Analyze consistency
/speckit.analyze

# 7. Implement
/speckit.implement
```

### Feature Directory Resolution

Uses **numeric prefix matching** (not exact branch match):
- `001-feature-name` branch → `specs/001-feature-name/`
- `001-feature-alternate` branch → `specs/001-feature-name/` (same directory)
- Allows multiple branches per spec

### Non-Git Workflow

Set environment variable:
```bash
export SPECIFY_FEATURE=001-user-auth
```

Scripts will use this instead of git branch detection.

## Technology Stack

**Current Stack:** Rust 1.75+ with system Git integration

- **Language:** Rust 1.75+ (edition 2021)
- **CLI Framework:** clap 4.5 (derive feature for command parsing)
- **Error Handling:** anyhow 1.0 (error propagation), thiserror 1.0 (custom error types)
- **Serialization:** serde 1.0, serde_json 1.0 (JSON output format)
- **Output Formatting:** comfy-table 7.0 (table display), colored 2.1 (terminal colors)
- **Logging:** env_logger 0.11, log 0.4
- **Time Handling:** chrono 0.4 (with serde support)
- **Interactive UI:** dialoguer 0.11 (interactive prompts)
- **Testing:** tempfile 3.10, assert_cmd 2.0, predicates 3.1

**Git Integration:** System commands via `std::process::Command`
- Uses `git rev-parse --git-common-dir` to find main repository
- Executes `git worktree` commands for all worktree operations
- Avoids `git2` crate dependency (no OpenSSL required)

**Release Configuration:**
- Optimizations: opt-level 3, LTO enabled, single codegen unit
- Binary stripping enabled for minimal size
- Binary name: `work`

## Constitution

The project constitution (`.specify/memory/constitution.md`, v1.0.1) defines core development principles:

### I. 规范优先开发 (Specification-First Development)
- All features must start with natural language specification (`/speckit.specify`)
- Separate WHAT (requirements) from HOW (implementation)
- Technical decisions recorded in planning phase (`/speckit.plan`)

### II. 独立可交付性 (Independent Deliverability)
- User stories prioritized (P1, P2, P3)
- P1 must be independently deliverable as MVP
- Avoid cross-story dependencies

### III. 质量前置 (Quality Upfront)
- All specs must pass completeness validation
- Max 3 [NEEDS CLARIFICATION] markers before planning
- Encourage TDD (tests fail before implementation)

## Visual Design

**Worktree Display Colors:**
- Main directory: Purple + bold + ⌂ roof symbol
- Regular worktrees: Cyan
- Current worktree: Green * marker
- Branch name (when different from directory): Yellow
- Modified status: Red
- Detached HEAD: Yellow

**Example Output:**
```
*⌂  worktree on 001-git-worktree-cli (modified)
  feature-auth on main
  feature-bugfix
```

## Key Implementation Details

### Worktree Path Resolution

When creating a new worktree, the tool uses `git rev-parse --git-common-dir` to find the main repository's `.git` directory. This ensures worktrees are created correctly even when the command is run from within another worktree.

**Path Pattern:** `<parent-dir>/<repo-name>.worktrees/<worktree-name>`

Example: If main repo is at `/Volumes/code/myproject/`, worktrees are created at `/Volumes/code/myproject.worktrees/<name>/`

### Worktree Naming

Worktree names are based on **directory name**, not branch name. This allows:
- Switching branches within a worktree without changing its identity
- Clear separation between "where" (directory) and "what" (current branch)

When directory name differs from current branch, output shows: `dirname on branchname`

### Git Operations Pattern

All git operations use `std::process::Command` instead of libgit2:
```rust
let output = Command::new("git")
    .args(["worktree", "list", "--porcelain"])
    .output()?;
```

**Benefits:**
- No OpenSSL dependency (avoids compilation issues)
- Always uses user's configured git version
- Simpler error handling via stdout/stderr parsing

### Error Handling Strategy

- **Core layer** (`core/git_ops.rs`): Returns `Result<T>` with custom `WorktreeError` from `utils/errors.rs`
- **CLI layer** (`main.rs`): Uses `anyhow::Result<T>` for flexible error propagation to user
- **User-facing errors**: Colored, context-rich error messages

### Output Formatting

Three formats supported (via `-o/--output` flag):
1. **compact** (default): Simplified, color-coded single line per worktree
2. **table**: Full table with all columns using comfy-table
3. **json**: Machine-readable JSON for scripting

## Active Technologies
- Rust 1.75+ (001-git-worktree-cli)
- System Git (no persistent storage) (001-git-worktree-cli)
- YAML (GitHub Actions Workflow) + Bash scripting (002-fix-release)
- N/A (CI/CD 流程，无持久化存储) (002-fix-release)
- Rust 1.75+ (edition 2021) + clap 4.5 (CLI), anyhow 1.0 (错误处理), thiserror 1.0 (自定义错误), serde 1.0 (序列化), comfy-table 7.0 (表格输出), colored 2.1 (颜色输出), chrono 0.4 (时间处理) (003-branch-slash-conversion)
- 无持久化存储（仅 Git worktree 元数据） (003-branch-slash-conversion)
- Rust 1.75+ (edition 2021) + clap 4.5 (CLI), dialoguer 0.11 (交互式界面), anyhow 1.0 (错误处理) (005-worktree-branch-source)

## Recent Changes
- 001-git-worktree-cli: Added colored output with main directory marker
- 001-git-worktree-cli: Fixed worktree path resolution using git-common-dir
- 001-git-worktree-cli: Separated directory name from branch name display
