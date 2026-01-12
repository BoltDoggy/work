<!--
Sync Impact Report
==================
Version change: 1.0.1 → 1.0.1 (no changes)
Modified principles: None
Added sections: None
Removed sections: None
Templates requiring updates:
  - ✅ .specify/templates/plan-template.md (Constitution Check section verified compatible)
  - ✅ .specify/templates/spec-template.md (no constitution dependencies, verified)
  - ✅ .specify/templates/tasks-template.md (no constitution dependencies, verified)
  - ✅ .claude/commands/speckit.specify.md (verified compliant with Principle I)
  - ✅ .claude/commands/speckit.plan.md (verified compliant with Principle III)
  - ✅ .claude/commands/speckit.tasks.md (verified compliant with Principle II)
  - ✅ .claude/commands/speckit.implement.md (verified compliant with Principle III)
  - ✅ CLAUDE.md (verified aligned with current constitution)
  - ✅ README.md (no constitution dependencies, verified)
Follow-up TODOs: None
Notes:
  - Routine consistency verification completed
  - All templates and commands remain aligned with constitution principles
  - No version bump required - this is a maintenance validation
  - All three core principles (规范优先开发, 独立可交付性, 质量前置) are properly reflected across all artifacts
-->

# SpecKit 宪章

## 核心原则

### I. 规范优先开发

**规则**：
- 所有功能开发必须从自然语言规范开始（`/speckit.specify`）
- 规范必须明确用户需求（WHAT）和原因（WHY），避免实现细节（HOW）
- 规范必须包含可测试的需求和可衡量成功标准
- 技术实现决策必须在规划阶段记录（`/speckit.plan`）

**理由**：规范优先开发确保团队对需求达成共识，减少返工，提高交付质量。通过分离"做什么"和"怎么做"，可以更好地评估技术方案并适应变化。

### II. 独立可交付性

**规则**：
- 每个用户故事必须独立可测试和可交付
- 用户故事按优先级排序（P1、P2、P3），确保P1可独立作为MVP交付
- 任务拆分必须反映用户故事边界，而非技术层次
- 避免跨用户故事的实现依赖，如不可避免必须明确记录

**理由**：独立可交付性支持增量开发，允许团队在任意里程碑暂停并交付价值。它还支持并行开发和更灵活的优先级调整。

### III. 质量前置

**规则**：
- 所有规范必须通过完整性验证才能进入规划阶段
- 所有"NEEDS CLARIFICATION"标记必须在规划前解决（最多3个）
- 规划必须通过宪章检查才能进入任务拆分
- 鼓励测试驱动开发（如需测试，测试必须先于实现编写并失败）

**理由**：质量前置在问题成本最低时发现和解决它们。晚期发现的需求缺陷会导致数十倍的返工成本。

## 开发标准

### 文档要求

- 所有功能必须有完整的规范文档（`spec.md`）
- 技术决策必须记录在研究文档（`research.md`）中并包含理由
- 数据模型和API契约必须作为设计工件（`data-model.md`、`contracts/`）维护
- 代码必须包含必要的内联注释解释复杂逻辑

### 代码风格

- 代码必须遵循所选技术栈的社区最佳实践
- 使用一致的命名约定和代码格式化工具
- 保持函数和模块的单一职责
- 优先选择清晰而非简洁的代码

### 版本控制

- 所有开发必须在编号的功能分支上进行（`###-feature-name`）
- 分支编号在本地分支、远程分支和规范目录之间全局递增
- 提交信息必须清晰描述变更内容和理由
- 功能完成后创建拉取请求/合并请求进行代码审查

## 质量保证

### 规范质量门禁

- 规范必须包含所有必需部分：用户场景、需求、成功标准
- 最多3个[NEEDS CLARIFICATION]标记
- 成功标准必须可衡量且与技术无关
- 需求必须明确且可测试

### 规划质量门禁

- 必须解决所有技术"NEEDS CLARIFICATION"项
- 必须通过宪章检查（违规必须有合理理由）
- 必须生成数据模型、API契约和快速入门指南

### 任务质量门禁

- 每个任务必须遵循严格的清单格式
- 任务必须按用户故事组织，支持独立实现
- 每个任务必须包含具体文件路径
- 必须识别并行执行机会

### 实现质量门禁

- 检查清单必须完成（如适用）
- 必须验证/创建忽略文件（.gitignore等）
- 必须遵守阶段依赖关系
- 如采用TDD，测试必须在实现前失败

## 治理

### 宪章修订流程

1. **提议修订**：通过`/speckit.constitution`命令或直接编辑`constitution.md`文件
2. **影响评估**：评估变更对现有规范、计划和任务的影响
3. **模板同步**：更新依赖模板以反映新原则或规则
4. **版本控制**：按语义版本规则递增版本号
5. **文档更新**：记录变更理由和影响范围

### 版本策略

- **MAJOR**（主版本）：向后不兼容的原则移除或重新定义
- **MINOR**（次版本）：新增原则或实质性扩展指导原则
- **PATCH**（补丁版本）：澄清、措辞改进、非语义细化

### 合规性审查

- 所有代码审查必须验证对宪章原则的遵守情况
- 违反宪章必须记录理由并得到批准
- 复杂度增加必须有正当理由（例如：性能、安全、合规要求）
- 定期审查规范、计划和任务的一致性

### 例外管理

- 例外情况必须记录在计划文档的"复杂度追踪"部分
- 例外必须说明：
  - 需要什么以及为什么
  - 被拒绝的更简单替代方案及其不适用原因
- 例外必须在实施前得到批准

### 运行时指导

- 项目运行时开发指导见`CLAUDE.md`（或项目的等效文件）
- 该文件包含具体的命令、工作流程和项目特定约定
- 违反宪章的例外必须反映在运行时指导中

---

**版本**: 1.0.1 | **批准日期**: 2026-01-10 | **最后修订**: 2026-01-10
