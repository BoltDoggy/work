# Implementation Plan: 修复 GitHub Actions Release Workflow

**Branch**: `002-fix-release` | **Date**: 2026-01-10 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-fix-release/spec.md`

## Summary

修复 GitHub Actions release workflow 中的关键问题：构建成功但二进制文件未上传到 GitHub Release Downloads 区域。经分析，问题出在 artifacts 上传/下载路径不匹配。build job 将 artifacts 按平台名称分别上传到不同子目录，但 release job 使用 `download-artifact@v4` 下载时会保留目录结构，导致 `softprops/action-gh-release` 无法正确找到文件。

**技术方案**：
1. 修改 download-artifact 步骤，使用 `pattern-matcher` 或直接指定文件路径
2. 或修改 upload-artifact 步骤，使用统一名称上传所有文件
3. 添加验证步骤确保文件正确上传到 Release

## Technical Context

**Language/Version**: YAML (GitHub Actions Workflow) + Bash scripting
**Primary Dependencies**:
- `actions/checkout@v4` - 代码检出
- `dtolnay/rust-toolchain@stable` - Rust 工具链
- `actions/upload-artifact@v4` - 上传构建产物
- `actions/download-artifact@v4` - 下载构建产物
- `softprops/action-gh-release@v1` - 创建 GitHub Release

**Storage**: N/A (CI/CD 流程，无持久化存储)
**Testing**: 手动测试（推送 tag 验证）+ GitHub Actions 日志验证
**Target Platform**: GitHub Actions (ubuntu-latest, macos-latest, windows-latest)
**Project Type**: Single project (Rust CLI 工具)
**Performance Goals**: 构建时间 < 15 分钟，Release 创建时间 < 2 分钟
**Constraints**: GitHub Actions 免费版配额（每月 2000 分钟）
**Scale/Scope**: 5 个平台 × 1-5 MB 文件 = 总计 ~25 MB per release

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. 规范优先开发 ✅

- ✅ 功能从自然语言规范开始（`spec.md` 明确描述用户需求和期望）
- ✅ 规范聚焦用户价值（用户能下载预编译二进制文件）
- ✅ 规范包含可测试需求（所有 FR 都有明确验收标准）
- ✅ 技术决策在此计划中记录（见 research.md）

### II. 独立可交付性 ✅

- ✅ P1 用户故事独立可测试（推送 tag 验证 Release 产物）
- ✅ P1 作为 MVP 可独立交付（修复核心上传问题）
- ✅ P2 作为增强功能独立（checksums 验证）
- ⚠️ **复杂度追踪**：此功能修复现有 CI/CD 流程，不涉及新增代码模块，因此"任务按用户故事拆分"不适用。这是配置文件修复，而非功能开发。

**理由**：CI/CD workflow 修复本质上是配置调整，不涉及源代码开发。独立可交付性原则通过每个平台构建的独立性来体现（构建失败不影响其他平台）。

### III. 质量前置 ✅

- ✅ 规范通过完整性验证（0 个 [NEEDS CLARIFICATION] 标记）
- ✅ 所有技术澄清在 research.md 中解决
- ✅ 规划通过宪章检查
- ⚠️ **复杂度追踪**：此功能不适用 TDD（配置文件无法单元测试）。验证通过实际 tag 推送和 GitHub Actions 日志完成。

**理由**：GitHub Actions workflow 是声明式配置，无法进行传统单元测试。质量保证通过：
1. YAML 语法验证（GitHub Actions 自动完成）
2. 手动测试 tag 推送验证端到端流程
3. 检查 Actions 日志确认每个步骤执行

### 治理合规性 ✅

- ✅ 文档要求满足：spec.md、research.md（本文件）、data-model.md（N/A）
- ✅ 代码风格：遵循 YAML 最佳实践和 GitHub Actions 官方文档
- ✅ 版本控制：使用编号功能分支 `002-fix-release`
- ✅ 提交信息：遵循约定式提交规范（见 git log）
- ✅ 质量保证：规范通过验证，规划通过宪章检查

**Gate 结果**: ✅ **PASS** (2 个记录的例外已合理说明)

## Project Structure

### Documentation (this feature)

```text
specs/002-fix-release/
├── plan.md              # 本文件
├── research.md          # Phase 0 输出（待生成）
├── data-model.md        # N/A（无数据模型）
├── quickstart.md        # Phase 1 输出（待生成）
├── contracts/           # N/A（无 API 契约）
└── tasks.md             # Phase 2 输出（由 /speckit.tasks 命令生成）
```

### Source Code (repository root)

```text
# 修改的文件
.github/workflows/
└── release.yml          # 主要修改对象

