# Quickstart Guide: 分支名斜杠转换

**Feature**: 003-branch-slash-conversion
**Date**: 2026-01-12
**Purpose**: Integration test scenarios and validation steps

## Overview

本文档提供了分支名斜杠转换功能的集成测试场景，用于验证实现的正确性和完整性。

---

## Prerequisites

1. **Rust 工具链**: 1.75+ edition 2021
2. **Git**: 2.0+ (用于测试)
3. **测试仓库**: 一个 Git 仓库用于创建 worktree

```bash
# 检查 Rust 版本
rustc --version

# 检查 Git 版本
git --version

# 创建测试仓库
mkdir -p /tmp/work-test
cd /tmp/work-test
git init
git config user.email "test@example.com"
git config user.name "Test User"
echo "test" > README.md
git add README.md
git commit -m "Initial commit"
```

---

## Test Scenarios

### Scenario 1: 创建包含单个斜杠的分支 (US1-P1)

**Objective**: 验证系统正确将 `feat/feature-001` 转换为 `feat-feature-001` 目录。

**Steps**:

1. 创建测试分支：
   ```bash
   git checkout -b feat/feature-001
   ```

2. 使用 work CLI 创建 worktree：
   ```bash
   cd /tmp/work-test
   work add feat/feature-001
   ```

3. 验证目录创建：
   ```bash
   ls -la /tmp/work-test.worktrees/
   # 应该看到 feat-feature-001 目录
   ```

4. 验证分支正确：
   ```bash
   cd /tmp/work-test.worktrees/feat-feature-001
   git branch --show-current
   # 应该输出: feat/feature-001
   ```

**Expected Results**:
- ✅ 目录 `/tmp/work-test.worktrees/feat-feature-001` 存在
- ✅ Git 分支为 `feat/feature-001`（原始名称）
- ✅ 目录名为 `feat-feature-001`（斜杠转换为连字符）
- ✅ worktree 可以正常使用

**Success Criteria**:
- SC-001: 在 5 秒内成功创建 worktree
- SC-002: 100% 正确转换目录名，分支名保持不变

---

### Scenario 2: 创建包含多个斜杠的分支 (US1-P1)

**Objective**: 验证系统正确处理多级分支 `feature/auth/oauth`。

**Steps**:

1. 创建测试分支：
   ```bash
   cd /tmp/work-test
   git checkout -b feature/auth/oauth
   ```

2. 创建 worktree：
   ```bash
   work add feature/auth/oauth
   ```

3. 验证目录名：
   ```bash
   ls -la /tmp/work-test.worktrees/
   # 应该看到 feature-auth-oauth 目录
   ```

4. 验证分支名：
   ```bash
   cd /tmp/work-test.worktrees/feature-auth-oauth
   git branch --show-current
   # 应该输出: feature/auth/oauth
   ```

**Expected Results**:
- ✅ 目录名为 `feature-auth-oauth`（所有斜杠转换为连字符）
- ✅ Git 分支为 `feature/auth/oauth`（原始名称）

---

### Scenario 3: 创建无斜杠的分支 (US1-P1)

**Objective**: 验证系统不对无斜杠分支进行转换。

**Steps**:

1. 创建测试分支：
   ```bash
   cd /tmp/work-test
   git checkout -b main
   ```

2. 创建 worktree：
   ```bash
   work add main
   ```

3. 验证目录名：
   ```bash
   ls -la /tmp/work-test.worktrees/
   # 应该看到 main 目录
   ```

**Expected Results**:
- ✅ 目录名为 `main`（无转换）
- ✅ Git 分支为 `main`（与目录名相同）

---

### Scenario 4: 列出包含斜杠分支的 worktree (US2-P2)

**Objective**: 验证 `work list` 正确显示目录名和分支名的对应关系。

**Steps**:

1. 创建多个 worktree：
   ```bash
   cd /tmp/work-test
   work add feat/feature-001
   work add feature/auth/oauth
   work add main
   ```

2. 列出所有 worktree（compact 格式）：
   ```bash
   work list
   ```

3. 列出所有 worktree（table 格式）：
   ```bash
   work list -o table
   ```

