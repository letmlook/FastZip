# Changelog

## [Unreleased]

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
