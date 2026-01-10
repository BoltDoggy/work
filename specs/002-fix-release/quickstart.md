# Quickstart: 修复 GitHub Actions Release Workflow

**Feature**: 002-fix-release
**Last Updated**: 2026-01-10

## 快速验证

### 前置条件

- ✅ 有 GitHub 仓库写权限
- ✅ 本地已配置 git 和 GitHub SSH/HTTPS 认证
- ✅ 分支 `002-fix-release` 已创建并检出

### 步骤 1: 应用修复

修改 `.github/workflows/release.yml` 文件：

#### 修改 1: Upload-Artifact 步骤（第 91-97 行）

**当前代码**:
```yaml
- name: Upload tarball/zip
  uses: actions/upload-artifact@v4
  with:
    name: ${{ matrix.asset_name }}
    path: |
      ${{ matrix.asset_name }}.*
```

**修改为**:
```yaml
- name: Upload tarball/zip
  uses: actions/upload-artifact@v4
  with:
    name: release-artifacts
    path: |
      ${{ matrix.asset_name }}.*
    if-no-files-found: error
```

#### 修改 2: Download-Artifact 步骤（第 107-110 行）

**当前代码**:
```yaml
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    path: artifacts
```

**修改为**:
```yaml
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    pattern: release-artifacts
    path: artifacts
    merge-multiple: true
```

#### 修改 3: Release 步骤（第 120-121 行）

**当前代码**:
```yaml
files: |
  artifacts/*.*
```

**修改为**:
```yaml
files: |
  artifacts/**/*.tar.gz
  artifacts/**/*.zip
  artifacts/checksums.txt
```

### 步骤 2: 提交修复

```bash
git add .github/workflows/release.yml
git commit -m "fix: 修复 GitHub Actions release workflow artifacts 上传问题

- 统一 artifact 命名为 release-artifacts
- 添加 merge-multiple: true 展平目录结构
- 使用递归通配符 **/*.tar.gz 匹配文件
- 添加 if-no-files-found: error 确保文件存在"

git push origin 002-fix-release
```

### 步骤 3: 测试验证

#### 创建测试 tag

```bash
# 切换到 main 分支
git checkout main

# 创建测试 tag
git tag v0.9.0-test

# 推送 tag
git push origin v0.9.0-test
```

#### 验证 Actions 运行

1. 访问 GitHub Actions 页面
2. 找到 "Release" workflow 运行
3. 检查每个步骤的状态：
   - ✅ Build binary (5 个平台)
   - ✅ Upload tarball/zip
   - ✅ Download all artifacts
   - ✅ Create checksums
   - ✅ Create Release

#### 验证 Release

1. 访问 GitHub Releases 页面
2. 找到 `v0.9.0-test` release
3. 确认以下文件存在：
   - ✅ `work-linux-x86_64.tar.gz`
   - ✅ `work-linux-aarch64.tar.gz`
   - ✅ `work-macos-x86_64.tar.gz`
   - ✅ `work-macos-aarch64.tar.gz`
   - ✅ `work-windows-x86_64.zip`
   - ✅ `checksums.txt`

#### 验证文件完整性

```bash
# 下载任意平台的 tar.gz 文件
wget https://github.com/BoltDoggy/work/releases/download/v0.9.0-test/work-linux-x86_64.tar.gz

# 下载 checksums.txt
wget https://github.com/BoltDoggy/work/releases/download/v0.9.0-test/checksums.txt

# 验证哈希
sha256sum -c checksums.txt
```

预期输出：
```
work-linux-x86_64.tar.gz: OK
```

### 步骤 4: 清理测试 tag

```bash
# 删除本地 tag
git tag -d v0.9.0-test

# 删除远程 tag
git push origin :refs/tags/v0.9.0-test

# 删除 GitHub Release（通过网页或 GitHub CLI）
gh release delete v0.9.0-test --yes
```

## 故障排除

### 问题 1: Artifacts 下载后文件结构不对

**症状**: `ls -R artifacts` 显示文件仍在子目录中

**原因**: `merge-multiple: true` 未生效

**解决方案**:
1. 确认 download-artifact 使用了 `pattern: release-artifacts`
2. 确认所有 upload-artifact 使用了统一名称 `release-artifacts`
3. 检查 workflow 日志中的下载步骤输出

### 问题 2: Release 无文件上传

**症状**: Release 页面创建成功，但 Downloads 区域为空

**原因**: 文件路径通配符不匹配

**解决方案**:
1. 添加调试步骤查看文件结构：
   ```yaml
   - name: Debug artifacts structure
     run: |
       echo "Artifacts structure:"
       ls -R artifacts
   ```
2. 确认使用了递归通配符 `**/*.tar.gz`
3. 检查 `softprops/action-gh-release` 的输出日志

### 问题 3: Checksums 验证失败

**症状**: `sha256sum -c checksums.txt` 报告 FAILED

**原因**:
- 文件在下载过程中损坏
- checksums.txt 生成时路径不正确

