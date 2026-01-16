# Integration Test Scenarios: Worktree Branch Source Selection

**Feature**: 005-worktree-branch-source
**Date**: 2025-01-16
**Purpose**: 定义集成测试场景和验收测试用例

## 概述

本文档定义了完整的集成测试场景，覆盖所有用户故事和边界情况。每个场景都包含前置条件、操作步骤和预期结果。

## 测试环境设置

### 基础仓库结构

```bash
# 创建测试仓库
test-repo/
├── .git/              # 主仓库
├── main-file.txt      # 主分支文件
└── .worktrees/        # worktree 目录（自动创建）
    ├── feature-a/     # 已存在的 worktree
    └── feature-b/     # 已存在的 worktree
```

### 初始分支状态

```bash
# 本地分支
main (当前)
develop
feature-a
feature-b

# 远程分支
origin/main
origin/develop
origin/feature-remote
```

## 测试场景

### 场景 1: 基于当前目录分支创建 Worktree (P1)

#### 1.1 在主目录中基于当前分支创建

**前置条件**:
- 当前位于主目录 `/test-repo`
- 当前分支是 `main`
- 无未提交更改

**操作步骤**:
```bash
cd /test-repo
git checkout main
work create feature-new --interactive
# 选择: "基于当前目录分支"
```

**预期结果**:
```text
✓ 显示交互式菜单，3 个选项
✓ 默认选中第一项（基于当前分支）
✓ 用户选择后，创建 worktree
✓ 输出: "Created worktree feature-new from branch main"
✓ worktree 路径: /test-repo.worktrees/feature-new
✓ 新 worktree 的分支是 main
✓ 当前目录仍在 /test-repo，未切换
```

**验收标准**:
- [ ] 交互式菜单正确显示
- [ ] 新 worktree 基于当前分支创建
- [ ] 成功消息包含分支名
- [ ] 路径提示正确

---

#### 1.2 在 worktree 中基于当前分支创建

**前置条件**:
- 当前位于 worktree `/test-repo.worktrees/feature-a`
- feature-a 分支上有未提交的更改（如 `test.txt` 已修改但未提交）
- feature-a 当前分支是 `feature-a`

**操作步骤**:
```bash
cd /test-repo.worktrees/feature-a
echo "modified" > test.txt  # 未提交的更改
work create feature-fix --interactive
# 选择: "基于当前目录分支"
```

**预期结果**:
```text
✓ 显示交互式菜单
✓ 显示当前分支名称 "feature-a"
✓ 创建成功，基于 feature-a 分支
✓ 当前 worktree 的未提交更改不受影响
✓ test.txt 仍然有未提交的更改
```

**验收标准**:
- [ ] 正确检测当前分支（feature-a）
- [ ] 新 worktree 基于当前分支创建
- [ ] 当前工作目录的更改不受影响
- [ ] 未提交的更改保留

---

### 场景 2: 基于主目录分支创建 Worktree (P2)

#### 2.1 从 worktree 基于主目录分支创建

**前置条件**:
- 当前位于 worktree `/test-repo.worktrees/feature-a`
- 当前分支是 `feature-a`
- 主目录当前分支是 `develop`

**操作步骤**:
```bash
cd /test-repo.worktrees/feature-a
git checkout feature-a
cd /test-repo
git checkout develop
cd /test-repo.worktrees/feature-a  # 回到 worktree

work create feature-b --interactive
# 选择: "基于主目录分支"
```

**预期结果**:
```text
✓ 显示交互式菜单
✓ 主目录分支显示为 "develop"
✓ 创建成功，基于 develop 分支（而非 feature-a）
✓ 新 worktree 路径: /test-repo.worktrees/feature-b
✓ 当前目录仍在 feature-a worktree
```

**验收标准**:
- [ ] 正确识别主目录分支（develop）
- [ ] 新 worktree 基于主目录分支创建
- [ ] 不基于当前 worktree 分支（feature-a）
- [ ] 成功消息正确

---

#### 2.2 从主目录选择"基于主目录分支"

**前置条件**:
- 当前位于主目录 `/test-repo`
- 主目录当前分支是 `main`

**操作步骤**:
```bash
cd /test-repo
git checkout main
work create feature-x --interactive
# 选择: "基于主目录分支"
```

**预期结果**:
```text
✓ 行为等同于"基于当前分支"
✓ 创建成功，基于 main 分支
✓ 用户友好提示（可选）：提示"与当前分支相同"
```

**验收标准**:
- [ ] 在主目录时，两种选项行为一致
- [ ] 不会产生混淆
- [ ] 创建成功

---

### 场景 3: 基于自定义分支创建 Worktree (P3)

#### 3.1 输入本地分支名称

**前置条件**:
- 存在本地分支 `develop`
- 当前位于任何目录

**操作步骤**:
```bash
cd /test-repo.worktrees/feature-a
work create feature-custom --interactive
# 选择: "自定义输入分支"
# 输入: develop
```

