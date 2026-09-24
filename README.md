<div align="center">

# 📖 Narratics

**High-Performance Local-First Novel & Worldbuilding Studio and CRDT Engine**

[![License: PolyForm Noncommercial 1.0.0](https://img.shields.io/badge/License-PolyForm%20Noncommercial%201.0.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](Cargo.toml)
[![Zero-Defect](https://img.shields.io/badge/Adversarial%20Hardening-13%2F13%20PASS-brightgreen.svg)](crates/engine-core/tests/adversarial_hardening.rs)
[![E2E UX Gate](https://img.shields.io/badge/E2E%20UX%20Gate-5%2F5%20PASS-brightgreen.svg)](tests/e2e_ux_automated_gate.py)
[![Platforms](https://img.shields.io/badge/Platform-Web%20%7C%20macOS%20%7C%20Windows%20%7C%20Linux%20%7C%20iOS%20%7C%20Android-lightgrey.svg)](#architecture)

**[ English ](README.md)** • **[ 한국어 ](README.ko.md)**

</div>

---

## 1. Vision & Core Philosophy

Narratics is a production-grade, local-first novel writing studio and worldbuilding engine engineered to solve the reliability, synchronization, and formatting bottlenecks of traditional narrative software (e.g. Scrivener, Notion, Google Docs).

- **100% Local-First (Zero-Network Invariant):** All writing, worldbuilding lore, chapter organization, and snapshot versioning operate directly against a single-file `.narr` SQLite WAL container. Zero network calls or cloud accounts required.
- **Cycle-Free Tree CRDT (Kleppmann Tree Move):** Arbitrary reparenting, chapter reordering, and multi-device tree divergence deterministically converge without orphan nodes or circular dependency panics.
- **UTF-16 Indexing (yrs Text Engine):** Eliminates multi-byte clipping bugs in 3-byte Korean characters, astral plane symbols, and 4-byte emojis across Web (TipTap), Desktop (Tauri), and Mobile (Swift/Kotlin) surfaces.
- **Single-File SQLite WAL Container (`.narr`):** Replaces fragile directory-bundle formats (`.scriv`) with atomic, zero-corruption SQLite WAL storage supporting 1M+ character manuscripts.
- **Automated Ergonomics & Zero-Manual-QA:** Built-in 45% eye-level typewriter scroll locking, `@` mention lore autocomplete, corkboard matrix synchronization, and split-view reference inspection.

---

## 2. Monorepo Architecture

```
narratics/
├── Cargo.toml                       # Workspace root & package manifest
├── LICENSE                          # PolyForm Noncommercial License 1.0.0
├── apps/
│   └── desktop/                     # Tauri v2 Native Desktop Application (macOS/Win/Linux)
│       ├── Cargo.toml
│       ├── tauri.conf.json
│       ├── src/                     # Native IPC commands & desktop state
│       └── ui/                      # Zero-divergence local web studio shell
├── crates/
│   ├── engine-core/                 # Pure offline Rust CRDT engine & container
│   │   ├── src/tree_crdt.rs         # Kleppmann Tree Move CRDT
│   │   ├── src/text_engine.rs       # yrs UTF-16 text editing & tombstone compaction
│   │   └── src/container.rs         # .narr single-file SQLite WAL persistence
│   ├── backend-sync/                # Axum AOT sync server & responsive Web Studio
│   │   ├── src/app_state.rs         # Multi-book library, CRUD & Typst PDF compiler
│   │   ├── src/web_ui.rs            # TipTap editor, corkboard, lore mentions & split-view
│   │   ├── src/landing_ui.rs        # Production landing page & downloads
│   │   └── tests/regression_suite.rs# 5-test comprehensive backend regression suite
│   └── mobile-bridge/               # C-FFI / JNI / Swift Native Mobile Bridge
│       ├── bindings/android/        # Kotlin JNI bindings
│       └── bindings/ios/            # Swift bridge interface
├── docs/                            # Verified screenshots, benchmarks & specs
├── scripts/
│   ├── regression_gate.sh           # Full zero-defect build, test & E2E gate script
│   └── sync_desktop_ui.py           # Auto-synchronizes web studio to desktop shell
└── tests/
    └── e2e_ux_automated_gate.py     # Headless browser E2E interaction gate
```

---

## 3. Adversarial Hardening (13-Axis Stress Suite)

Every release candidate is strictly subjected to a 13-axis adversarial hardening suite (`crates/engine-core/tests/adversarial_hardening.rs`), ensuring zero-defect stability under extreme conditions:

| Axis | Scenario & Invariant | Verdict | Execution Time |
| :--- | :--- | :---: | :---: |
| **Cycle 1: Offline Invariant** | Total network partition during heavy writes, crash restart, snapshot preservation | **PASS** | 0.05s |
| **Cycle 2: Boundary & Negative** | Empty doc deletion, negative offsets, out-of-bound text slicing | **PASS** | 0.00s |
| **Cycle 3: Filesystem Hostility** | Special characters, nested path traversal, directory permissions friction | **PASS** | 0.05s |
| **Cycle 4: Deep Tree Cycle** | 50-level circular ancestor reparenting attempts blocked deterministically | **PASS** | 0.00s |
| **Cycle 5: Poison CRDT Payload** | Malformed byte streams and truncated wire packets rejected gracefully | **PASS** | 0.06s |
| **Cycle 6: Korean & Astral Emoji**| 3-byte Hangul + 4-byte emoji boundary slicing without panic | **PASS** | 0.00s |
| **Cycle 7: 4-Peer Convergence** | 4-device concurrent edits and chapter movements converge to identical tree | **PASS** | 0.00s |
| **Cycle 8: Tombstone Compaction**| 200 consecutive deletes/edits with 1.1ms bounded memory compaction | **PASS** | 0.01s |
| **Cycle 9: SQLite WAL Contention**| Multi-connection concurrency, transaction isolation, atomic rollback | **PASS** | 0.01s |
| **Cycle 10: Schema Parity** | Lossless JSON / Binary wire-format roundtrip fidelity | **PASS** | 0.00s |
| **Cycle 11: 1M-Char Stress Load** | 1,000,000-character manuscript streaming without latency spikes | **PASS** | 0.02s |
| **Cycle 12: Injection Defense** | Malicious lore IDs and SQL injection attempts neutralized | **PASS** | 0.01s |
| **Cycle 13: Trash Quarantine** | Deleted chapter/scene safe quarantine and isolation invariant | **PASS** | 0.00s |

---

## 4. Automated E2E Browser Interaction Gate

To eliminate human manual QA fatigue, `tests/e2e_ux_automated_gate.py` runs real browser DOM and keyboard event evaluations:

1. **`@` Mention Autocomplete Navigation:** Simulates `@` keystroke, `ArrowDown` highlight switching, and `Enter` token insertion.
2. **Typewriter Mode Vertical Locking:** Measures cursor vertical coordinates across 30 lines of typing to verify eye-level 45% ($\pm 5\%$) lock.
3. **Cross-Chapter Scene Reparenting:** Verifies dragging and inspector dropdown reparenting with SQLite WAL roundtrip.
4. **Corkboard Active Synchronization:** Validates chapter switching and active card highlighting when navigating sidebar items.
5. **Reversible Undo Notification:** Validates 6-second timeout toast notification and undo callbacks.

---

## 5. Quick Start & Development

### Prerequisites
- **Rust:** `1.80+` (stable)
- **Python:** `3.10+` (for E2E browser tests)
- **Typst:** (Optional, for PDF book typesetting export)

### Build & Test

```bash
# Clone the repository
git clone https://github.com/cycorld/narratics.git
cd narratics

# Check workspace compilation
cargo check --workspace

# Run complete 22-unit test suite + adversarial hardening
cargo test --workspace -- --nocapture

# Run full zero-defect regression gate (includes E2E UX tests)
./scripts/regression_gate.sh
```

### Launching the Studio Server

```bash
# Build and run the Axum web studio locally
cargo run --release -p narratics-backend-sync

# Open http://localhost:3901/ in your browser
```

---

## 6. License & Commercial Inquiries

This project is licensed under the **[PolyForm Noncommercial License 1.0.0](LICENSE)**.

### Summary of Terms:
- **Personal & Noncommercial Use:** **100% Free and Unrestricted.** You may freely use, modify, study, run, and distribute Narratics for personal projects, open-source research, education, hobbyist writing, and amateur creative pursuits.
- **Commercial Use & Commercial Services:** Any use for commercial purposes (including but not limited to commercial SaaS hosting, enterprise licensing, or paid subscription services) requires a separate commercial license agreement.

For commercial licensing, enterprise deployment, or partnership inquiries, please contact:
- **Charles Choi** (`cycorld@martian.link`)
- **Entropy Paradox, Inc.** (엔트로피패러독스 주식회사)
