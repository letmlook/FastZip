# FastZip

跨平台快速解压缩工具，参考 Bandizip 设计，支持主流压缩格式与智能解压。

## 功能特性

- **智能解压**：根据压缩包内容自动选择目标路径（单文件→当前目录；单根目录→当前目录；多文件→创建子文件夹）
- **多格式支持**：ZIP、7z、tar.gz、tar.xz、tar.bz2、tar.zst、gz、xz、bz2、zst；可选 **RAR**（`--features full`）
- **加密解压**：ZIP/7z 密码保护（`-p` 或 `FASTZIP_PASSWORD`）
- **压缩**：打包为 .zip 或 .7z（`compress` / `c` 子命令）
- **并行解压**：多文件批量解压时使用多核加速

## 安装

### 方式一：Cargo 安装

```bash
# 仅解压（ZIP/7z/tar 等）
cargo install --path crates/fastzip-cli

# 含 RAR 解压
cargo install --path crates/fastzip-cli --features full
```

### 方式二：从源码构建

```bash
cargo build -p fastzip-cli --release
# 可执行文件位于 target/release/fastzip-cli
```

**若编译时电脑卡顿**，项目已配置限流（`.cargo/config.toml`），或使用低资源脚本：

```powershell
# Windows
.\scripts\build-low.ps1 build -p fastzip-cli --release
```

### 方式三：平台 Shell 集成（右键菜单）

安装 fastzip 后，运行对应平台的安装脚本以添加右键菜单：

| 平台 | 命令 |
|------|------|
| **Windows** | `powershell -ExecutionPolicy Bypass -File scripts/install-windows.ps1 -FastZipPath "path/to/fastzip.exe"` |
| **macOS** | `./scripts/install-macos.sh` |
| **Linux** | `./scripts/install-linux.sh` |

- **Windows**：在 .zip、.7z 等压缩文件上右键，可见「解压到此处」「智能解压到此处」
- **macOS**：Finder 中选中压缩文件 → 右键 → 服务 → FastZip Smart Extract
- **Linux**：Nautilus 下为 右键 → Scripts → FastZip ...；Dolphin 下为右键直接显示

## 使用方法

```bash
# 智能解压（默认）
fastzip x archive.zip

# 解压到指定目录
fastzip x archive.zip -d ./output

# 解压到此处（flat，不使用智能解压）
fastzip x archive.zip -f

# 多文件并行解压
fastzip x a.zip b.7z c.tar.gz

# 加密包（密码）
fastzip x locked.zip -p mypassword

# 压缩为 ZIP 或 7z
fastzip c file1.txt dir/ -o out.zip
fastzip compress mydir -o archive.7z
```

### 子命令

- `extract` / `x`：解压压缩文件
- `compress` / `c`：压缩为 .zip 或 .7z（需 `-o` 指定输出路径）

### 选项

| 选项 | 说明 |
|------|------|
| `-d, --dest <DIR>` | 解压目标目录 |
| `-s, --smart` | 智能解压（默认） |
| `-f, --flat` | 解压到此处，不使用智能解压 |
| `-o, --overwrite` | 覆盖已存在文件 |
| `-p, --password <PWD>` | 密码（或使用 `FASTZIP_PASSWORD` 环境变量） |
| `-q, --quiet` | 静默模式 |

## 图形界面（GUI）

GUI 采用 **Leptos + Tauri** 实现（基于 [rust-ui.com](https://rust-ui.com) 生态，Leptos 组件库 + Tailwind 风格）。

### 环境要求

- Rust（含 `wasm32-unknown-unknown` 目标）
- [Trunk](https://trunkrs.dev/)（`cargo install trunk`）
- [Tauri CLI](https://tauri.app/)（`cargo install tauri-cli`）

### 构建与运行

```bash
cd crates/fastzip-gui
cargo tauri dev    # 开发模式（热重载）
cargo tauri build  # 生产构建
```

- **解压**：选择压缩包 → 选择目标目录 → 可选智能解压/密码 → 预览顶层条目 → 解压
- **压缩**：添加文件或目录 → 指定输出 .zip 或 .7z → 压缩

### 添加 rust-ui 组件（可选）

安装 `ui-cli` 后可使用 [rust-ui.com](https://rust-ui.com) 组件库：

```bash
cargo install ui-cli --force
cd crates/fastzip-gui
ui add button card tabs   # 示例：添加按钮、卡片、标签页组件
```

## 项目结构

```
FastZip/
├── crates/
│   ├── fastzip-core/        # 核心库：解压引擎、智能解压、格式适配、压缩
│   ├── fastzip-cli/         # 命令行工具
│   └── fastzip-gui/         # 图形界面（Leptos + Tauri）
│       ├── src/             # Leptos 前端 (WASM)
│       ├── src-tauri/       # Tauri 后端
│       ├── style/           # 样式 (rust-ui 风格)
│       ├── index.html       # Trunk 入口
│       └── Trunk.toml
├── DESIGN.md
└── DEVELOPMENT_PLAN.md
```

## 开发状态

- [x] Phase 1：CLI MVP、主流格式、智能解压
- [x] Phase 2：Windows/macOS/Linux 右键菜单集成
- [x] Phase 3：RAR（full feature）、ZIP/7z 加密解压、路径规范化
- [x] Phase 4：压缩功能（ZIP、7z 创建）

## License

MIT
