# Contributing to Narratics

Thank you for your interest in contributing to Narratics!

## Core Invariants

Before submitting code, ensure that:
1. **Zero-Network Invariant:** `crates/engine-core` must never assume network connectivity. All operations must function 100% offline.
2. **UTF-16 Indexing:** Always preserve UTF-16 character boundaries for TipTap/ProseMirror compatibility.
3. **Cycle-Free Tree Invariant:** All binder tree mutations must go through Kleppmann Tree Move CRDT.
4. **Adversarial Hardening:** All 13 stress tests in `crates/engine-core/tests/adversarial_hardening.rs` must pass without panics.
5. **Zero-Defect Regression Gate:** The automated E2E browser UX test (`./scripts/regression_gate.sh`) must pass with 0 failures.

## Development Workflow

1. Fork the repository and create your feature branch:
   ```bash
   git checkout -b feature/my-feature
   ```
2. Make your changes and verify with the regression gate:
   ```bash
   ./scripts/regression_gate.sh
   ```
3. Commit using [Conventional Commits](https://www.conventionalcommits.org/):
   ```bash
   git commit -m "feat(studio): add new distraction-free focus mode"
   ```
4. Push to your fork and submit a Pull Request.

## Licensing Note

By contributing to Narratics, you agree that your contributions will be licensed under the PolyForm Noncommercial License 1.0.0.
