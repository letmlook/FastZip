#!/bin/bash
# FastZip Linux 安装脚本 - 安装 CLI 并创建文件管理器右键菜单
# 支持 Nautilus (GNOME)、Dolphin (KDE)

set -e

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# 安装 fastzip 到 INSTALL_DIR
install_binary() {
    if command -v fastzip &>/dev/null; then
        echo "fastzip 已在 PATH 中: $(which fastzip)"
        return 0
    fi
    if [ -f "$PROJECT_ROOT/target/release/fastzip" ]; then
        mkdir -p "$INSTALL_DIR"
        cp "$PROJECT_ROOT/target/release/fastzip" "$INSTALL_DIR/"
        echo "已复制 fastzip 到 $INSTALL_DIR"
        echo "请确保 $INSTALL_DIR 在 PATH 中: export PATH=\"\$PATH:$INSTALL_DIR\""
    else
        echo "请先运行: cargo build -p fastzip --release"
        exit 1
    fi
}

# Nautilus (GNOME Files) 右键脚本 - 出现在 Scripts 子菜单
install_nautilus_scripts() {
    FASTZIP_PATH=$(which fastzip 2>/dev/null || echo "$INSTALL_DIR/fastzip")
    SCRIPTS_DIR="$HOME/.local/share/nautilus/scripts"
    mkdir -p "$SCRIPTS_DIR"
    
    # 智能解压
    cat > "$SCRIPTS_DIR/FastZip 智能解压" << EOF
#!/bin/bash
for f in \$NAUTILUS_SCRIPT_SELECTED_FILE_PATHS; do
  if [ -f "\$f" ]; then
    cd "\$(dirname "\$f")"
    $FASTZIP_PATH x "\$f" -s -q
  fi
done
EOF
    chmod +x "$SCRIPTS_DIR/FastZip 智能解压"
    
    # 解压到此处
    cat > "$SCRIPTS_DIR/FastZip 解压到此处" << EOF
#!/bin/bash
for f in \$NAUTILUS_SCRIPT_SELECTED_FILE_PATHS; do
  if [ -f "\$f" ]; then
    cd "\$(dirname "\$f")"
    $FASTZIP_PATH x "\$f" -f -q
  fi
done
EOF
    chmod +x "$SCRIPTS_DIR/FastZip 解压到此处"
    
    echo "已添加 Nautilus 脚本（右键 -> Scripts -> FastZip ...）"
}

# Dolphin (KDE) 服务菜单 - 使用 .desktop 格式
install_dolphin_services() {
    FASTZIP_PATH=$(which fastzip 2>/dev/null || echo "$INSTALL_DIR/fastzip")
    SERVICES_DIR="$HOME/.local/share/kio/servicemenus"
    mkdir -p "$SERVICES_DIR"
    
    cat > "$SERVICES_DIR/fastzip.desktop" << EOF
[Desktop Entry]
Type=Service
ServiceTypes=KonqPopupMenu/Plugin
MimeType=application/zip;application/x-7z-compressed;application/x-tar;application/gzip;application/x-xz;application/x-bzip2;application/zstd;
Actions=smartExtract;extractHere;

[Desktop Action smartExtract]
Name=智能解压到此处
Exec=$FASTZIP_PATH x %F -s -q

[Desktop Action extractHere]
Name=解压到此处
Exec=$FASTZIP_PATH x %F -f -q
EOF
    
    echo "已添加 Dolphin 右键菜单（如使用 KDE）"
}

# 通用 .desktop 关联（部分文件管理器）
install_mime_association() {
    FASTZIP_PATH=$(which fastzip 2>/dev/null || echo "$INSTALL_DIR/fastzip")
    APPS_DIR="$HOME/.local/share/applications"
    mkdir -p "$APPS_DIR"
    
    cat > "$APPS_DIR/fastzip-extract.desktop" << EOF
[Desktop Entry]
Name=FastZip 解压
Comment=使用 FastZip 解压
Exec=$FASTZIP_PATH x %f
Terminal=false
Type=Application
MimeType=application/zip;application/x-7z-compressed;application/x-tar;application/gzip;application/x-xz;application/x-bzip2;application/zstd;
Categories=Utility;
EOF
    
    echo "已创建 .desktop 应用关联"
}

install_binary
install_nautilus_scripts
install_dolphin_services
install_mime_association

echo ""
echo "安装完成。请确保 fastzip 在 PATH 中。"
echo "若使用 Nautilus，可能需要重启文件管理器。"