**预期结果**:
```text
✓ 显示输入提示: "输入分支名称"
✓ 用户输入: develop
✓ 验证分支存在（本地）
✓ 创建成功，基于 develop 分支
✓ 输出: "Created worktree feature-custom from branch develop"
```

**验收标准**:
- [ ] 输入提示清晰
- [ ] 接受有效的本地分支名
- [ ] 验证逻辑正确
- [ ] 创建成功

---

#### 3.2 输入远程分支名称

**前置条件**:
- 存在远程分支 `origin/feature-remote`
- 本地不存在 `feature-remote` 分支

**操作步骤**:
```bash
work create feature-local --interactive
# 选择: "自定义输入分支"
# 输入: origin/feature-remote
```

**预期结果**:
```text
✓ 显示输入提示
✓ 用户输入: origin/feature-remote
✓ 验证远程分支存在
✓ 创建成功
✓ 自动创建本地分支 feature-remote
✓ 自动设置上游为 origin/feature-remote
✓ 输出: "Created worktree feature-local from branch origin/feature-remote"
```

**验收标准**:
- [ ] 接受远程分支格式
- [ ] 自动创建本地分支
- [ ] 自动设置跟踪关系
- [ ] 成功消息显示远程分支名

---

#### 3.3 输入不存在的分支名称

**前置条件**:
- 不存在分支 `non-existent`

**操作步骤**:
```bash
work create feature-test --interactive
# 选择: "自定义输入分支"
# 输入: non-existent
```

**预期结果**:
```text
✓ 显示输入提示
✓ 用户输入: non-existent
✓ 验证失败（本地和远程都不存在）
✓ 错误消息:
  错误: 分支不存在

  您输入的分支 'non-existent' 在本地和远程都不存在

  可用的本地分支:
    develop
    feature-a
    feature-b
    main

  可用的远程分支:
    origin/develop
    origin/feature-remote
    origin/main

✓ 不创建 worktree
✓ 退出码非 0
```

