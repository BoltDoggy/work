# Technical Research: Git Worktree 管理工具

**Feature**: Git Worktree 管理工具
**Date**: 2026-01-10
**Phase**: Phase 0 - Research & Technology Decisions

## Overview

本文档记录 Git Worktree 管理工具的技术研究和决策。所有技术选择都基于功能需求、性能目标、开发效率和维护成本的综合考虑。

## Technology Decisions

### 1. CLI 框架选择

**决策**: 使用 clap 4.x

**理由**:
- clap 是 Rust 生态系统中最成熟和广泛使用的 CLI 框架
- 派生宏支持大大减少样板代码
- 优秀的错误处理和用户友好的帮助信息自动生成
- 原生支持子命令、参数验证和默认值
- 活跃的维护和庞大的社区支持

**替代方案**:
- **structopt**: 已被弃用，合并到 clap 3.x+ 中
- **argh**: 更简洁但功能较少，不支持复杂的参数组合
- **pico-args**: 极简主义，需要手动实现帮助和验证

**参考**: clap 官方文档 https://docs.rs/clap/latest/clap/

### 2. Git 操作库选择

**决策**: 使用 git2 0.18.x (libgit2 的 Rust 绑定)

**理由**:
- git2 是 Rust 中功能最完整的 Git 库
- 提供 worktree 操作的原生支持（`Repository::worktree()`）
- 跨平台一致性（Linux, macOS, Windows）
- 性能优秀，直接调用 libgit2 C 库
- 不需要外部 `git` 命令行工具依赖

**替代方案**:
- **gitoxide (gix)**: 纯 Rust 实现，性能更好但 API 较新且不稳定
- **直接调用 git CLI**: 简单但需要解析输出，跨平台兼容性差，性能较低

**权衡**: git2 的性能略低于 gitoxide，但 API 更稳定和成熟。对于 CLI 工具，开发效率和稳定性优先。

**参考**: git2 文档 https://docs.rs/git2/latest/git2/

### 3. 交互式选择库

**决策**: 使用 inquire 0.7.x

**理由**:
- 现代化的终端交互 UI，支持多选、单选、文本输入等
- 内置支持模糊搜索和键盘导航
- 跨平台终端兼容性（包括 Windows）
- 可定制主题和样式
- 活跃维护，文档完善

**替代方案**:
- **dialoguer**: 较老的项目，功能较基础
- **promptly**: 功能有限，不支持复杂交互
- **自实现**: 开发成本高，难以保证跨平台兼容性

**参考**: inquire 文档 https://docs.rs/inquire/latest/inquire/

### 4. 输出格式化

**决策**:
- 表格输出：comfy-table 0.7.x（人类可读）
- JSON 输出：serde_json 1.x（机器可解析）

**理由**:
- **comfy-table**: 支持对齐、边框、样式，终端友好
- **serde_json**: Rust 事实上的 JSON 序列化标准

**替代方案**:
- **table_formatter**: 功能较少
- **json**: Rust 内置 JSON 支持，API 较低级

### 5. 错误处理

**决策**: 使用 anyhow 1.x

**理由**:
- 提供错误上下文（`anyhow::Context` trait）
- 与 `Result<T>` 类型无缝集成
- 简化错误传播（`?` 操作符）
- 不需要定义大量错误类型

**替代方案**:
- **thiserror**: 适合库开发，需要为每种错误定义枚举
- **自定义错误类型**: 开发成本高，维护复杂

**权衡**: 对于 CLI 应用，anyhow 的简洁性优于 thiserror 的类型安全。

### 6. 日志记录

**决策**: 使用 env_logger 0.11.x

**理由**:
- 通过环境变量配置（`RUST_LOG=debug`）
- 支持日志级别过滤（error, warn, info, debug, trace）
- 标准库兼容（使用 `log` crate）
- 零配置即可使用

**替代方案**:
- **tracing**: 功能更强大但更复杂，适合异步和分布式系统
- **flexi_logger**: 配置更丰富但学习曲线陡峭

### 7. 测试策略

**决策**: 使用 cargo test + 集成测试 + Git 仓库 fixture

