# 任务：将 Aide CLI 工具从 Python 重写为 Rust

## 任务概述

将 Aide 命令行工作流辅助工具从当前的 Python 3.11+ 实现迁移为 Rust 实现，编译为单一可执行文件，消除 Python 运行环境依赖。

## 迁移范围

### 包含（需实现）

| 命令 | 子命令 | 说明 |
|------|--------|------|
| `aide init` | — | 初始化 .aide 目录和默认配置 |
| `aide config` | `get <key>` | 读取 TOML 配置值（点分隔键） |
| | `set <key> <value>` | 写入配置值（保留注释，自动推断类型） |
| `aide flow` | `start <phase> "<summary>"` | 开始新任务（含创建 git 分支） |
| | `next-step "<summary>"` | 步骤前进 |
| | `back-step "<reason>"` | 步骤回退 |
| | `next-part <phase> "<summary>"` | 环节前进（仅允许相邻跳转） |
| | `back-part <phase> "<reason>"` | 环节回退（两阶段确认） |
| | `back-confirm --key <key>` | 确认环节回退 |
| | `issue "<description>"` | 记录非阻塞问题 |
| | `error "<description>"` | 记录阻塞错误 |
| | `status` | 查看当前任务状态 |
| | `list` | 列出所有任务 |
| | `show <task_id>` | 查看任务详情 |
| | `clean` | 强制清理当前任务 |
| `aide decide` | `submit <file>` | 提交待定项 JSON 并启动 Web 服务 |
| | `result` | 获取用户决策结果 |

### 排除（不实现）

- `aide env` 及其所有子命令（`ensure`、`list`、`set`）
- 与 env 相关的环境检测模块（python、uv、venv、rust、node、flutter、android 等）

## 技术选型

| 领域 | 选择 | 理由 |
|------|------|------|
| 语言 | Rust (edition 2024) | 单一可执行文件，无运行时依赖 |
| CLI 框架 | clap (derive API) | 生态成熟，自动生成帮助信息 |
| 异步运行时 | tokio | 用户要求异步实现 decide 服务器 |
| HTTP 框架 | axum | tokio 生态一流框架 |
| TOML 读取 | toml | 标准 TOML 解析 |
| TOML 写入 | toml_edit | 保留注释和格式 |
| JSON | serde_json | 标准 JSON 处理 |
| 序列化 | serde | 统一序列化框架 |
| 文件锁 | fs2 | 跨平台文件锁 |
| 时间处理 | chrono | 时区感知的时间操作 |

## 关键技术决策

### Web 前端文件加载方式

- 从文件系统加载，**不**编译时嵌入
- 默认路径：可执行文件所在目录下的 `web/`（相对于可执行文件路径，非工作目录）
- 可通过命令行参数 `--web-dir` 覆盖

### PlantUML Hooks

- 完整保留，通过 `std::process::Command` 调用外部 PlantUML
- JAR 路径解析顺序：配置 `plantuml.jar_path` → 可执行文件相对 `lib/plantuml.jar` → 系统 PATH 中的 `plantuml`

### 数据格式兼容

- 与 Python 版本的 `.aide/` 目录结构完全兼容
- JSON schema（flow-status.json、branches.json、decisions/*.json）保持不变
- TOML 配置格式保持不变

## 参考资源

| 资源 | 路径 | 说明 |
|------|------|------|
| Python 源码 | `aide/` | 完整的参考实现 |
| 程序设计文档 | `old-data-reference/aide-program.md` | 组件和接口说明 |
| 命令文档 | `old-data-reference/docs/commands/` | 各命令的详细设计 |
| 数据格式文档 | `old-data-reference/docs/formats/` | 配置和数据格式规范 |

## 文档要求

- 完成 Rust 实现后编写全新配套文档
- 保存到项目根目录下的 `docs/` 目录
- 文档内容要求详细、清晰
- 使用简体中文
- 不维护旧文档（old-data-reference 仅作参考）

## 交付物

1. 完整的 Rust 源码（`src/` 目录）
2. Web 前端文件（`web/` 目录，复用现有 HTML/CSS/JS）
3. 中文配套文档（`docs/` 目录）
4. 更新后的 Cargo.toml（含所有依赖）
