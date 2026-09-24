## Summary of Changes

<!-- Describe what this PR introduces, fixes, or optimizes. -->

## Type of Change

- [ ] 🐛 Bug fix (non-breaking change which fixes an issue)
- [ ] ⚡ New feature (corkboard, typewriter mode, mention palette, exporter)
- [ ] 🚀 Performance optimization (CRDT snapshotting, memory footprint)
- [ ] 🛡️ Security / Zero-PII sanitization
- [ ] 📝 Documentation update (i18n, guides, architecture specs)
- [ ] 🧪 Tests (automated UX gate, regression tests, unit tests)

## Invariant Checklist

- [ ] **Zero-Network Invariant**: `crates/engine-core` has zero network or socket dependencies.
- [ ] **Deterministic Merge**: All CRDT state merges produce deterministic results regardless of arrival order.
- [ ] Code follows formatting rules (`cargo fmt --all -- --check`).
- [ ] Clippy checks pass (`cargo clippy --workspace --all-targets --all-features -- -D warnings`).
- [ ] Workspace tests pass (`cargo test --workspace`).
- [ ] Regression gate passes (`./scripts/regression_gate.sh`).
- [ ] Conventional Commit format used (`feat(...)`, `fix(...)`, `chore(...)`, `docs(...)`).
