# Changelog

## [Unreleased]

### Added

- **fastzip-gui**：图形界面（eframe/egui）
  - 解压：选择压缩包、目标目录，智能解压/密码，归档顶层预览
  - 压缩：添加文件或目录，输出 ZIP/7z，递归选项
  - 底部状态栏显示进度与错误

## [0.3.0] - Phase 3 & Phase 4

### Added（Phase 3：格式扩展与完善）

- **ZIP 加密解压**：支持密码保护的 ZIP（ZipCrypto/AES），通过 `-p/--password` 或环境变量 `FASTZIP_PASSWORD` 提供密码
- **RAR 支持**：启用 `full` feature 时支持 .rar 解压（基于 unrar），含多卷与密码
- **路径规范化**：`path_utils::normalize_entry_path` 防止路径穿越，Unicode/UTF-8 文件名兼容

### Added（Phase 4：压缩功能）

- **压缩子命令**：`fastzip compress` / `fastzip c`，将文件或目录打包为 .zip 或 .7z
- **ZIP 创建**：`-o output.zip`，支持多源、递归目录（`--recursive`，默认开启）
- **7z 创建**：`-o output.7z`，单一路径（文件或目录）
- **核心库**：`compress_to_zip`、`compress_to_7z`、`CompressOptions` 供 CLI 与未来 GUI 使用

### Changed

- **fastzip-core**：新增可选依赖 `walkdir`、可选 `unrar`（feature `full`）
- **fastzip-cli**：新增 feature `full`，透传至 core 以启用 RAR；解压/压缩分支到 `run_extract` / `run_compress`

### Note

- CAB、ISO 等格式未在本阶段实现（可后续通过 libarchive 等方案接入）

## [0.2.0] - Phase 2

### Added

- **Windows**: 右键菜单（解压到此处、智能解压到此处），支持 .zip/.7z/.rar/.tar/.gz/.tgz/.xz/.bz2/.zst
- **Windows**: `scripts/install-windows.ps1` 安装脚本、`scripts/uninstall-windows.ps1` 卸载脚本
- **macOS**: Finder 服务「FastZip Smart Extract」，安装脚本 `scripts/install-macos.sh`
- **Linux**: Nautilus 脚本（右键 -> Scripts）、Dolphin 服务菜单，安装脚本 `scripts/install-linux.sh`
- **CI/CD**: GitHub Actions CI（多平台测试与构建）、Release 工作流

## [0.1.0] - Phase 1

### Added

- CLI: `fastzip extract` / `fastzip x` 解压子命令
- 支持格式: ZIP、7z、tar.gz、tar.xz、tar.bz2、tar.zst、gz、xz、bz2、zst
- 智能解压（单文件/单根目录/多文件）
- 并行解压、进度条、tracing 日志
- 选项: `--dest`、`--smart`、`--flat`、`--overwrite`、`--password`、`--quiet`