4. 列出所有 worktree（JSON 格式）：
   ```bash
   work list -o json
   ```

**Expected Results**:

**Compact 格式输出**:
```
*⌂  worktree on main
  feat-feature-001 on feat/feature-001
  feature-auth-oauth on feature/auth/oauth
```

**Table 格式输出**:
```
┌─────────────────────┬──────────────────────┬─────────┬──────────┐
│ Directory           │ Branch               │ Status  │ Head     │
├─────────────────────┼──────────────────────┼─────────┼──────────┤
│ ⌂ worktree          │ main                 │ clean   │ abc1234  │
│ feat-feature-001    │ feat/feature-001     │ clean   │ def5678  │
│ feature-auth-oauth  │ feature/auth/oauth   │ clean   │ ghi9012  │
└─────────────────────┴──────────────────────┴─────────┴──────────┘
```

**JSON 格式输出**:
```json
[
  {
    "directory": "feat-feature-001",
    "branch": "feat/feature-001",
    "status": "clean",
    "commit": "def5678"
  },
  {
    "directory": "feature-auth-oauth",
    "branch": "feature/auth/oauth",
    "status": "clean",
    "commit": "ghi9012"
  }
]
```

**Success Criteria**:
- SC-003: 100% 清晰显示目录名和分支名的对应关系

---

### Scenario 5: 显示包含斜杠分支的 worktree 详情 (US3-P3)

**Objective**: 验证 `work show` 正确显示目录名和分支名信息。

**Steps**:

1. 创建 worktree：
   ```bash
   cd /tmp/work-test
   work add feat/feature-001
   ```

2. 显示 worktree 详情：
   ```bash
   work show feat-feature-001
   ```

3. 显示 worktree 详情（JSON 格式）：
   ```bash
   work show feat-feature-001 -o json
   ```

**Expected Results**:

**Human 格式输出**:
```
Worktree: feat-feature-001
Branch: feat/feature-001
Path: /tmp/work-test.worktrees/feat-feature-001
Status: clean
HEAD: def5678 Add new feature (2 hours ago)
Is Main: No
Is Current: No
```

**JSON 格式输出**:
```json
{
  "directory": "feat-feature-001",
  "branch": "feat/feature-001",
  "path": "/tmp/work-test.worktrees/feat-feature-001",
  "status": "clean",
  "commit": "def5678",
  "is_main": false,
  "is_current": false
}
```

---

### Scenario 6: 目录名冲突检测 (FR-008)

**Objective**: 验证系统检测并报告目录名冲突。

**Steps**:

1. 创建第一个 worktree：
   ```bash
   cd /tmp/work-test
   work add feat/feature-001
   ```

2. 尝试创建冲突的 worktree：
   ```bash
   work add feat/feature-001
   ```

**Expected Results**:

**错误输出**:
```
Error: Cannot create worktree - directory name conflict

The branch 'feat/feature-001' would create directory 'feat-feature-001',
which conflicts with existing worktree for branch 'feat/feature-001'.

Suggested solutions:
  1. Use a different branch name
  2. Delete the existing worktree with: work delete feat-feature-001
```

**Success Criteria**:
- SC-004: 100% 检测到命名冲突并在创建前报告

---

### Scenario 7: 删除包含斜杠分支的 worktree

**Objective**: 验证可以使用目录名删除 worktree。

**Steps**:

1. 创建 worktree：
   ```bash
   cd /tmp/work-test
   work add feat/feature-001
   ```

2. 删除 worktree：
   ```bash
   work delete feat-feature-001
   ```

3. 验证删除：
   ```bash
   ls -la /tmp/work-test.worktrees/
   # 不应该看到 feat-feature-001 目录
   ```

**Expected Results**:
- ✅ worktree 成功删除
- ✅ 目录 `/tmp/work-test.worktrees/feat-feature-001` 不存在
- ✅ Git worktree 元数据已清理

---

### Scenario 8: 边界情况 - 连续多个斜杠 (Edge Case)

**Objective**: 验证系统处理 `feat///feature` 分支。

**Steps**:

