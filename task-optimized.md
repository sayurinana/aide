# 任务解析结果

## 原始内容概述

用户希望将 PlantUML 工具集成到 aide 程序中，使其不依赖外部环境即可运行。需要在配置系统中添加下载缓存、安装路径、下载链接等配置项，并在 `aide init --global` 和 `aide -V` 命令中集成 PlantUML 的检测、安装和版本显示功能。

## 核心意图

将 PlantUML 作为 aide 的内置工具，通过自动下载和解压自包含的可执行程序包实现开箱即用，消除对外部 Java/PlantUML 环境的依赖。

## 结构化任务描述

### 目标

在 aide 配置系统中新增工具管理相关配置，并实现 PlantUML 可执行程序的自动检测、下载、解压和版本显示功能。

### 具体要求

#### 1. 配置变更

在 `DEFAULT_CONFIG` 中进行以下修改：

**废弃字段：**
- 移除 `[plantuml]` 节中的 `jar_path` 字段

**新增字段（位于 `[plantuml]` 节）：**

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `plantuml.download_cache_path` | 字符串 | `"download-buffer"` | 下载缓存目录，相对于 `~/.aide/` |
| `plantuml.clean_cache_after_install` | 布尔值 | `true` | 安装完成后是否删除下载的压缩包 |
| `plantuml.install_path` | 字符串 | `"utils"` | 工具程序安装目录，相对于 `~/.aide/` |
| `plantuml.download_url` | 字符串 | `"https://github.com/sayurinana/agent-aide/releases/download/resource-001/plantuml-1.2025.4-linux-x64.tar.gz"` | PlantUML 程序包下载链接 |

> 所有相对路径均相对于 `~/.aide/` 目录。例如 `install_path = "utils"` 对应 `~/.aide/utils/`。

**PlantUML 可执行文件路径拼接规则：**
`~/.aide/{install_path}/plantuml/bin/plantuml`

按默认配置即为：`~/.aide/utils/plantuml/bin/plantuml`

#### 2. `aide init --global` 命令增强

在全局初始化完成后，增加 PlantUML 检测和安装流程：

1. 根据配置拼接 PlantUML 可执行文件路径
2. 检测该文件是否存在且可执行
3. 如可用：输出 PlantUML 版本信息（执行 `plantuml -version`）
4. 如不可用：
   - 提示用户 "PlantUML 未安装，是否现在自动下载并安装？[Y/n]"
   - 用户确认后执行自动安装：
     a. 创建下载缓存目录（`~/.aide/{download_cache_path}/`）
     b. 从 `plantuml.download_url` 下载压缩包到缓存目录
     c. 创建安装目录（`~/.aide/{install_path}/`）
     d. 解压 tar.gz 到安装目录（解压后应得到 `plantuml/` 子目录）
     e. 验证安装结果（检测可执行文件是否存在）
     f. 如配置了 `clean_cache_after_install = true`，删除下载的压缩包
     g. 输出安装成功信息及版本号

#### 3. `aide -V` 命令增强

自定义版本输出行为（替代 clap 默认的 `--version` 处理）：

输出格式：
```
aide 0.1.0

PlantUML:
  版本: 1.2025.4
  路径: ~/.aide/utils/plantuml/bin/plantuml
  状态: 可用
```

当 PlantUML 不可用时：
```
aide 0.1.0

PlantUML:
  状态: 未安装
  提示: 运行 aide init --global 安装 PlantUML
```

#### 4. 配置文档更新

同步更新 `DEFAULT_CONFIG_MD` 中的 `[plantuml]` 节说明，反映新增和废弃的配置项。

### 约束条件

- 仅支持 Linux x64 平台
- 使用 reqwest 库实现 HTTP 下载（需新增依赖）
- 解压 tar.gz 使用 flate2 + tar 库（需新增依赖）
- 所有路径配置均为相对于 `~/.aide/` 的相对路径
- PlantUML 检测应通过执行 `plantuml -version` 并解析输出获取版本号
- 下载过程应显示进度信息
- 必须同步更新 `meta.schema_version` 版本号

### 期望产出

- 更新后的 `DEFAULT_CONFIG` 和 `DEFAULT_CONFIG_MD` 常量
- PlantUML 管理模块（检测、下载、解压、验证）
- 增强后的 `handle_init_global()` 函数
- 自定义版本输出逻辑
- 对应的单元测试
- `schema_version` 升级及配置迁移处理

## 分析发现

### 识别的风险

- **网络依赖**：下载过程依赖网络连接，需处理网络超时、连接失败等情况
- **磁盘空间**：压缩包约 74MB，解压后更大，需考虑磁盘空间不足的情况
- **权限问题**：解压后的可执行文件需要正确的执行权限
- **配置迁移**：`jar_path` 废弃后，已有用户的配置文件需要迁移处理（`aide config update` 逻辑）
- **下载中断**：大文件下载可能中断，需考虑断点续传或重新下载

### 优化建议

- 下载时显示进度条或百分比，提升用户体验
- 对现有使用 `jar_path` 的代码进行检索，确保废弃后不会导致功能回归
- `aide -V` 的 PlantUML 检测应该是轻量的（仅检查文件存在），避免每次都执行子进程

## 复杂度评估

| 维度 | 评估 | 说明 |
|------|------|------|
| 结构复杂度 | 中 | 涉及 config.rs、init.rs、main.rs 及新增模块 |
| 逻辑复杂度 | 中 | 下载/解压/检测逻辑、用户交互、错误处理 |
| 集成复杂度 | 中 | 需引入 reqwest、flate2、tar 依赖 |
| 风险等级 | 低 | 新增功能为主，不影响现有核心流程 |

建议将任务拆分为：配置变更 → PlantUML 管理模块 → init 命令集成 → 版本命令集成 → 测试 五个阶段实施。
