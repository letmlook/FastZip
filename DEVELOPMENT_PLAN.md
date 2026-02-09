# FastZip 开发计划 - Rust 方案 A

> 按方案 A（Rust 主导）制定的详细开发计划，包含项目结构、依赖选型、阶段拆分与里程碑。

---

## 一、项目结构

```
fastzip/
├── Cargo.toml                 # 工作空间配置
├── DESIGN.md                  # 设计方案
├── DEVELOPMENT_PLAN.md        # 本开发计划
├── README.md
│
├── crates/
│   ├── fastzip-core/          # 核心库：解压逻辑、智能解压、格式适配
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── extractor/     # 解压引擎
│   │       ├── smart_dest/    # 智能解压目标决策
│   │       ├── formats/       # 各格式适配器 (zip, 7z, tar, etc.)
│   │       └── error.rs
│   │
│   ├── fastzip-cli/           # 命令行工具
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       └── args.rs
│   │
│   ├── fastzip-shell-win/     # Windows 右键菜单集成（可选：独立 crate）
│   │   └── ...
│   │
│   └── fastzip-gui/           # 主程序 GUI（P1，后续）
│       └── ...
│
├── assets/                    # 图标、资源
└── tests/                     # 集成测试
```

---

## 二、依赖选型（Cargo.toml）

### fastzip-core

```toml
[dependencies]
# 格式支持
zip = "0.6"                   # ZIP
sevenz-rust = "0.6"           # 7z
tar = "0.4"                   # TAR 容器
flate2 = "1.0"                # gzip (tar.gz)
xz2 = "0.1"                   # xz (tar.xz, .xz)
bzip2 = "0.4"                 # bzip2 (tar.bz2, .bz2)
zstd = "0.13"                 # zstd (.zst, tar.zst)
libarchive-sys = { version = "0.2", optional = true }  # RAR/CAB/ISO 等

# 工具
walkdir = "2"                 # 目录遍历（智能解压预扫描）
thiserror = "2"               # 错误类型
anyhow = "1"                  # 错误传播
tracing = "0.1"               # 日志
rayon = "1.10"                # 并行解压

[features]
default = []
full = ["libarchive-sys"]     # 启用 RAR 等全格式
```

### fastzip-cli

```toml
[dependencies]
fastzip-core = { path = "../fastzip-core" }
clap = { version = "4", features = ["derive"] }
indicatif = "0.17"            # 进度条
console = "0.15"              # 终端输出
```

---

## 三、开发阶段与里程碑

### Phase 1：核心库与 CLI MVP（约 8–10 周）

**目标**：可用的命令行解压工具，支持主流格式，具备智能解压逻辑。

| 周次 | 任务 | 产出 |
|------|------|------|
| **W1** | 项目初始化、目录结构、基础 crate 骨架 | `cargo build` 通过 |
| **W2** | 实现 ZIP 解压（zip crate） | 支持 .zip |
| **W3** | 实现 7z 解压（sevenz-rust） | 支持 .7z |
| **W4** | 实现 tar.* 解压（tar + flate2/xz2/bzip2/zstd） | 支持 .tar.gz, .tar.xz, .tar.bz2, .tar.zst |
| **W5** | 实现纯压缩格式（.gz, .xz, .bz2, .zst） | 支持单文件压缩格式 |
| **W6** | 智能解压逻辑：单文件/单根文件夹/多文件分支 | `--smart` 行为正确 |
| **W7** | 智能解压：重名文件夹 (2)、(3) 等 | 目标路径无冲突 |
| **W8** | CLI 完善：`extract` / `x` 子命令，`--dest`、`--smart`、进度输出 | 完整 CLI |
| **W9** | 并行解压（rayon，多文件场景） | 大包解压加速 |
| **W10** | 单元测试、集成测试、错误处理与日志 | MVP 发布就绪 |

**里程碑 M1**：`fastzip x archive.zip --smart` 可在三大平台正确解压主流格式。

---

### Phase 2：平台 Shell 集成（约 6–8 周）

**目标**：在 Windows、macOS、Linux 上提供右键菜单等快捷入口。

| 周次 | 任务 | 产出 |
|------|------|------|
| **W11** | Windows：注册表添加上下文菜单，调用 `fastzip x <path>` | 右键可解压 |
| **W12** | Windows：智能解压菜单项，传递 `--smart` | 智能解压入口 |
| **W13** | Windows：安装/卸载脚本或 installer 集成 | 一键安装 |
| **W14** | macOS：Finder 服务 / Automator 调用 CLI | 右键可解压 |
| **W15** | Linux：Nautilus/Dolphin 脚本或 .desktop 集成 | 右键可解压 |
| **W16** | 各平台路径、编码、权限处理 | 跨平台一致性 |
| **W17–18** | 文档、发布流程、CI/CD | 可分发安装包 |

