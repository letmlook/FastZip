#!/bin/bash
# FastZip macOS 安装脚本 - 安装 CLI 并创建 Finder 右键服务
# 用法: 在项目根目录运行 ./scripts/install-macos.sh

set -e

INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
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
    else
        echo "请先运行: cargo build -p fastzip --release"
        echo "或将 fastzip 放到 PATH 中（如 $INSTALL_DIR）"
        exit 1
    fi
}

# 创建 shell 脚本供 Automator 服务调用
create_extract_script() {
    FASTZIP_PATH=$(which fastzip 2>/dev/null || echo "$INSTALL_DIR/fastzip")
    SCRIPT_PATH="$HOME/.local/bin/fastzip-extract.sh"
    mkdir -p "$(dirname "$SCRIPT_PATH")"
    cat > "$SCRIPT_PATH" << EOF
#!/bin/bash
# FastZip 智能解压 - 供 Finder 服务调用
for f in "\$@"; do
  if [ -f "\$f" ]; then
    cd "\$(dirname "\$f")"
    "$FASTZIP_PATH" x "\$f" -s -q
  fi
done
EOF
    chmod +x "$SCRIPT_PATH"
    echo "已创建脚本: $SCRIPT_PATH"
}

# 创建 Automator 工作流（Finder 服务）
create_finder_service() {
    create_extract_script
    SCRIPT_PATH="$HOME/.local/bin/fastzip-extract.sh"
    WORKFLOW_DIR="$HOME/Library/Services/FastZip Smart Extract.workflow"
    mkdir -p "$WORKFLOW_DIR/Contents"
    
    # Automator workflow plist
    cat > "$WORKFLOW_DIR/Contents/document.wflow" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>AMDocumentBuild</key>
    <string>522</string>
    <key>actions</key>
    <array>
        <dict>
            <key>action</key>
            <dict>
                <key>AMActionScriptKey</key>
                <string>run shell script</string>
                <key>AMActionScriptSelectedShell</key>
                <string>/bin/bash</string>
                <key>AMActionScriptTextKey</key>
                <string>$SCRIPT_PATH "\$@"</string>
            </dict>
        </dict>
    </array>
    <key>connectors</key>
    <dict/>
    <key>workflowMetaData</key>
    <dict>
        <key>workflowTypeIdentifier</key>
        <string>com.apple.Automator.servicesMenu</string>
    </dict>
</dict>
</plist>
EOF
    
    # Info.plist for workflow
    cat > "$WORKFLOW_DIR/Contents/Info.plist" << 'INFOPLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>FastZip Smart Extract</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
</dict>
</plist>
INFOPLIST
    
    echo "已创建 Finder 服务: FastZip Smart Extract"
    echo "用法: 在 Finder 中选中压缩文件 -> 右键 -> 服务 -> FastZip Smart Extract"
}

install_binary
create_finder_service
