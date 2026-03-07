# 安装和构建指南

## 环境要求

### 必需

- **Rust 工具链**: edition 2024（Rust 1.85+）
  - 安装: https://rustup.rs/

### 可选

- **Git**: 用于 flow 命令的分支管理和自动提交
- **Java + PlantUML**: 用于流程图生成 hooks

## 从源码构建

### 1. 克隆仓库

```bash
git clone <repo-url>
cd aide
```

### 2. 编译

```bash
# Debug 构建
cargo build

# Release 构建（推荐，体积更小、运行更快）
cargo build --release
```

编译产物：
- Debug: `target/debug/aide`
- Release: `target/release/aide`

### 3. 运行测试

```bash
cargo test
```

### 4. 安装到系统

```bash
# 安装到 ~/.cargo/bin/（需要该目录在 PATH 中）
cargo install --path .
```

或手动复制：

```bash
cp target/release/aide /usr/local/bin/
```

## 部署

### 单文件部署

Aide 编译为单一可执行文件，无运行时依赖。部署只需：

1. 将 `aide` 可执行文件放到目标位置
2. 将 `web/` 目录放到可执行文件同级目录下（用于 decide Web UI）

```
/opt/aide/
├── aide          # 可执行文件
└── web/          # Web 前端文件
    ├── index.html
    ├── style.css
    └── app.js
```

### Web 前端路径

`aide decide submit` 默认从可执行文件同级 `web/` 目录加载前端文件。可通过 `--web-dir` 参数指定其他路径：

```bash
aide decide submit data.json --web-dir /path/to/custom/web
```

### PlantUML 配置

如需使用 PlantUML 流程图功能：

1. 安装 Java 运行环境
2. 下载 PlantUML jar 文件
3. 配置 jar 路径（三种方式，按优先级）：
   - `aide config set plantuml.jar_path /path/to/plantuml.jar`
   - 将 `plantuml.jar` 放到可执行文件同级 `lib/` 目录下
   - 确保系统 PATH 中有 `plantuml` 命令

## 交叉编译

借助 Rust 的交叉编译能力，可为其他平台构建：

```bash
# 添加目标平台
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-apple-darwin

# 编译（可能需要对应的链接器）
cargo build --release --target x86_64-unknown-linux-musl
```

## 依赖说明

| 依赖 | 版本 | 用途 |
|------|------|------|
| clap | 4 | CLI 参数解析（derive API） |
| serde | 1 | 序列化框架 |
| serde_json | 1 | JSON 序列化/反序列化 |
| toml | 0.8 | TOML 配置读取 |
| toml_edit | 0.22 | TOML 配置写入（保留注释） |
| tokio | 1 | 异步运行时（decide 服务器） |
| axum | 0.8 | HTTP 框架 |
| tower-http | 0.6 | HTTP 中间件（CORS） |
| chrono | 0.4 | 时间处理 |
| fs2 | 0.4 | 文件锁 |
| rand | 0.8 | 随机数生成 |
| ctrlc | 3 | Ctrl+C 信号处理 |
| libc | 0.2 | Unix 系统调用（仅 Unix） |
