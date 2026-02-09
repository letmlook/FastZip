# FastZip 安装脚本

用于在各平台添加右键菜单/服务集成。

## Windows

```powershell
# 安装（需先构建 fastzip）
cargo build -p fastzip --release
powershell -ExecutionPolicy Bypass -File scripts/install-windows.ps1 -FastZipPath "target\release\fastzip.exe"

# 若 fastzip 已在 PATH 中，可省略 -FastZipPath
powershell -ExecutionPolicy Bypass -File scripts/install-windows.ps1

# 卸载
powershell -ExecutionPolicy Bypass -File scripts/uninstall-windows.ps1
```

右键 .zip、.7z 等压缩文件，可见「Extract Here」「Smart Extract」。

## macOS

```bash
# 安装
cargo build -p fastzip --release
./scripts/install-macos.sh

# 用法：Finder 中选中压缩文件 -> 右键 -> 服务 -> FastZip Smart Extract
```

## Linux

```bash
# 安装
cargo build -p fastzip --release
./scripts/install-linux.sh

# Nautilus: 右键 -> Scripts -> FastZip 智能解压 / FastZip 解压到此处
# Dolphin: 右键直接显示
```

确保 `~/.local/bin`（或 INSTALL_DIR）在 PATH 中。
