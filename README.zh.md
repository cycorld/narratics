<div align="center">

# 📖 Narratics

**直至终章坚若磐石的叙事蓝图 — 高性能 Local-First 长篇小说创作工作室与 CRDT 引擎**

[![License: PolyForm Noncommercial 1.0.0](https://img.shields.io/badge/License-PolyForm%20Noncommercial%201.0.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](Cargo.toml)
[![Zero-Defect](https://img.shields.io/badge/Adversarial%20Hardening-13%2F13%20PASS-brightgreen.svg)](crates/engine-core/tests/adversarial_hardening.rs)
[![E2E UX Gate](https://img.shields.io/badge/E2E%20UX%20Gate-5%2F5%20PASS-brightgreen.svg)](tests/e2e_ux_automated_gate.py)
[![Platforms](https://img.shields.io/badge/Platform-Web%20%7C%20macOS%20%7C%20Windows%20%7C%20Linux%20%7C%20iOS%20%7C%20Android-lightgrey.svg)](#architecture)

**[ English ](README.md)** • **[ 한국어 ](README.ko.md)** • **[ 日本語 ](README.ja.md)** • **[ 简体中文 ](README.zh.md)**

</div>

---

## 1. 愿景与核心理念

Narratics 是一款面向生产环境设计的**本地优先（Local-First）长篇小说创作工作室与世界观引擎**，专为解决传统创作工具（如 Scrivener、Notion、Google Docs）在多设备同步不稳定、排版损坏和厂商锁定等痛点而设计。

- **100% 本地优先（Zero-Network Invariant）：** 无需依赖外部互联网或云端账户，仅凭单个 `.narr` SQLite WAL 文件即可全离线驱动写作、世界观设定库与快照管理。
- **防循环树形 CRDT（Kleppmann Tree Move）：** 章节与场景的任意重定父级（Reparenting）、重排序及多端离线冲突，均能确定性收敛，杜绝孤立节点与循环死锁。
- **UTF-16 索引驱动 yrs 文本引擎：** 在 Web (TipTap)、桌面 (Tauri) 及移动原生端之间彻底解决 3 字节汉字、特殊生僻字与 4 字节 Emoji 截断崩溃缺陷。
- **单文件 SQLite WAL 容器（`.narr`）：** 替代传统脆弱的文件夹打包格式（如 `.scriv`），保障百万字长篇巨作的不可逆事务持久性与零损坏。
- **专注人机工学写作体验：** 内置 45% 视线打字机居中滚动、`@` 快捷世界观引用补全、软木板分镜联动与双栏参考对照。

---

## 2. 代码库架构

```
narratics/
├── Cargo.toml                       # 工作区配置
├── LICENSE                          # PolyForm Noncommercial License 1.0.0
├── apps/
│   └── desktop/                     # Tauri v2 桌面客户端 (macOS/Win/Linux)
├── crates/
│   ├── engine-core/                 # 离线 Rust CRDT 引擎与存储容器
│   │   ├── src/tree_crdt.rs         # Kleppmann Tree Move CRDT
│   │   ├── src/text_engine.rs       # yrs UTF-16 文本编辑与 Tombstone 压缩
│   │   └── src/container.rs         # .narr 单文件 SQLite WAL 持久化
│   ├── backend-sync/                # Axum 同步服务器与 Web Studio
│   │   ├── src/app_state.rs         # 多项目书库管理与 Typst 导出
│   │   └── src/web_ui.rs            # TipTap 编辑器、分镜软木板与设定引用
│   └── mobile-bridge/               # iOS / Android C-FFI 绑定
```

---

## 3. 快速上手

### 环境要求
- **Rust:** 1.80 或更高版本 (`cargo`)
- **Typst (可选):** 用于出版级排版 PDF 编译 (`typst`)

### 编译与运行

```bash
# 1. 克隆代码仓库
git clone https://github.com/cycorld/narratics.git
cd narratics

# 2. 运行全量测试与防御性抗压门禁
cargo test --workspace

# 3. 启动 Web 创作工作室服务
cargo run -p narratics-backend-sync
# 浏览器访问 http://127.0.0.1:3901

# 4. 启动原生桌面应用
cd apps/desktop
cargo tauri dev
```

---

## 4. 开源许可协议 (License)

本项目采用 **PolyForm Noncommercial License 1.0.0** 许可证发布。

- **个人与非商业用途:** 完全免费开源（包括学习、二次开发、个人创作使用）。
- **商业用途（付费 SaaS 服务、商业销售、企业级部署）:** 须签署商业授权协议。
- **商业授权咨询:** `cycorld@martian.link`
