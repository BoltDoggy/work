# Feature Specification: 修复 GitHub Actions Release Workflow

**Feature Branch**: `002-fix-release`
**Created**: 2026-01-10
**Status**: Draft
**Input**: User description: "修复 .github/workflows, 当我 push tag 后能正常运行 github action 也能正常构建成功，但完成后downloads 里没有构建的产物"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 发布构建产物到 GitHub Release (Priority: P1)

开发者推送版本 tag（如 v0.1.1）后，GitHub Actions 自动构建所有平台的二进制文件，并成功上传到 GitHub Release 的 Downloads 区域，使用户可以直接下载预编译的二进制文件。

**Why this priority**: 这是核心发布流程。构建成功但产物未上传导致用户无法获取预编译版本，违背了自动化发布的初衷。这是阻塞性问题。

**Independent Test**: 推送一个测试 tag（如 v0.9.0-test），验证 GitHub Actions 完成后，在 GitHub Releases 页面能看到所有平台的二进制文件（.tar.gz 和 .zip）可供下载。

**Acceptance Scenarios**:

1. **Given** 开发者在本地仓库推送了 tag `v1.0.0`, **When** GitHub Actions workflow 完成执行, **Then** 在 GitHub Releases 页面创建的 v1.0.0 release 中包含所有 5 个平台的二进制文件（linux-x86_64.tar.gz, linux-aarch64.tar.gz, macos-x86_64.tar.gz, macos-aarch64.tar.gz, windows-x86_64.zip）
2. **Given** Release 已创建, **When** 用户访问 Releases 页面, **Then** 能够看到并下载对应平台的二进制文件
3. **Given** 构建产物包含 checksums.txt, **When** Release 创建成功, **Then** checksums.txt 文件也出现在 Downloads 区域

---

### User Story 2 - 验证构建产物的完整性 (Priority: P2)

用户下载二进制文件后，能够通过提供的 checksums.txt 验证文件完整性，确保下载的文件未被篡改。

**Why this priority**: 这是安全性增强功能，但不阻塞基本发布流程。用户可以在获取文件后验证，但不影响文件可用性。

**Independent Test**: 下载任意平台的 tar.gz 文件，使用 `sha256sum` 命令计算哈希值，与 checksums.txt 中的值对比，验证一致。

**Acceptance Scenarios**:

1. **Given** Release 包含多个二进制文件, **When** 用户查看 Downloads 区域, **Then** 能够看到 checksums.txt 文件
2. **Given** 用户下载了 tar.gz 文件和 checksums.txt, **When** 运行 `sha256sum -c checksums.txt`, **Then** 所有文件验证通过显示 OK
3. **Given** 文件被意外损坏, **When** 运行验证命令, **Then** 能够检测到哈希不匹配并报错

---

### Edge Cases

- 当某个平台构建失败时，其他平台的产物是否仍需上传？
- 当 GitHub Release 已存在时（如手动创建），是否需要覆盖或合并？
- 当 artifacts 下载失败时，是否需要重试机制？
- 当 checksums.txt 生成失败时，是否需要继续发布其他文件？
- 当 tag 名称不符合 v* 格式时（如 0.1.1 而非 v0.1.1），workflow 是否应触发？

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: GitHub Actions workflow 必须在推送符合 v* 格式的 tag 时自动触发
- **FR-002**: Workflow 必须成功构建所有 5 个目标平台的二进制文件（x86_64/aarch64 Linux/macOS, x86_64 Windows）
- **FR-003**: Build job 必须成功将构建产物（.tar.gz 或 .zip）上传为 artifacts
- **FR-004**: Release job 必须成功从 build job 下载所有 artifacts
- **FR-005**: Release job 必须将下载的 artifacts 正确上传到 GitHub Release 的 Downloads 区域
- **FR-006**: Release 必须包含 checksums.txt 文件，列出所有二进制文件的 SHA256 哈希值
- **FR-007**: 每个 artifact 的名称必须匹配预期格式（work-linux-x86_64.tar.gz, work-macos-aarch64.tar.gz 等）
- **FR-008**: Workflow 失败时必须在 GitHub Actions 界面显示明确的错误信息

### Key Entities

- **GitHub Actions Workflow**: 自动化构建和发布流程的定义
- **Build Job**: 负责编译和打包各平台二进制文件的任务
- **Release Job**: 负责收集构建产物并创建 GitHub Release 的任务
- **Artifact**: 构建产物，包括压缩的二进制文件和校验和文件
- **GitHub Release**: GitHub 上的发布版本，包含版本号、描述和可下载文件

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 推送 tag 后 10 分钟内，GitHub Release 页面显示所有 5 个平台的二进制文件
- **SC-002**: 所有上传的二进制文件大小在合理范围内（Linux/macOS: 1-5 MB, Windows: 1-5 MB）
- **SC-003**: 每次发布的 Downloads 区域包含准确且完整的 checksums.txt 文件
- **SC-004**: 用户能够成功下载并解压所有平台的二进制文件
- **SC-005**: 验证 checksums 后，所有文件的哈希值完全匹配
- **SC-006**: 100% 的推送 tag 触发的 workflow 都成功完成（无失败或部分失败）
- **SC-007**: 用户能够通过 `sha256sum -c checksums.txt` 命令验证所有文件完整性
