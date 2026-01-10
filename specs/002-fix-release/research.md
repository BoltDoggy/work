# Research: 修复 GitHub Actions Release Workflow

**Feature**: 002-fix-release
**Date**: 2026-01-10
**Status**: Complete

## 问题诊断

### 当前状态

**观察到的行为**:
1. ✅ 构建成功：所有 5 个平台二进制文件编译通过
2. ✅ Artifacts 上传成功：`actions/upload-artifact@v4` 步骤无错误
3. ✅ Artifacts 下载成功：`actions/download-artifact@v4` 步骤无错误
4. ✅ Release 创建成功：GitHub Release 页面生成
5. ❌ **核心问题**: Release Downloads 区域为空，无二进制文件可下载

### 根本原因分析

通过代码审查 `.github/workflows/release.yml`（第 91-129 行），定位到以下问题：

#### 1. Upload-Artifact 配置问题

**当前实现** (第 91-97 行):
```yaml
- name: Upload tarball/zip
  uses: actions/upload-artifact@v4
  with:
    name: ${{ matrix.asset_name }}  # ❌ 按平台分别命名
    path: |
      ${{ matrix.asset_name }}.*
```

**问题**: 每个平台的 artifact 被上传到独立的命名空间
- `work-linux-x86_64/` → 包含 `work-linux-x86_64.tar.gz`
- `work-linux-aarch64/` → 包含 `work-linux-aarch64.tar.gz`
- `work-macos-x86_64/` → 包含 `work-macos-x86_64.tar.gz`
- 等等...

#### 2. Download-Artifact 配置问题

**当前实现** (第 107-110 行):
```yaml
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    path: artifacts  # ❌ 保留目录结构
```

**问题**: 下载时保留原始目录结构，导致：
```
artifacts/
├── work-linux-x86_64/
│   └── work-linux-x86_64.tar.gz
├── work-linux-aarch64/
│   └── work-linux-aarch64.tar.gz
└── ...
```

#### 3. Release 步骤文件匹配问题

**当前实现** (第 117-129 行):
```yaml
- name: Create Release
  uses: softprops/action-gh-release@v1
  with:
    files: |
      artifacts/*.*  # ❌ 无法匹配嵌套目录中的文件
```

**问题**: `artifacts/*.*` 只能匹配 `artifacts/` 根目录下的文件，无法匹配嵌套的 `artifacts/work-linux-x86_64/work-linux-x86_64.tar.gz`

## 技术决策

### Decision 1: Artifacts 命名策略

**选择**: **统一命名策略** (方案 A)

**实现**:
```yaml
- name: Upload tarball/zip
  uses: actions/upload-artifact@v4
  with:
    name: release-artifacts  # 统一名称
    path: |
      ${{ matrix.asset_name }}.*
```

**理由**:
1. **简化配置**: 所有文件在同一命名空间，下载时自动合并
2. **官方推荐**: GitHub Actions 官方文档推荐做法
3. **易于调试**: 减少目录层级，文件结构清晰
4. **文件名区分**: 文件名已包含完整平台信息（如 `work-linux-x86_64.tar.gz`），无需额外区分

**替代方案** (方案 B - 未选择):
```yaml
# 使用 pattern-matcher + merge-multiple
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    path: artifacts
    pattern: release-artifacts-*
    merge-multiple: true
```

**拒绝理由**: 配置更复杂，需要统一的命名前缀，维护成本高

### Decision 2: Checksums 生成时机

**选择**: **在 Release job 下载后统一生成** (方案 B)

**实现** (已存在于第 112-115 行):
```yaml
- name: Create checksums
  run: |
    cd artifacts
    find . -type f -exec sha256sum {} \; | sort -k2 | tee checksums.txt
```

**理由**:
1. **避免重复**: 不需要在每个 build job 中生成
2. **完整性**: 确保包含所有文件的 SHA256 哈希
3. **易于维护**: 集中管理，单一职责
4. **代码复用**: 已有的实现无需修改

**替代方案** (方案 A - 未选择):
在每个 build job 中生成：
```yaml
- name: Generate checksum
  run: sha256sum ${{ matrix.asset_name }}.* > checksum.txt
```

**拒绝理由**:
- 在 5 个 job 中重复代码
- 需要额外的合并步骤
- 不符合 DRY 原则

### Decision 3: 文件路径通配符

**选择**: **使用递归通配符**

**实现**:
```yaml
files: |
  artifacts/**/*.tar.gz
  artifacts/**/*.zip
  artifacts/checksums.txt
```

**理由**:
1. **明确匹配**: 指定具体文件类型，避免意外匹配
2. **递归搜索**: `**` 匹配所有子目录
3. **安全性**: 防止上传临时文件或日志文件
4. **可扩展**: 便于添加其他文件类型（如 `.sha256`）

**当前实现的问题** (第 120-121 行):
```yaml
files: |
  artifacts/*.*  # 只匹配 artifacts/ 根目录的一级文件
```

## 实现方案

### Phase 1: 修复 Artifacts 上传/下载

**目标**: 确保 artifacts 正确上传和下载到扁平目录结构

**步骤**:

1. **修改 upload-artifact** (第 91-97 行):
   ```yaml
   - name: Upload tarball/zip
     uses: actions/upload-artifact@v4
     with:
       name: release-artifacts  # 改为统一名称
       path: |
         ${{ matrix.asset_name }}.*
       if-no-files-found: error  # 添加错误检查
   ```