**验收标准**:
- [ ] 清晰的错误提示
- [ ] 显示可用的分支列表
- [ ** 不创建 worktree
- [ ] 命令返回错误

---

### 场景 4: Detached HEAD 处理

#### 4.1 主目录处于 detached HEAD

**前置条件**:
- 主目录处于 detached HEAD 状态
- 指向 commit `abc1234`

**操作步骤**:
```bash
cd /test-repo
git checkout abc1234  # 进入 detached HEAD
work create feature-test --interactive
# 选择: "基于主目录分支"
```

**预期结果**:
```text
✓ 错误消息:
  错误: 主目录处于 detached HEAD 状态，无法作为分支来源

  主目录路径: /test-repo
  当前 HEAD: abc1234

  原因: detached HEAD 状态没有分支名称，无法作为分支来源

  建议的解决方案:
  1. 切换到一个分支: cd /test-repo && git checkout <branch>
  2. 选择其他分支来源（当前分支、自定义分支）
  3. 先在主目录创建或切换分支，然后重试

✓ 不创建 worktree
✓ 退出码非 0
```

**验收标准**:
- [ ] 检测到 detached HEAD
- [ ] 显示当前 commit SHA
- [ ] 提供清晰的解决建议
- [ ] 不创建 worktree

---

#### 4.2 当前目录处于 detached HEAD

**前置条件**:
- 当前 worktree 处于 detached HEAD 状态

**操作步骤**:
```bash
cd /test-repo.worktrees/feature-a
git checkout abc1234  # 进入 detached HEAD
work create feature-test --interactive
# 选择: "基于当前目录分支"
```

**预期结果**:
```text
✓ 错误消息类似于主目录 detached HEAD
✓ 提示当前路径和 commit SHA
✓ 建议切换分支或选择其他来源
✓ 不创建 worktree
```

**验收标准**:
- [ ] 检测到 detached HEAD
- [ ] 显示友好的错误信息
- [ ] 提供解决建议

---

### 场景 5: 边界情况

#### 5.1 空分支名称输入

**操作步骤**:
```bash
work create feature-test --interactive
# 选择: "自定义输入分支"
# 输入: (直接按 Enter，空字符串)
```

**预期结果**:
```text
✓ 输入验证失败，提示重新输入
✓ 或: 错误消息 "分支名称不能为空"
✓ 不创建 worktree
```

**验收标准**:
- [ ] 拒绝空输入
- [ ] 清晰的错误提示

---

#### 5.2 分支名称包含特殊字符

**操作步骤**:
```bash
work create feature-test --interactive
# 选择: "自定义输入分支"
# 输入: feature~test
```

**预期结果**:
```text
✓ Git 命令会失败（Git 不允许某些特殊字符）
✓ 捕获错误并显示友好消息
✓ 提示有效的分支名规则
✓ 不创建 worktree
```

**验收标准**:
- [ ] 捕获 Git 错误
- [ ] 显示友好的错误信息

---

#### 5.3 worktree 名称冲突

**前置条件**:
- 已存在 worktree `feature-a`

**操作步骤**:
```bash
work create feature-a --interactive
# 选择: "基于当前分支"
```

**预期结果**:
```text
✓ 错误消息 (现有行为):
  Error: Worktree 'feature-a' already exists

✓ 不创建 worktree
```

**验收标准**:
- [ ] 检测到冲突
- [ ] 使用现有的冲突处理逻辑

---

### 场景 6: 性能测试

#### 6.1 响应时间测试

**目标**:
- 交互式菜单显示: < 100ms
- 分支验证: < 500ms
- 完整创建流程: < 2 秒

**测试方法**:
```bash
# 使用 time 命令测量
time work create feature-perf --interactive
# 选择: "基于当前分支"
```

**预期结果**:
```text
✓ real time < 2s
✓ user time + sys time < 1.5s
✓ 用户体验流畅，无明显卡顿
```

**验收标准**:
- [ ] 满足性能目标
- [ ] 响应及时

---

## 命令行参数测试

### 测试 7.1: 新增 `--branch-source` 参数（P2，可选）

**操作步骤**:
```bash
# 直接指定分支来源，跳过交互
work create feature-x --branch-source main
```

**预期结果**:
```text
✓ 直接创建，无需交互
✓ 基于主目录分支创建
✓ 行为等同于交互式选择
```

**验收标准**:
- [ ] 参数解析正确
- [ ] 跳过交互式菜单
- [ ] 创建成功

---

### 测试 7.2: 向后兼容性

**操作步骤**:
```bash
# 旧版参数应继续工作
work create feature-y --branch develop
```

**预期结果**:
```text
✓ 使用现有逻辑创建
✓ 基于 develop 分支创建
✓ 无交互式菜单
✓ 行为与之前版本一致
```

**验收标准**:
- [ ] `--branch` 参数继续工作
- [ ] 不显示分支来源菜单
- [ ] 向后兼容

---

## 自动化测试实现

### 单元测试

**文件**: `src/core/git_ops.rs` (在 `#[cfg(test)]` 模块中)

```rust
#[test]
fn test_get_main_repo_branch() {
    // 测试获取主目录分支
    let (path, branch) = get_main_repo_branch().unwrap();
    assert!(path.exists());
    assert!(!branch.is_empty());
}

#[test]
fn test_branch_source_from_str() {
    // 测试字符串解析
    let source = BranchSource::from_str("current").unwrap();
    assert!(matches!(source, BranchSource::Current { .. }));
}

#[test]
fn test_branch_exists_remote() {
    // 测试远程分支验证
    assert!(branch_exists_remote("main"));
    assert!(!branch_exists_remote("non-existent"));
}
```

### 集成测试

**文件**: `tests/integration/branch_source_test.rs`

```rust
#[test]
fn test_create_from_current_branch() {
    // 创建测试仓库
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path();

    // 初始化 Git 仓库
    Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(repo_path)
        .output()
        .unwrap();

    // 配置用户信息
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .unwrap();

    // 创建初始提交
    std::fs::write(repo_path.join("test.txt"), "test").unwrap();
    Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(repo_path)
        .output()
        .unwrap();

    Command::new("git")
        .args(["commit", "-m", "Initial"])
        .current_dir(repo_path)
        .output()
        .unwrap();

    // 执行 work create 命令
    let cmd = Command::new(cargo_bin("work"))
        .args(["create", "feature-test", "--branch-source", "current"])
        .current_dir(repo_path)
        .output()
        .unwrap();

    // 验证输出
    assert!(cmd.status.success());

    // 验证 worktree 创建成功
    let worktrees = list_worktrees().unwrap();
    assert!(worktrees.iter().any(|wt| wt.dirname == "feature-test"));
}
```

### 端到端测试

**文件**: `tests/e2e/create_workflow_test.rs`

```rust
#[test]
fn test_full_workflow_with_branch_selection() {
    // 测试完整的用户工作流程
    // 1. 创建仓库
    // 2. 创建多个分支
    // 3. 从不同来源创建 worktree
    // 4. 验证结果
}
```

## 测试覆盖率目标

- **单元测试**: 80%+ 代码覆盖率
- **集成测试**: 所有用户故事场景
- **端到端测试**: 主要使用场景路径

## 总结

本文档定义了 7 大类测试场景，涵盖：

1. ✅ 基于当前分支（P1）
2. ✅ 基于主目录分支（P2）
3. ✅ 自定义分支（P3）
4. ✅ Detached HEAD 处理
5. ✅ 边界情况
6. ✅ 性能测试
7. ✅ 命令行参数和向后兼容

每个场景都包含明确的前置条件、操作步骤和预期结果，可直接用于自动化测试实现。