**里程碑 M2**：用户安装后可通过右键菜单进行解压与智能解压。

> 已实现：scripts/install-windows.ps1、install-macos.sh、install-linux.sh；.github/workflows/ci.yml、release.yml

---

### Phase 3：格式扩展与完善（约 4–6 周）

**目标**：支持 RAR 及更多格式，加密、Unicode、性能与稳定性优化。

| 周次 | 任务 | 产出 |
|------|------|------|
| **W19** | RAR：集成 libarchive-sys 或 unrar 绑定，许可合规 | 支持 .rar |
| **W20** | 密码保护：ZIP/7z 加密解压，交互式/环境变量密码 | 加密包可解压 |
| **W21** | Unicode/UTF-8 文件名、路径规范化 | 中文等路径正确 |
| **W22** | 可选格式：CAB、ISO（只读）| `full` feature |
| **W23** | 性能测试、内存与崩溃边界测试 | 稳定性报告 |
| **W24** | 文档、CHANGELOG、版本发布 | v1.0 候选 |

**里程碑 M3**：支持 RAR、加密包，具备生产可用稳定性。

---

### Phase 4（可选）：GUI 与高级功能

- 主程序 GUI（预览、选择性解压、管理）
- 压缩功能（ZIP、7z 等）
- 更多 P1 格式（LHA、EGG、ALZ 等）

---

## 四、详细任务说明

### 4.1 Phase 1 关键实现

#### 智能解压逻辑（smart_dest）

```text
输入：archive_path, base_dir, 预扫描条目列表
输出：dest_dir

1. 预扫描：列出顶层条目（不含递归子路径）
2. 若只有 1 个文件 → return base_dir
3. 若所有条目共享同一根目录 → return base_dir
4. 否则：
   - name = archive_stem (去掉扩展名)
   - 若 base_dir/name 不存在 → return base_dir/name
   - 否则找最小 k 使 base_dir/name (k) 不存在 → return base_dir/name (k)
```

#### 格式检测

- 按扩展名优先：.zip → zip, .7z → 7z, .tar.gz/.tgz → tar+flate2, ...
- 备选：魔数检测（ZIP: PK\x03\x04, 7z: 7z\xBC\xAF\x27\x1C, gzip: \x1f\x8b, ...）

#### CLI 接口

```bash
fastzip extract <archive> [options]
fastzip x <archive> [options]

选项:
  -d, --dest <dir>    指定解压目标目录
  -s, --smart         智能解压（默认）
  -f, --flat          解压到此处（等同于 Bandizip 的普通解压）
  -o, --overwrite     覆盖已存在文件
  -q, --quiet         静默模式
  -p, --password      密码（或通过 FASTZIP_PASSWORD 环境变量）
```

### 4.2 Phase 2 平台集成要点

| 平台 | 实现方式 |
|------|----------|
| **Windows** | 注册表 `HKCU\Software\Classes\*\shell\FastZip`，command 调用 `fastzip x "%1" --smart` |
| **macOS** | Automator 服务 或 `.workflow`，调用 `/usr/local/bin/fastzip` |
| **Linux** | `~/.local/share/file-manager/actions/` 或 `.desktop` 文件，调用 `fastzip` |

### 4.3 Phase 3 RAR 与加密

- **RAR**：优先用 `libarchive-sys`（BSD），若功能不足再评估 `unrar` 源码或 `unrar-rs` 类 crate
- **加密**：`zip` crate 支持 AES；`sevenz-rust` 支持 AES256；需实现密码输入（tty、环境变量、`--password`）

---

## 五、时间线总览

| Phase | 时长 | 里程碑 |
|-------|------|--------|
| Phase 1 | 8–10 周 | CLI MVP，主流格式 + 智能解压 |
| Phase 2 | 6–8 周 | 三大平台右键菜单集成 |
| Phase 3 | 4–6 周 | RAR、加密、稳定性，v1.0 候选 |
| **合计** | **约 5–6 个月** | |

---

## 六、风险与应对

| 风险 | 应对 |
|------|------|
| 某格式 Rust crate 不稳定 | 备选 libarchive-sys 统一兜底 |
| Windows Shell 集成权限问题 | 提供用户级注册表方案，避免需管理员 |
| RAR 许可复杂 | 使用 libarchive 的 RAR 读取，或延迟至 Phase 3 单独评估 |

---

## 七、验收标准

- [ ] `fastzip x <file> --smart` 在 Windows/macOS/Linux 上正确解压 ZIP、7z、tar.gz、tar.xz、tar.bz2、tar.zst、gz、xz、bz2、zst
- [ ] 智能解压三种分支（单文件/单根目录/多文件）行为与 Bandizip 一致
- [ ] 重名文件夹自动使用 (2)、(3) 等后缀
- [ ] 安装后在目标平台可通过右键菜单完成解压
- [ ] 无已知内存安全漏洞，通过基本模糊测试或压力测试