1. 创建测试分支（如果 Git 允许）：
   ```bash
   cd /tmp/work-test
   git branch feat///feature
   ```

2. 尝试创建 worktree：
   ```bash
   work add feat///feature
   ```

3. 验证目录名：
   ```bash
   ls -la /tmp/work-test.worktrees/
   # 应该看到 feat---feature 目录（所有斜杠转换为连字符）
   ```

**Expected Results**:
- ✅ 目录名为 `feat---feature`（3 个连字符）
- ✅ Git 分支为 `feat///feature`（如果允许）

**Note**: Git 可能拒绝此分支名（取决于版本），这是预期行为。

---

### Scenario 9: 边界情况 - 超长分支名 (Edge Case)

**Objective**: 验证系统处理超长分支名。

**Steps**:

1. 创建超长分支名：
   ```bash
   cd /tmp/work-test
   git branch feat/$(python3 -c "print('a'*1000)")
   ```

2. 尝试创建 worktree：
   ```bash
   work add feat/$(python3 -c "print('a'*1000)")
   ```

**Expected Results**:

**情况 A**: 路径过长导致失败
```
Error: Cannot create worktree - path too long

The worktree path '/tmp/work-test.worktrees/feat-aaa...' exceeds the
maximum path length allowed by the filesystem.

Suggested solutions:
  1. Use a shorter branch name
  2. Create the worktree in a location with a shorter base path
```

**情况 B**: 成功创建（如果路径长度在限制内）

---

## Validation Checklist

### Functional Requirements

- [ ] FR-001: 系统检测分支名中的斜杠
- [ ] FR-002: 系统将所有 `/` 转换为 `-`
- [ ] FR-003: 系统保持原始分支名不变
- [ ] FR-004: 无斜杠分支名不进行转换
- [ ] FR-005: 列出 worktree 时显示目录名和分支名
- [ ] FR-006: 目录名符合文件系统规范
- [ ] FR-007: 显示详情时提供目录名和分支名
- [ ] FR-008: 检测并报告目录名冲突
- [ ] FR-009: 支持多个连续斜杠
- [ ] FR-010: 处理斜杠在开头或结尾

### Success Criteria

- [ ] SC-001: 在 5 秒内成功创建 worktree
- [ ] SC-002: 100% 正确转换目录名
- [ ] SC-003: 100% 清晰显示目录名和分支名
- [ ] SC-004: 100% 检测到命名冲突
- [ ] SC-005: 解决 90% 以上的分支命名问题

### Edge Cases

- [ ] 空分支名被拒绝
- [ ] 仅斜杠的分支名被拒绝
- [ ] 斜杠在开头/结尾被正确处理
- [ ] 连续多个斜杠被正确处理
- [ ] 超长分支名返回清晰的错误
- [ ] 不同操作系统的路径差异被正确处理

---

## Performance Testing

### Benchmark: 创建 worktree

```bash
# 测试创建 100 个 worktree 的性能
time for i in {1..100}; do
    work add feat/feature-$i
done

# 预期: < 100 秒（平均 < 1 秒每个）
```

### Benchmark: 列出 worktree

```bash
# 创建 100 个 worktree
for i in {1..100}; do
    work add feat/feature-$i
done

# 测试列出性能
time work list

# 预期: < 100ms
```

---

## Manual Testing Guide

### 1. 基本功能测试

```bash
# 测试单个斜杠
work add feat/feature-001
work list | grep "feat-feature-001 on feat/feature-001"

# 测试多个斜杠
work add feature/auth/oauth
work list | grep "feature-auth-oauth on feature/auth/oauth"

# 测试无斜杠
work add main
work list | grep "main"
```

### 2. 冲突检测测试

```bash
# 创建 worktree
work add feat/feature-001

# 尝试创建冲突的 worktree（应该失败）
work add feat/feature-001 2>&1 | grep "directory name conflict"
```

### 3. 输出格式测试

```bash
# Compact 格式
work list

# Table 格式
work list -o table

# JSON 格式
work list -o json | jq '.[] | select(.branch | contains("/"))'
```

### 4. 详情显示测试

```bash
# Human 格式
work show feat-feature-001

# JSON 格式
work show feat-feature-001 -o json | jq '.branch'
```