# 验证文件
specs/002-fix-release/
└── test-plan.md         # 手动测试计划（可选）
```

**Structure Decision**: Single project (Rust CLI)。此功能仅修改现有 workflow 配置文件，不涉及源代码结构变更。

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| 不适用 TDD | GitHub Actions workflow 是声明式配置，无法编写单元测试 | 配置文件只能通过端到端测试验证（推送 tag 验证） |
| 不适用按用户故事拆分任务 | CI/CD workflow 修复是单一配置文件调整，不是功能开发 | 将配置拆分为多个任务反而增加复杂度和风险 |

## Phase 0: Research & Decisions

### 问题诊断

#### 当前症状
1. ✅ 构建成功：所有 5 个平台二进制文件编译通过
2. ✅ Artifacts 上传成功：upload-artifact 步骤无错误
3. ✅ Artifacts 下载成功：download-artifact 步骤无错误
4. ❌ Release 无文件：GitHub Release Downloads 区域为空

#### 根本原因分析

通过审查 `.github/workflows/release.yml` (第 91-129 行)：

**Upload 步骤** (第 91-97 行)：
```yaml
- name: Upload tarball/zip
  uses: actions/upload-artifact@v4
  with:
    name: ${{ matrix.asset_name }}  # ❌ 问题：按平台分别命名
    path: |
      ${{ matrix.asset_name }}.*
```

这导致 artifacts 被上传到不同的命名空间：
- `work-linux-x86_64/` → `work-linux-x86_64.tar.gz`
- `work-linux-aarch64/` → `work-linux-aarch64.tar.gz`
- `work-macos-x86_64/` → `work-macos-x86_64.tar.gz`
- 等等...

**Download 步骤** (第 107-110 行)：
```yaml
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    path: artifacts  # ❌ 问题：保留目录结构
```

下载后的文件结构：
```
artifacts/
├── work-linux-x86_64/
│   └── work-linux-x86_64.tar.gz
├── work-linux-aarch64/
│   └── work-linux-aarch64.tar.gz
└── ...
```

**Release 步骤** (第 117-129 行)：
```yaml
- name: Create Release
  uses: softprops/action-gh-release@v1
  with:
    files: |
      artifacts/*.*  # ❌ 问题：期望在 artifacts/ 下直接找到文件
```

但实际文件在 `artifacts/work-linux-x86_64/work-linux-x86_64.tar.gz`，导致匹配失败。

### 技术决策

#### Decision 1: Artifacts 上传策略

**选择**: 统一命名 + 目录展平

**方案 A**: 修改 upload-artifact，使用统一名称
```yaml
- name: Upload tarball/zip
  uses: actions/upload-artifact@v4
  with:
    name: release-artifacts  # 统一名称
    path: |
      ${{ matrix.asset_name }}.*
```

**优点**:
- 所有文件在同一命名空间
- download-artifact 自动合并到同一目录

**缺点**:
- 失去平台级别的命名隔离
- 文件名必须包含完整平台信息（已满足）

**方案 B**: 使用 `pattern-matcher` 扁平化下载
```yaml
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    path: artifacts
    pattern: release-artifacts-*  # 匹配所有
    merge-multiple: true  # 合并到同一目录
```

**优点**:
- 保持平台级别隔离
- 更细粒度控制

**缺点**:
- 需要统一 artifact 命名规范
- 配置更复杂

**决策**: **方案 A** - 统一命名

**理由**:
1. 简单直接，减少配置复杂度
2. GitHub Actions 官方文档推荐的做法
3. 更易于维护和调试
4. 文件名已包含完整平台信息，无需额外区分

#### Decision 2: Checksums 生成位置

**选择**: 在 Release job 下载后生成

**方案 A**: 在每个 Build job 中生成 checksums
```yaml
# 在 build job 中
- name: Generate checksum
  run: sha256sum ${{ matrix.asset_name }}.* > checksum.txt
```

**方案 B**: 在 Release job 下载后统一生成
```yaml
# 在 release job 中
- name: Create checksums
  run: |
    cd artifacts
    find . -type f -exec sha256sum {} \; | sort -k2 | tee checksums.txt
```

**决策**: **方案 B** - 在 Release job 中生成

**理由**:
1. 避免在多个 job 中重复代码
2. 确保 checksums.txt 包含所有文件的完整列表
3. 符合现有代码结构（第 112-115 行已实现）
4. 更易于验证和调试

#### Decision 3: 文件路径通配符策略

**选择**: 使用明确的文件模式

**当前实现** (第 120-121 行)：
```yaml
files: |
  artifacts/*.*
```

**问题**: `*.*` 无法匹配嵌套目录中的文件（如 `artifacts/work-linux-x86_64/work-linux-x86_64.tar.gz`）

**改进方案**:
```yaml
files: |
  artifacts/**/*.tar.gz
  artifacts/**/*.zip
  artifacts/checksums.txt