2. **修改 download-artifact** (第 107-110 行):
   ```yaml
   - name: Download all artifacts
     uses: actions/download-artifact@v4
     with:
       pattern: release-artifacts  # 匹配统一名称
       path: artifacts
       merge-multiple: true  # 展平目录结构
   ```

3. **添加验证步骤**:
   ```yaml
   - name: Verify artifacts structure
     run: |
       echo "Downloaded artifacts:"
       ls -R artifacts
       echo "---"
       echo "Tar.gz files:"
       find artifacts -name "*.tar.gz"
       echo "Zip files:"
       find artifacts -name "*.zip"
   ```

**预期结果**:
```
artifacts/
├── work-linux-x86_64.tar.gz
├── work-linux-aarch64.tar.gz
├── work-macos-x86_64.tar.gz
├── work-macos-aarch64.tar.gz
├── work-windows-x86_64.zip
└── checksums.txt
```

### Phase 2: 修复 Release 上传

**目标**: 确保 softprops/action-gh-release 正确找到并上传文件

**步骤**:

1. **更新 files 参数** (第 120-121 行):
   ```yaml
   files: |
     artifacts/**/*.tar.gz
     artifacts/**/*.zip
     artifacts/checksums.txt
   ```

2. **添加失败处理**:
   ```yaml
   - name: Create Release
     uses: softprops/action-gh-release@v1
     with:
       files: |
         artifacts/**/*.tar.gz
         artifacts/**/*.zip
         artifacts/checksums.txt
       fail_on_unmatched_files: true  # 添加此行
     continue-on-error: false  # 确保失败时停止
   ```

3. **添加验证步骤**:
   ```yaml
   - name: Verify Release
     run: |
       echo "Release created at:"
       echo "${{ github.server_url }}/${{ github.repository }}/releases/tag/${{ github.ref_name }}"
       echo "---"
       echo "Files uploaded:"
       gh release view ${{ github.ref_name }} --json assets --jq '.assets[].name'
   ```

### Phase 3: 添加调试和文档

**目标**: 便于后续维护和故障排除

**步骤**:

1. **添加详细日志**:
   - 在 upload-artifact 后显示上传的文件列表
   - 在 download-artifact 后显示下载的文件结构
   - 在 release 创建后显示 Release URL 和上传的文件

2. **更新 quickstart.md**:
   - 记录完整的发布流程
   - 提供故障排除指南
   - 包含常见错误和解决方案

## 参考资源

### GitHub Actions 官方文档

- [upload-artifact@v4](https://github.com/actions/upload-artifact)
- [download-artifact@v4](https://github.com/actions/download-artifact)
- [softprops/action-gh-release@v1](https://github.com/softprops/action-gh-release)

### 关键配置选项

**upload-artifact**:
- `name`: artifact 命名空间（关键配置）
- `path`: 要上传的文件路径
- `if-no-files-found`: 文件不存在时的行为

**download-artifact**:
- `pattern`: 匹配 artifact 名称的模式
- `merge-multiple`: 是否合并多个 artifacts 到同一目录
- `path`: 下载目标目录

**action-gh-release**:
- `files`: 要上传的文件路径（支持通配符）
- `fail_on_unmatched_files`: 文件不匹配时是否失败

## 验证方法

### 测试步骤

1. **创建测试 tag**:
   ```bash
   git tag v0.9.0-test
   git push origin v0.9.0-test
   ```

2. **监控 Actions 运行**:
   - 访问 GitHub Actions 页面
   - 查看 workflow 运行日志
   - 验证每个步骤成功执行

3. **验证 Release**:
   - 访问 Releases 页面
   - 确认 v0.9.0-test release 已创建
   - 验证所有 5 个平台的文件存在
   - 下载并验证文件完整性

4. **清理测试 tag**:
   ```bash
   git tag -d v0.9.0-test
   git push origin :refs/tags/v0.9.0-test
   ```

### 成功标准

- ✅ 所有 5 个平台的二进制文件在 Release Downloads 中可见
- ✅ checksums.txt 包含所有文件的 SHA256 哈希
- ✅ 下载的文件可以正常解压和执行
- ✅ `sha256sum -c checksums.txt` 验证通过

## 风险和缓解措施

### 风险 1: 文件名冲突

**场景**: 多个平台生成相同文件名
**缓解**: 文件名已包含完整平台信息（如 `work-linux-x86_64.tar.gz`），无冲突风险

### 风险 2: Artifacts 下载失败

**场景**: 某个 artifact 下载失败
**缓解**:
- 使用 `if-no-files-found: error` 确保失败时停止
- 添加验证步骤检查文件完整性

### 风险 3: Release 创建失败

**场景**: softprops/action-gh-release 无法找到文件
**缓解**:
- 使用递归通配符 `**/*.tar.gz`
- 添加 `fail_on_unmatched_files: true`
- 提前验证文件结构

## 后续优化建议

1. **添加并行下载**: 使用 `matrix` 优化 artifacts 下载速度
2. **添加重试机制**: 使用 `actions/upload-artifact@v4` 的重试功能
3. **添加签名**: 使用 GPG 签名二进制文件
4. **自动更新文档**: 从 release 自动生成 CHANGELOG