---

## Automation Script

```bash
#!/bin/bash
# tests/integration/slash_conversion.sh

set -e

echo "=== Testing Branch Slash Conversion ==="

# Setup
TEST_REPO="/tmp/work-test-$$"
mkdir -p "$TEST_REPO"
cd "$TEST_REPO"
git init
git config user.email "test@example.com"
git config user.name "Test User"
echo "test" > README.md
git add README.md
git commit -m "Initial commit"

# Test 1: Single slash
echo "Test 1: Single slash"
git checkout -b feat/feature-001
work add feat/feature-001
if [ -d "$TEST_REPO.worktrees/feat-feature-001" ]; then
    echo "✅ Directory created: feat-feature-001"
else
    echo "❌ Directory not created"
    exit 1
fi

# Test 2: Multiple slashes
echo "Test 2: Multiple slashes"
git checkout -b feature/auth/oauth
work add feature/auth/oauth
if [ -d "$TEST_REPO.worktrees/feature-auth-oauth" ]; then
    echo "✅ Directory created: feature-auth-oauth"
else
    echo "❌ Directory not created"
    exit 1
fi

# Test 3: No slash
echo "Test 3: No slash"
git checkout -b main
work add main
if [ -d "$TEST_REPO.worktrees/main" ]; then
    echo "✅ Directory created: main"
else
    echo "❌ Directory not created"
    exit 1
fi

# Test 4: List output
echo "Test 4: List output"
OUTPUT=$(work list)
if echo "$OUTPUT" | grep -q "feat-feature-001 on feat/feature-001"; then
    echo "✅ Compact format shows directory and branch"
else
    echo "❌ Compact format incorrect"
    exit 1
fi

# Test 5: Conflict detection
echo "Test 5: Conflict detection"
if work add feat/feature-001 2>&1 | grep -q "directory name conflict"; then
    echo "✅ Conflict detected"
else
    echo "❌ Conflict not detected"
    exit 1
fi

# Cleanup
cd /
rm -rf "$TEST_REPO"

echo "=== All Tests Passed ==="
```

---

### Scenario 10: 主目录路径显示 (US2-P2 增强功能)

**Objective**: 验证 `work list` 在 Compact 格式下显示主目录完整路径。

**用户需求**: "work list 需要输出主目录的完整路径"

**Steps**:

1. 在主仓库目录创建分支：
   ```bash
   cd /tmp/work-test
   git checkout -b feat/feature-001
   ```

2. 列出 worktree（默认 Compact 格式）：
   ```bash
   work list
   ```

3. 验证主目录显示路径：
   ```bash
   work list | grep "⌂" | grep "at /tmp/work-test"
   ```

**Expected Output** (Compact 格式):
```text
*⌂  worktree on feat/feature-001 (modified) at /tmp/work-test
  feat-feature-001 on feat/feature-001 at /tmp/work-test.worktrees/feat-feature-001
```

**Expected Results**:
- ✅ 主目录行包含 " at /tmp/work-test" 后缀（灰色显示）
- ✅ worktree 目录行不包含路径（避免信息过载）
- ✅ 路径使用完整绝对路径格式
- ✅ 路径颜色为灰色（dimmed），不干扰主要信息

**Success Criteria**:
- SC-003: 100% 清晰显示主目录路径
- 用户能在 3 秒内定位主仓库位置

**Note**: Table 格式已包含 PATH 列，无需验证。

---

## Validation Checklist (Updated)

### Functional Requirements

- [ ] FR-001: 系统检测分支名中的斜杠
- [ ] FR-002: 系统将所有 `/` 转换为 `-`
- [ ] FR-003: 系统保持原始分支名不变
- [ ] FR-004: 无斜杠分支名不进行转换
- [ ] FR-005: 列出 worktree 时显示目录名和分支名
- [ ] FR-006: 目录名符合文件系统规范
- [ ] FR-007: 显示详情时提供目录名和分支名
- [ ] FR-008: 检测并报告目录名冲突
- [ ] FR-009: 支持多个连续斜杠
- [ ] FR-010: 处理斜杠在开头或结尾
- [ ] **FR-011 (NEW)**: Compact 格式显示主目录完整路径