```

**决策**: **使用递归通配符 `**/*.tar.gz`**

**理由**:
1. 明确指定文件类型，避免意外匹配
2. `**` 递归匹配所有子目录
3. 更安全，防止上传临时文件
4. 便于后续添加其他文件类型（如 .sha256 文件）

### 实现策略

#### Phase 1: 修复 Artifacts 上传（P1）

1. **修改 upload-artifact 步骤**：
   - 将 `name: ${{ matrix.asset_name }}` 改为 `name: release-artifacts`
   - 添加 `if-no-files-found: error` 确保文件存在

2. **修改 download-artifact 步骤**：
   - 添加 `pattern: release-artifacts`
   - 添加 `merge-multiple: true` 展平目录结构

3. **验证文件结构**：
   - 添加 `ls -R artifacts` 步骤查看下载的文件结构
   - 确保所有 tar.gz/zip 文件在 artifacts/ 根目录

#### Phase 2: 修复 Release 上传（P1）

1. **更新 softprops/action-gh-release 的 files 参数**：
   - 使用 `artifacts/**/*.tar.gz` 和 `artifacts/**/*.zip`
   - 添加 `artifacts/checksums.txt`

2. **添加失败重试**：
   - 使用 `continue-on-error: false` 确保失败时停止
   - 添加明确的错误消息

#### Phase 3: 验证和文档（P2）

1. **添加调试步骤**：
   - 显示上传的文件列表
   - 显示 Release 创建后的 URL

2. **更新 quickstart.md**：
   - 记录完整的发布流程
   - 提供故障排除指南

### 研究结果摘要

**问题根因**: artifacts 上传/下载路径不匹配导致文件嵌套在子目录中，`softprops/action-gh-release` 无法正确找到。

**解决方案**:
1. 统一 artifact 命名 (`release-artifacts`)
2. 展平下载目录结构 (`merge-multiple: true`)
3. 使用递归通配符 (`**/*.tar.gz`)

**预期结果**:
- 所有 5 个平台的二进制文件正确上传到 Release Downloads
- checksums.txt 包含所有文件的 SHA256 哈希
- 用户能成功下载和验证文件

## Phase 1: Design Artifacts

### data-model.md

**说明**: 此功能不涉及数据模型变更。GitHub Actions workflow 是声明式配置，不定义数据结构。

**相关实体**:
- **Artifact**: GitHub Actions 构建产物，包含名称、路径、类型（tar.gz/zip）
- **Release**: GitHub Release，包含 tag、名称、描述、assets（下载文件）

### contracts/

**说明**: 此功能不涉及 API 契约。不是 REST API 或 GraphQL 服务。

### quickstart.md

**说明**: 将在 Phase 1 生成完整的快速验证指南。