**解决方案**:
1. 重新下载文件并验证
2. 检查 checksums.txt 内容格式：
   ```
   <hash>  ./work-linux-x86_64.tar.gz
   ```
3. 确保在 artifacts 目录根目录生成 checksums

### 问题 4: 某个平台构建失败

**症状**: Actions 显示某个平台的 build 步骤失败

**原因**:
- 编译错误
- 依赖问题
- 交叉编译工具链问题

**解决方案**:
1. 检查失败步骤的详细日志
2. 查看编译错误信息
3. 参考修复方案（见 `specs/001-git-worktree-cli/`）

### 问题 5: Tag 推送后 workflow 未触发

**症状**: GitHub Actions 无运行记录

**原因**: Tag 名称不符合 `v*` 格式

**解决方案**:
1. 确认 tag 以 `v` 开头：`v0.1.0` 而非 `0.1.0`
2. 检查 workflow 触发条件：
   ```yaml
   on:
     push:
       tags:
         - 'v*'
   ```
3. 如果需要支持其他格式，修改触发条件

## 发布正式版本

### 预发布检查清单

- [ ] 所有测试通过（`cargo test`）
- [ ] 版本号已更新（`Cargo.toml` 和 `src/main.rs`）
- [ ] CHANGELOG.md 已更新
- [ ] 测试 tag 验证成功

### 创建正式 Release

```bash
# 确保在 main 分支
git checkout main
git pull origin main

# 创建版本 tag
git tag v0.2.0

# 推送 tag（触发 workflow）
git push origin v0.2.0 --tags
```

### 验证 Release

1. 等待 workflow 完成（约 10-15 分钟）
2. 访问 Releases 页面
3. 验证所有平台的文件存在
4. 下载并测试二进制文件

## 常见命令

### 查看 Actions 日志

```bash
# 使用 GitHub CLI
gh run list --workflow=release.yml

# 查看特定运行
gh run view <run-id>

# 查看实时日志
gh run watch
```

### 管理 Releases

```bash
# 列出所有 releases
gh release list

# 查看特定 release
gh release view v0.2.0

# 删除 release
gh release delete v0.2.0 --yes
```

### 本地测试 workflow

```bash
# 使用 act 本地运行 GitHub Actions
# 需要先安装: brew install act

act -j build --matrix target:x86_64-unknown-linux-gnu
act release
```

## 验证结果 (2026-01-10)

### ✅ 测试成功

**测试 tag**: v0.9.0-test
**测试日期**: 2026-01-10
**测试结果**: 全部通过 ✅

### 最终实施方案

**问题**: 原计划使用统一命名 `release-artifacts` 导致 409 冲突
**解决**: 使用带后缀的命名 + pattern 匹配

#### 修改 1: Upload-Artifact（实际实施方案）

```yaml
- name: Upload tarball/zip
  uses: actions/upload-artifact@v4
  with:
    name: release-artifacts-${{ matrix.asset_name }}  # 带后缀避免冲突
    path: |
      ${{ matrix.asset_name }}.*
    if-no-files-found: error
```

#### 修改 2: Download-Artifact（实际实施方案）

```yaml
- name: Download all artifacts
  uses: actions/download-artifact@v4
  with:
    pattern: release-artifacts-*  # 匹配所有平台
    path: artifacts
    merge-multiple: true          # 展平目录结构
```

#### 修改 3: Release Files（实际实施方案）

```yaml
files: |
  artifacts/**/*.tar.gz
  artifacts/**/*.zip
  artifacts/checksums.txt
fail_on_unmatched_files: true
```

### 验证结果

✅ **构建阶段**: 所有 5 个平台成功构建
✅ **上传阶段**: 无 409 冲突，所有 artifacts 成功上传
✅ **下载阶段**: merge-multiple 成功展平目录结构
✅ **Release 创建**: 所有 6 个文件成功上传到 Release Downloads
✅ **文件完整性**: SHA256 哈希验证通过

### Release 文件列表

1. work-linux-x86_64.tar.gz (935K)
2. work-linux-aarch64.tar.gz
3. work-macos-x86_64.tar.gz
4. work-macos-aarch64.tar.gz
5. work-windows-x86_64.zip
6. checksums.txt (466B)

### 关键经验

1. **避免 artifact 命名冲突**: 多个并行 jobs 不能上传同名 artifact
2. **使用 pattern 匹配**: `pattern: release-artifacts-*` + `merge-multiple: true` 是推荐方式
3. **递归通配符很重要**: `**/*.tar.gz` 能匹配嵌套目录中的文件
4. **验证步骤必不可少**: 添加 "Verify artifacts structure" 步骤便于调试

## 相关资源

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [upload-artifact@v4](https://github.com/actions/upload-artifact)
- [download-artifact@v4](https://github.com/actions/download-artifact)
- [action-gh-release@v1](https://github.com/softprops/action-gh-release)
- [GitHub CLI](https://cli.github.com/)
