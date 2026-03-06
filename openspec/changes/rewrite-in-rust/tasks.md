## 1. 项目骨架搭建
- [ ] 1.1 更新 Cargo.toml，添加依赖（clap, serde, serde_json, toml, toml_edit, tokio, axum, chrono, fs2, rand）
- [ ] 1.2 创建 Rust 模块目录结构（src/cli/, src/core/, src/flow/, src/decide/, src/utils.rs）
- [ ] 1.3 实现 main.rs CLI 框架（clap derive，定义所有命令和子命令的参数结构）

## 2. 核心基础层 (core)
- [ ] 2.1 实现 output.rs（ok/warn/err/info/step 输出函数）
- [ ] 2.2 实现 project.rs（项目根目录三阶段发现算法）
- [ ] 2.3 实现 config.rs — ConfigManager（.aide 目录创建、config.toml 读写、默认配置生成、.gitignore 管理）
- [ ] 2.4 实现 config.rs — 点分隔键值读取/写入（使用 toml_edit 保留注释）
- [ ] 2.5 实现 config.rs — 值类型自动推断（bool/int/float/string）

## 3. Init 和 Config 命令
- [ ] 3.1 实现 cli/init.rs（`aide init` 命令处理）
- [ ] 3.2 实现 cli/config.rs（`aide config get` 和 `aide config set` 命令处理）
- [ ] 3.3 编写默认配置模板（带中文注释的完整 config.toml 内容）

## 4. Flow 数据层
- [ ] 4.1 实现 flow/types.rs（FlowStatus, HistoryEntry, BranchesData, BranchInfo, BackConfirmState 数据结构 + serde 序列化）
- [ ] 4.2 实现 flow/storage.rs（flow-status.json 原子读写、文件锁、状态归档、任务列表查询）
- [ ] 4.3 实现 flow/validator.rs（环节校验规则：相邻跳转、回退范围、phases 列表非空、无重复）

## 5. Flow Git 集成
- [ ] 5.1 实现 flow/git.rs（git add/commit/rev-parse/status/branch/checkout/merge 封装）
- [ ] 5.2 实现 flow/branch.rs — BranchManager（分支创建、编号管理、branches.json 读写、branches.md 生成）
- [ ] 5.3 实现 flow/branch.rs — 正常合并策略（squash merge 到源分支）
- [ ] 5.4 实现 flow/branch.rs — 临时分支合并策略（源分支有新提交时）

## 6. Flow Hooks
- [ ] 6.1 实现 flow/hooks.rs — PlantUML hook（文件收集、命令路径解析、语法检查、PNG 生成）
- [ ] 6.2 实现 flow/hooks.rs — CHANGELOG hook（进入 docs 提醒、离开 docs 验证修改）
- [ ] 6.3 实现 flow/hooks.rs — Finish 清理 hook（删除 task-plans 目录内容）

## 7. Flow 命令
- [ ] 7.1 实现 flow/tracker.rs — FlowTracker 核心编排（校验→hooks→存储→git→输出）
- [ ] 7.2 实现 cli/flow.rs — `aide flow start`（含分支创建和状态初始化）
- [ ] 7.3 实现 cli/flow.rs — `aide flow next-step` 和 `aide flow back-step`
- [ ] 7.4 实现 cli/flow.rs — `aide flow next-part`（含 pre/post hooks、finish 合并逻辑）
- [ ] 7.5 实现 cli/flow.rs — `aide flow back-part` 和 `aide flow back-confirm`（两阶段确认）
- [ ] 7.6 实现 cli/flow.rs — `aide flow issue` 和 `aide flow error`
- [ ] 7.7 实现 cli/flow.rs — `aide flow status`、`aide flow list`、`aide flow show`
- [ ] 7.8 实现 cli/flow.rs — `aide flow clean`（强制清理）
- [ ] 7.9 实现任务完成清理（lock 文件、spec 文件、decisions、diagrams 清理）

## 8. Decide 数据层
- [ ] 8.1 实现 decide/types.rs（DecideInput, DecideItem, Option, Location, DecideOutput, Decision, DecisionRecord, ServerInfo 数据结构 + serde + 验证逻辑）
- [ ] 8.2 实现 decide/storage.rs（pending.json 读写、session 文件保存、server.json 管理）

## 9. Decide HTTP 服务器
- [ ] 9.1 实现 decide/server.rs — 异步 HTTP 服务器（tokio + axum，端口探测、CORS、graceful shutdown）
- [ ] 9.2 实现 decide/handlers.rs — GET /api/items（返回待定项数据 + 源码内容）
- [ ] 9.3 实现 decide/handlers.rs — POST /api/submit（验证、保存、触发关闭）
- [ ] 9.4 实现 decide/handlers.rs — 静态文件服务（从 --web-dir 指定目录加载 HTML/CSS/JS）
- [ ] 9.5 实现 decide/handlers.rs — 错误响应（400/404/405/413/500）

## 10. Decide 命令
- [ ] 10.1 实现 cli/decide.rs — `aide decide submit`（JSON 读取、验证、保存 pending、启动后台服务器、输出 URL）
- [ ] 10.2 实现 cli/decide.rs — `aide decide result`（检查结果、输出 JSON、清理）
- [ ] 10.3 实现后台进程管理（detach 子进程启动服务器、PID 跟踪）

## 11. Web 前端
- [ ] 11.1 将现有 Web 前端文件（index.html, style.css, app.js）复制到 web/ 目录
- [ ] 11.2 验证 Web 前端与新 API 的兼容性（确认接口路径和数据格式一致）

## 12. 测试
- [ ] 12.1 core 模块单元测试（config 读写、项目根目录发现、输出格式化）
- [ ] 12.2 flow 模块单元测试（状态存储、环节校验、分支管理）
- [ ] 12.3 decide 模块单元测试（数据验证、存储操作）
- [ ] 12.4 集成测试（完整 CLI 命令流程测试）

## 13. 文档
- [ ] 13.1 编写 docs/README.md（文档索引）
- [ ] 13.2 编写命令文档（init、config、flow、decide 各子命令详细说明）
- [ ] 13.3 编写数据格式文档（config.toml、flow-status.json、branches.json、decide JSON 格式）
- [ ] 13.4 编写安装和构建指南
