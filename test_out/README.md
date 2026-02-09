# FastZip

跨平台快速解压缩工具，参考 Bandizip 设计，支持主流压缩格式与智能解压。

## 功能特性

- **智能解压**：根据压缩包内容自动选择目标路径（单文件→当前目录；单根目录→当前目录；多文件→创建子文件夹）
- **多格式支持**：ZIP、7z、tar.gz、tar.xz、tar.bz2、tar.zst、gz、xz、bz2、zst
- **并行解压**：多文件批量解压时使用多核加速

## 安装

```bash
cargo install --path crates/fastzip-cli
```

或从项目根目录构建：

```bash
cargo build -p fastzip --release
# 可执行文件位于 target/release/fastzip
```

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
```

### 子命令

- `extract` / `x`：解压压缩文件

### 选项

| 选项 | 说明 |
|------|------|
| `-d, --dest <DIR>` | 解压目标目录 |
| `-s, --smart` | 智能解压（默认） |
| `-f, --flat` | 解压到此处，不使用智能解压 |
| `-o, --overwrite` | 覆盖已存在文件 |
| `-p, --password <PWD>` | 密码（或使用 `FASTZIP_PASSWORD` 环境变量） |
| `-q, --quiet` | 静默模式 |

## 项目结构

```
FastZip/
├── crates/
│   ├── fastzip-core/   # 核心库：解压引擎、智能解压、格式适配
│   └── fastzip-cli/    # 命令行工具
├── DESIGN.md           # 设计方案
└── DEVELOPMENT_PLAN.md # 开发计划
```

## 开发状态

- [x] Phase 1：CLI MVP、主流格式、智能解压
- [ ] Phase 2：Windows/macOS/Linux 右键菜单集成
- [ ] Phase 3：RAR、加密、Unicode 完善

## License

MIT
