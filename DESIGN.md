# FastZip 跨平台快速解压缩工具 - 设计方案

> 参考 Bandizip，设计兼容市面上主流压缩格式的解压缩工具，C++ vs Rust 技术栈选型评估

---

## 一、产品功能设计（参考 Bandizip）

### 1.1 核心解压功能

| 功能 | 描述 | 实现优先级 |
|------|------|-----------|
| **解压到此处** | 直接将压缩包内容解压到当前文件夹 | P0 |
| **智能解压到此处** | 根据压缩包内容自动决定目标路径：单文件→当前目录；单根文件夹→当前目录；多文件/多文件夹→创建「压缩包名」文件夹；重名时使用 `(2)`、`(3)` 等 | P0 |
| **解压到 [文件夹名]** | 智能推荐目标文件夹（如 `125logs (2)`），快速解压 | P0 |
| **解压到其他文件夹...** | 打开文件选择对话框，用户手动选择路径 | P0 |
| **用 FastZip 打开...** | 主程序界面：预览、选择性解压、管理压缩包 | P1 |

### 1.2 Bandizip 智能解压逻辑（Extract Here Smart）

1. **单文件**：解压到当前目录
2. **所有文件都在一个根文件夹内**：解压到当前目录，创建该根文件夹
3. **其他情况**：创建 `(压缩包名)` 文件夹并解压；若已存在则使用 `(2)`、`(3)` 等

### 1.3 系统集成

| 平台 | 集成方式 |
|------|----------|
| **Windows** | 资源管理器右键上下文菜单、Shell Extension |
| **macOS** | Finder 右键菜单（服务）、Quick Look 插件 |
| **Linux** | 文件管理器右键菜单（Nautilus/Dolphin 等） |

### 1.4 其他重要特性

- **多核并行解压**：大文件/多文件场景
- **流式解压**：支持大文件，无需完整加载
- ** Unicode / UTF-8 文件名**：正确处理各平台编码
- **加密压缩包**：支持密码保护的 ZIP、7z、RAR 等

---

## 二、支持的压缩格式

### 2.1 必须支持（P0）

| 格式 | 扩展名 | 说明 | 库/实现 |
|------|--------|------|---------|
| ZIP | .zip | 最通用，各平台原生 | libarchive / zip-rs |
| 7-Zip | .7z | 高压缩比，开源 | libarchive / sevenz-rust |
| RAR | .rar | 流行但专有 | unrar（注意许可） |
| GZIP | .gz, .tgz | Unix 常用 | libarchive / flate2 |
| TAR | .tar, .tar.gz, .tar.bz2, .tar.xz | 组合格式 | libarchive |
| BZIP2 | .bz2 | Unix 常用 | libarchive / bzip2 |
| XZ | .xz | 高压缩比 | libarchive / xz2 |
| ZSTD | .zst, .tar.zst | 新格式，性能好 | libzstd |

### 2.2 可选支持（P1）

| 格式 | 扩展名 |
|------|--------|
| CAB | .cab |
| LZMA | .lzma |
| LZIP | .lz |
| AR | .a, .ar |
| ISO | .iso（只读） |
| XAR | .xar |
| LHA/LZH | .lha, .lzh |
| EGG | .egg（韩国格式） |
| ALZ | .alz（韩国格式） |

### 2.3 RAR 格式许可说明

- **UnRAR**：RARLAB 提供免费解压，**禁止用于创建 RAR 压缩器**，许可证与 GPL 不兼容
- **unrarlib**：GPL，仅支持 RAR2，开发已停止
- **方案**：使用 UnRAR 源码（解压 only）或 libarchive 的 RAR 读取支持；商业分发需遵守 UnRAR 许可

---

## 三、技术架构概览

```
┌─────────────────────────────────────────────────────────────────┐
│                     UI 层（按平台实现）                            │
│  Windows: WinUI / Qt  │  macOS: SwiftUI / Qt  │  Linux: Qt / GTK  │
└─────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────┐
│                      核心业务层（跨平台）                          │
│  • 智能解压逻辑  • 进度回调  • 错误处理  • 配置管理                 │
└─────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────┐
│                    解压引擎层（格式适配器）                         │
│  ZIP Adapter │ 7z Adapter │ RAR Adapter │ TAR/GZ/BZ2/XZ Adapter  │
└─────────────────────────────────────────────────────────────────┘
                                    │
┌─────────────────────────────────────────────────────────────────┐
│                    底层库（C/C++/Rust 实现）                       │
│  libarchive / zlib-ng / LZMA SDK / UnRAR / 各 Rust crate          │
└─────────────────────────────────────────────────────────────────┘
```

---

## 四、技术栈选型：C++ vs Rust

### 4.1 综合对比

| 维度 | C++ | Rust |
|------|-----|------|
| **性能** | 与 Rust 相当，依赖实现 | 与 C++ 相当；zlib-rs 等已超过 zlib-ng |
| **内存安全** | 需人工保证，易有悬空指针、UAF | 编译期保证，无数据竞争 |
| **构建与依赖** | CMake + vcpkg/conan，配置复杂 | Cargo 统一，依赖管理简单 |
| **跨平台** | 成熟，但需处理各平台差异 | 同样成熟，cargo 支持多 target |
| **格式库生态** | libarchive 等 C/C++ 库丰富 | 部分格式需 FFI 调用 C 库 |
| **学习曲线** | 团队通常更熟悉 | 较陡，需适应所有权模型 |
| **二进制体积** | 可精细控制 | 默认略大，可优化 |
| **Shell 集成** | 与平台 API 对接成熟 | 需 FFI 或独立进程 |

### 4.2 压缩/解压库生态