**理由**:
- **单元测试**: 测试核心逻辑（worktree 实体、路径处理）
- **集成测试**: 测试 Git 操作（使用临时 Git 仓库）
- **fixture 策略**: 为每个测试创建独立的临时 Git 仓库

**测试覆盖率目标**:
- 核心模块：> 80%
- CLI 层：> 60% (UI 逻辑难以测试)
- Git 操作：> 70% (依赖外部 Git)

### 8. 性能优化策略

**决策**:
- 延迟加载：仅在需要时查询 Git 状态
- 并行处理：使用 rayon 并行化 worktree 状态查询
- 缓存：避免重复的 Git 仓库打开操作

**权衡**: 对于 100+ worktree，并行化可显著提升性能（目标 < 2 秒）

## Best Practices

### Rust CLI 开发

1. **错误处理**: 使用 `anyhow` + `Result<T>` 模式
   ```rust
   pub type Result<T> = std::result::Result<T, anyhow::Error>;
   ```

2. **命令结构**: 使用 clap 的派生宏
   ```rust
   #[derive(Subcommand)]
   enum Commands {
       List,
       Switch { name: String },
       Create { branch: String },
       Delete { name: String },
   }
   ```

3. **输出格式**: 支持 `-o json` 参数
   ```rust
   #[arg(short = 'o', long = "output", default_value = "table")]
   output_format: OutputFormat,
   ```

### Git Worktree 操作

1. **列出 worktree**:
   ```rust
   let repo = Repository::open(path)?;
   let worktrees = repo.worktrees()?;
   ```

2. **创建 worktree**:
   ```rust
   let repo = Repository::open(path)?;
   let branch = repo.find_branch(branch_name, BranchType::Local)?;
   let commit = branch.get().peel_to_commit()?;
   repo.worktree(name, path, &commit)?;
   ```

3. **删除 worktree**:
   ```rust
   let repo = Repository::open(path)?;
   let mut worktree = repo.find_worktree(name)?;
   worktree.prune()?;
   ```

### 交互式设计原则

1. **默认行为**: 无参数时进入交互模式
2. **幂等性**: 重复执行相同命令不应产生副作用
3. **明确性**: 操作前提示确认（如删除 worktree）
4. **可配置性**: 支持环境变量和配置文件

## Security Considerations

1. **路径验证**: 防止路径遍历攻击（如 `../../../etc/passwd`）
2. **命令注入**: 验证所有用户输入，不直接拼接 shell 命令
3. **权限检查**: 确保用户有权限修改 Git 仓库和 worktree 目录
4. **敏感信息**: 不在日志中记录敏感信息（如 token、密码）

## Compatibility

### Git 版本

- **最低要求**: Git 2.5.0+ (worktree 引入版本)
- **推荐版本**: Git 2.30.0+ (worktree 功能稳定)
- **测试版本**: Git 2.40.0+ (CI 环境版本)

### 操作系统

- **Linux**: 全功能支持（主要目标平台）
- **macOS**: 全功能支持（主要目标平台）
- **Windows**: 通过 Git Bash/WSL 支持（次要目标平台）

### Rust 版本

- **最低要求**: Rust 1.75.0+ (2024 edition)
- **推荐版本**: Rust 1.80.0+ (最新稳定版)

## Documentation Strategy

1. **用户文档**:
   - README.md: 安装、快速入门、示例
   - `work --help`: 命令行帮助
   - man pages: Unix 手册页（可选）

2. **开发文档**:
   - 代码注释：rustdoc 文档注释
   - ARCHITECTURE.md: 架构设计文档（可选）

## Dependencies Summary

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
git2 = "0.18"
inquire = "0.7"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
comfy-table = "0.7"
env_logger = "0.11"
log = "0.4"
rayon = "1.10"  # 并行处理（可选）

[dev-dependencies]
tempfile = "3.10"  # 临时目录和文件
assert_cmd = "2.0"  # CLI 测试
predicates = "3.1"  # 断言匹配
```

## Open Questions

**无** - 所有技术决策已明确。

## Next Steps

Phase 1: 设计数据模型、快速入门指南和更新 AI 代理上下文。