### Success Criteria

- [ ] SC-001: 在 5 秒内成功创建 worktree
- [ ] SC-002: 100% 正确转换目录名
- [ ] SC-003: 100% 清晰显示目录名和分支名
- [ ] SC-004: 100% 检测到命名冲突
- [ ] SC-005: 解决 90% 以上的分支命名问题
- [ ] **SC-006 (NEW)**: 主目录路径在 Compact 格式中 100% 显示

---

## Manual Testing Guide (Updated)

### 4. 详情显示测试

```bash
# Human 格式
work show feat-feature-001

# JSON 格式
work show feat-feature-001 -o json | jq '.branch'
```

### 5. 路径显示测试 (NEW)

```bash
# 测试主目录路径显示（Compact 格式）
work list | grep "⌂" | grep " at "

# 测试路径颜色（应该为灰色）
# 注意：颜色代码需要在终端中手动验证

# 测试 Table 格式路径（PATH 列）
work list -o table

# 测试 JSON 格式路径（path 字段）
work list -o json | jq '.[] | select(.is_main == true) | .path'
```

---

## Automation Script (Updated)

```bash
#!/bin/bash
# tests/integration/slash_conversion.sh

set -e

echo "=== Testing Branch Slash Conversion ==="

# Setup
TEST_REPO="/tmp/work-test-$$"
mkdir -p "$TEST_REPO"
cd "$TEST_REPO"
git init
git config user.email "test@example.com"
git config user.name "Test User"
echo "test" > README.md
git add README.md
git commit -m "Initial commit"

# Test 1: Single slash
echo "Test 1: Single slash"
git checkout -b feat/feature-001
work add feat/feature-001
if [ -d "$TEST_REPO.worktrees/feat-feature-001" ]; then
    echo "✅ Directory created: feat-feature-001"
else
    echo "❌ Directory not created"
    exit 1
fi

# Test 2: Multiple slashes
echo "Test 2: Multiple slashes"
git checkout -b feature/auth/oauth
work add feature/auth/oauth
if [ -d "$TEST_REPO.worktrees/feature-auth-oauth" ]; then
    echo "✅ Directory created: feature-auth-oauth"
else
    echo "❌ Directory not created"
    exit 1
fi

# Test 3: No slash
echo "Test 3: No slash"
git checkout -b main
work add main
if [ -d "$TEST_REPO.worktrees/main" ]; then
    echo "✅ Directory created: main"
else
    echo "❌ Directory not created"
    exit 1
fi

# Test 4: List output
echo "Test 4: List output"
OUTPUT=$(work list)
if echo "$OUTPUT" | grep -q "feat-feature-001 on feat/feature-001"; then
    echo "✅ Compact format shows directory and branch"
else
    echo "❌ Compact format incorrect"
    exit 1
fi

# Test 5: Conflict detection
echo "Test 5: Conflict detection"
if work add feat/feature-001 2>&1 | grep -q "directory name conflict"; then
    echo "✅ Conflict detected"
else
    echo "❌ Conflict not detected"
    exit 1
fi

# Test 6: Main directory path display (NEW)
echo "Test 6: Main directory path display"
OUTPUT=$(work list)
if echo "$OUTPUT" | grep "⌂" | grep -q " at $TEST_REPO"; then
    echo "✅ Main directory path displayed"
else
    echo "❌ Main directory path not displayed"
    echo "Output: $OUTPUT"
    exit 1
fi

# Cleanup
cd /
rm -rf "$TEST_REPO"

echo "=== All Tests Passed ==="
```

---

## Conclusion

本文档提供了完整的集成测试场景，覆盖所有用户故事、功能需求和边界情况，包括新增的主目录路径显示功能。实现完成后，应运行所有测试场景以验证功能的正确性。

**Next Steps**:
1. ✅ 规划完成（plan.md + research.md 已更新）
2. 运行 `/speckit.tasks` 生成任务列表
3. 实现功能（包括路径显示增强）
4. 运行本文档中的所有测试场景
5. 验证所有成功标准
