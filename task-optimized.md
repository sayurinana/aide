# 任务解析结果：aide 全局配置支持

## 原始内容概述

用户希望 aide 工具支持全局配置（`~/.aide/config.toml`），作为项目配置的模板源。`aide init` 时先确保全局配置存在，再将其复制到项目目录。同时为 `init`、`config get`、`config set`、`config reset`、`config update` 命令添加 `--global` 标志，支持直接操作全局配置。

## 核心意图

建立"全局配置 + 项目配置"的两级配置体系：全局配置作为所有新项目的默认模板，项目配置独立于全局配置可单独修改。

## 结构化任务描述

### 目标

为 aide 添加全局配置文件支持（`~/.aide/config.toml`），并为相关命令添加 `--global` 标志以支持直接操作全局配置。

### 具体要求

#### 1. 全局配置目录和文件

- 全局配置路径：`$HOME/.aide/config.toml`
- 若 `~/.aide/` 目录不存在，需自动创建
- 全局配置内容与项目配置完全一致（同一个 `DEFAULT_CONFIG` 模板）

#### 2. `aide init`（无 --global）行为变更

- **步骤 1**：检查全局配置 `~/.aide/config.toml` 是否存在
  - 若不存在：创建 `~/.aide/` 目录（如需要）并写入默认配置
  - 若已存在：不做任何操作
- **步骤 2**：检查项目配置 `.aide/config.toml` 是否存在
  - 若不存在：从全局配置**复制**到项目目录的 `.aide/config.toml`
  - 若已存在：跳过，不覆盖
- **步骤 3**（schema 版本检测）：当全局配置的 `meta.schema_version` 低于当前 aide 版本的 `CURRENT_SCHEMA_VERSION` 时，向用户输出提示信息（例如："全局配置 schema 版本较低，建议执行 `aide config update --global` 升级"），但不自动升级
- **步骤 4**：继续执行现有逻辑（`ensure_gitignore` 等）

#### 3. `aide init --global`

- 仅在用户主目录下创建 `~/.aide/config.toml`
- 若 `~/.aide/` 目录不存在，自动创建
- 若 `~/.aide/config.toml` 已存在，无操作，仅输出提示："全局配置已存在：~/.aide/config.toml"
- **不**修改当前工作目录下的任何文件
- **不**在当前工作目录创建 `.aide/` 目录

#### 4. `aide config get --global <key>`

- 从全局配置 `~/.aide/config.toml` 中读取指定配置项的值
- 若全局配置文件不存在，输出错误提示

#### 5. `aide config set --global <key> <value>`

- 修改全局配置 `~/.aide/config.toml` 中指定配置项的值
- 若全局配置文件不存在，输出错误提示

#### 6. `aide config reset --global [--force]`

- 仅对 `~/.aide/config.toml` 进行重置操作
- 重置前同样需要备份（备份到 `~/.aide/backups/`）和确认（除非 `--force`）
- **不**影响当前工作目录下的配置文件

#### 7. `aide config update --global`

- 仅对 `~/.aide/config.toml` 进行 schema 版本升级
- **不**影响当前工作目录下的配置文件

### 约束条件

- 全局配置和项目配置格式完全一致，使用同一个 `DEFAULT_CONFIG` 模板
- 全局配置路径固定为 `$HOME/.aide/config.toml`
- 现有项目配置的功能和行为不受影响（向后兼容）
- `--global` 标志在 `init`、`config get`、`config set`、`config reset`、`config update` 五个命令上统一支持
- 备份目录在全局模式下为 `~/.aide/backups/`

### 期望产出

- 修改 `src/main.rs`：为 `Init` 和各 `ConfigCommands` 添加 `--global` 参数
- 修改 `src/cli/init.rs`：实现 `--global` 分支逻辑和全局配置初始化
- 修改 `src/cli/config.rs`：各子命令支持 `--global` 操作全局配置
- 修改 `src/core/config.rs`：`ConfigManager` 支持全局配置路径操作（或新增全局配置辅助函数）
- 对应的单元测试

## 分析发现

### 识别的风险

- `$HOME` 环境变量在某些环境下可能未设置（如容器、CI），需要有降级处理
- 全局配置中的相对路径（如 `task.source = "task-now.md"`）在全局层面只是作为模板的默认值，实际运行时仍以项目目录为基准，这在语义上是合理的
- `config.md` 说明文档是否也需要在全局目录下生成？建议一致处理（全局目录下也生成）

### 优化建议

- 使用 Rust 标准库的 `dirs::home_dir()` 或 `std::env::var("HOME")` 获取主目录，建议优先使用 `dirs` crate 以获得跨平台兼容性（如果项目已引入），否则使用 `std::env::var("HOME")`
- 可以在 `ConfigManager` 中添加一个 `new_global()` 构造函数，以 `~/.aide/` 为根目录创建实例，复用现有的 `ensure_config`、`load_config` 等方法，避免代码重复
- 全局 `config.md` 建议也同步生成

## 复杂度评估

| 维度 | 评估 | 说明 |
|------|------|------|
| 结构复杂度 | 中 | 涉及 4 个文件的修改，新增参数和分支逻辑 |
| 逻辑复杂度 | 中 | init 流程需要处理全局/项目两级配置的协调 |
| 集成复杂度 | 低 | 不涉及外部依赖，仅文件系统操作 |
| 风险等级 | 低 | 纯新增功能，现有行为通过"不覆盖"策略保持兼容 |
