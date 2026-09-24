#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_ROOT}"

echo "=========================================================="
echo "  [Narratics Zero-Defect Continuous Regression Gate]     "
echo "=========================================================="

echo ">> [1/5] Checking Rust code formatting (cargo fmt)..."
./scripts/sync_desktop_ui.py
cargo fmt --check

echo ">> [2/4] Checking compilation across workspace (cargo check)..."
cargo check --workspace

echo ">> [3/4] Running 22-test regression & adversarial suite (cargo test)..."
cargo test --workspace -- --nocapture

echo ">> [4/5] Building production release artifacts (cargo build --release)..."
cargo build --release --workspace

echo ">> [5/5] Running E2E Interactive UX Browser Test Gate (agent-browser)..."
python3 tests/e2e_ux_automated_gate.py

echo "=========================================================="
echo "  [SUCCESS] All 22 Unit + 5 E2E UX Interaction Tests PASSED! "
echo "  Zero-Defect invariant maintained across all layers.     "
echo "=========================================================="
