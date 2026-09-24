<div align="center">

# 📖 Narratics

**完結まで揺るがない物語の設計図 — 高性能ローカルファースト小説執筆スタジオ＆CRDTエンジン**

[![License: PolyForm Noncommercial 1.0.0](https://img.shields.io/badge/License-PolyForm%20Noncommercial%201.0.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](Cargo.toml)
[![Zero-Defect](https://img.shields.io/badge/Adversarial%20Hardening-13%2F13%20PASS-brightgreen.svg)](crates/engine-core/tests/adversarial_hardening.rs)
[![E2E UX Gate](https://img.shields.io/badge/E2E%20UX%20Gate-5%2F5%20PASS-brightgreen.svg)](tests/e2e_ux_automated_gate.py)
[![Platforms](https://img.shields.io/badge/Platform-Web%20%7C%20macOS%20%7C%20Windows%20%7C%20Linux%20%7C%20iOS%20%7C%20Android-lightgrey.svg)](#architecture)

**[ English ](README.md)** • **[ 한국어 ](README.ko.md)** • **[ 日本語 ](README.ja.md)** • **[ 简体中文 ](README.zh.md)**

</div>

---

## 1. ビジョンと基本理念

Narratics（ナラティクス）は、従来の物語執筆ツール（Scrivener、Notion、Google Docsなど）の同期不安定、書式崩れ、ベンダーロックインを克服するために設計された**本番対応ローカルファースト長編小説執筆スタジオ＆世界観設定エンジン**です。

- **100% ローカルファースト (Zero-Network Invariant):** 外部インターネット接続やクラウド認証を一切必要とせず、単一の `.narr` SQLite WAL コンテナファイルのみで全執筆、世界観設定、スナップショット履歴が自律動作します。
- **循環防止ツリーCRDT (Kleppmann Tree Move):** フォルダやシーンの任意の再階層化、並べ替え、マルチデバイス同期時に循環参照（Cycle）を根本的に遮断し、決定論的に収束します。
- **UTF-16 インデックス対応 yrs テキストエンジン:** Web (TipTap)、Desktop (Tauri)、Mobile (Swift/Kotlin) 間で、3バイト日本語文字、特殊グリフ、4バイト絵文字の文字化け・切り落としクラッシュを完全に排除しました。
- **単一ファイル SQLite WAL コンテナ (`.narr`):** 壊れやすいディレクトリ束形式（`.scriv`）を廃止し、100万字を超える長編でも破損のない不可逆なトランザクション整合性を保証します。
- **作家工学エルゴノミクス:** 視線高さを45%に固定するタイプライタースクロール、`@` メンション世界観補完、コークボード同期、スプリットビュー参照を標準装備しています。

---

## 2. モノレポ・アーキテクチャ

```
narratics/
├── Cargo.toml                       # ワークスペース定義
├── LICENSE                          # PolyForm Noncommercial License 1.0.0
├── apps/
│   └── desktop/                     # Tauri v2 デスクトップアプリ (macOS/Win/Linux)
├── crates/
│   ├── engine-core/                 # オフライン Rust CRDT エンジン & コンテナ
│   │   ├── src/tree_crdt.rs         # Kleppmann Tree Move CRDT
│   │   ├── src/text_engine.rs       # yrs UTF-16 編集 & トゥームストーン圧縮
│   │   └── src/container.rs         # .narr 単一 SQLite WAL 永続化
│   ├── backend-sync/                # Axum 同期サーバー & Web Studio
│   │   ├── src/app_state.rs         # 複数作品管理 & Typst PDF 出力
│   │   └── src/web_ui.rs            # TipTap エディタ, コークボード, 設定メンション
│   └── mobile-bridge/               # iOS / Android C-FFI バインディング
```

---

## 3. クイックスタート

### 動作環境
- **Rust:** 1.80 以上 (`cargo`)
- **Typst (任意):** 出版品質 PDF レンダリング用 (`typst`)

### ビルドと実行

```bash
# 1. リポジトリのクローン
git clone https://github.com/cycorld/narratics.git
cd narratics

# 2. 回帰テスト & 敵対的ストレステストの実行
cargo test --workspace

# 3. Web スタジオ サーバーの起動
cargo run -p narratics-backend-sync
# ブラウザで http://127.0.0.1:3901 にアクセス

# 4. デスクトップ アプリケーションの起動
cd apps/desktop
cargo tauri dev
```

---

## 4. ライセンス (License)

本プロジェクトは **PolyForm Noncommercial License 1.0.0** の下で公開されています。

- **個人および非営利目的の利用:** 完全無償（改変・学習・個人作品執筆を含む）。
- **商用利用（有償SaaS、企業導入、商用販売）:** 別途商用ライセンス契約が必要です。
- **商用ライセンスのお問い合わせ:** `cycorld@martian.link`
