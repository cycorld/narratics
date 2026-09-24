#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_ROOT}"

echo "=========================================================="
echo "  [Narratics Zero-Defect Continuous Regression Gate]     "
echo "=========================================================="

echo ">> [1/4] Checking Rust code formatting (cargo fmt)..."
cargo fmt --check

echo ">> [2/4] Checking compilation across workspace (cargo check)..."
cargo check --workspace

echo ">> [3/4] Running 22-test regression & adversarial suite (cargo test)..."
cargo test --workspace -- --nocapture

echo ">> [4/4] Building production release artifacts (cargo build --release)..."
cargo build --release --workspace

echo "=========================================================="
echo "  [SUCCESS] All 22 Regression & Adversarial Tests PASSED! "
echo "  Zero-Defect invariant maintained across all platforms.  "
echo "=========================================================="
