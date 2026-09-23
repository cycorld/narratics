# AGENTS.md — Narratics Development Guidelines

## Project Identity
- **Project:** Narratics (내러틱스)
- **Domain:** `narratics.com`
- **Core Architecture:** Local-First Novel & Worldbuilding Engine (Rust AOT `engine-core` + yrs + SQLite WAL `.narr`)

## Invariants & Rules
1. **Zero-Network Invariant:** `crates/engine-core` must never assume network availability. Every read, write, snapshot, and search must be 100% executable offline.
2. **UTF-16 Indexing:** Always initialize `yrs::Doc` with `yrs::Options { offset_kind: yrs::OffsetKind::Utf16, .. }`. Never switch to UTF-8 byte indexing to avoid Korean/emoji clipping crashes.
3. **Acyclic Tree Invariant:** All tree mutations must pass through Kleppmann Tree Move CRDT with cycle prevention.
4. **Lock Order Safety:** Always access cached `TextRef` or acquire `Text` handles before entering read transactions (`doc.transact()`) to prevent `RwLock` re-entrancy deadlocks.
5. **Zero-Defect Verification:** Any change to `engine-core` must pass all 10 cycles of `tests/adversarial_hardening.rs` before landing.