#### C++ 方案

| 库 | 格式支持 | 许可 | 说明 |
|----|----------|------|------|
| **libarchive** | tar, zip, 7z, rar, cpio, xar, iso, gz, bz2, xz, zstd... | BSD | 一站式，跨平台，维护活跃 |
| **7-Zip (LZMA SDK)** | 7z, zip, gz, bz2, xz... | LGPL | 高压缩比，需组合使用 |
| **zlib-ng** | deflate/inflate (zip 核心) | zlib | 比原版 zlib 快约 2x |
| **UnRAR** | RAR 解压 | 专有 | RARLAB 官方，仅解压 |
| **libzstd** | Zstandard | BSD | 高性能压缩 |

优势：libarchive 几乎覆盖所有格式，集成成本低。

#### Rust 方案

| 库 | 格式支持 | 许可 | 说明 |
|----|----------|------|------|
| **zip** | ZIP | MIT | 纯 Rust，维护活跃 |
| **sevenz-rust** | 7z | MIT | 纯 Rust，支持 LZMA2/ZSTD 等 |
| **tar** + **flate2** | tar.gz | MIT/Apache | 标准组合 |
| **xz2** | xz | MIT/Apache | 纯 Rust |
| **bzip2** | bz2 | MIT/Apache | 绑定 bzip2 C 库 |
| **libarchive-sys** | 全格式 | BSD | libarchive 的 FFI 绑定 |
| **rar** | RAR | ? | 需核实许可和实现 |
| **zlib-rs** | zlib/deflate | ? | 解压性能已超 zlib-ng |

优势：Cargo 生态统一；劣势：全格式覆盖需组合多个 crate 或依赖 libarchive FFI。

### 4.3 性能对比（2024 年情况）

- **zlib-rs**：解压性能已超过 zlib-ng（1KB 输入约 10%+，65KB 约 6%+）
- **libarchive**：流式、零拷贝设计，C 实现，性能稳定
- **综合**：纯 Rust 解压性能已达标；若要「一站全覆盖」，C++ + libarchive 集成最快

### 4.4 跨平台 Shell 集成

| 平台 | C++ | Rust |
|------|-----|------|
| **Windows** | COM Shell Extension、注册表 | 需 FFI 调用 Win32 API，或独立 C++ 扩展 |
| **macOS** | Automator / Finder Sync / 服务 | 同上，或 Swift/ObjC 封装 |
| **Linux** | Nautilus/Dolphin 脚本或扩展 | 脚本调用 CLI 即可 |

Rust 可专注核心逻辑，Shell 集成用各平台原生方式封装（如 Windows 用小型 C++ COM 组件）。

### 4.5 选型建议

| 场景 | 建议 |
|------|------|
| **快速落地、全格式支持** | **C++ + libarchive**：集成成本低，格式最全 |
| **长期维护、安全性优先** | **Rust**：内存安全、并发安全，适合核心引擎 |
| **混合架构** | **Rust 核心 + C FFI**：用 Rust 写业务和格式适配，通过 libarchive-sys 或独立 C 库处理特殊格式 |

---

## 五、推荐方案

### 方案 A：Rust 主导（推荐）

**核心用 Rust 实现，格式支持策略：**

1. **纯 Rust 优先**：ZIP（zip）、7z（sevenz-rust）、tar.gz（tar+flate2）、xz（xz2）、zstd 等
2. **FFI 补全**：RAR 等通过 libarchive-sys 或 UnRAR 绑定
3. **CLI 先行**：先做跨平台命令行，再为各平台做 GUI 与 Shell 集成

**理由：**

- 内存安全、并发安全，降低崩溃与漏洞风险
- Cargo 简化构建、测试、发布
- 解压性能已不逊于 C/C++
- 团队若熟悉 Rust，长期维护成本更低

### 方案 B：C++ + libarchive

**适用于：**

- 团队 C++ 经验丰富，Rust 储备不足
- 需要最短时间内支持全部格式
- 不强调 Rust 生态与工具链

**实现要点：**

- 使用 libarchive 作为唯一解压后端
- 用 CMake + vcpkg 管理依赖
- 各平台分别实现 Shell 集成与 GUI

---

## 六、开发阶段建议

### Phase 1：MVP（2–3 个月）

- [ ] 选定技术栈（Rust 或 C++）
- [ ] 实现 CLI：`fastzip extract <file> [--dest dir] [--smart]`
- [ ] 支持 ZIP、7z、tar.gz、tar.xz、gz、xz、zstd
- [ ] 实现「智能解压」逻辑
- [ ] 多线程解压（大文件/多文件）

### Phase 2：平台集成（2–3 个月）

- [ ] Windows 右键菜单
- [ ] macOS Finder 集成
- [ ] Linux 文件管理器集成
- [ ] 简易 GUI（可选）

### Phase 3：完善（1–2 个月）

- [ ] RAR 支持（含许可合规）
- [ ] 更多格式（CAB、ISO 等）
- [ ] 加密压缩包、Unicode 路径
- [ ] 性能优化与稳定性测试

---

## 七、总结

| 技术栈 | 优势 | 劣势 |
|--------|------|------|
| **Rust** | 安全、现代工具链、解压性能优秀、易维护 | 格式需组合多个 crate，Shell 集成需额外工作 |
| **C++** | 格式库齐全（尤其 libarchive）、集成快 | 内存安全需人工保证，构建配置相对复杂 |

**最终建议**：若团队愿意投入 Rust 学习，优先采用 **Rust 主导** 方案；若以快速上线和全格式覆盖为首要目标，可选用 **C++ + libarchive**。两种方案均可实现与 Bandizip 类似的跨平台快速解压体验。
