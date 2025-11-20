#!/bin/bash
# Run all tests for the TRM project

set -e

echo "=== Running TRM Tests ==="
echo ""

echo "Running unit tests..."
cargo test

echo ""
echo "Running clippy checks..."
cargo clippy -- -D warnings

echo ""
echo "Checking code formatting..."
cargo fmt -- --check

echo ""
echo "=== All tests passed! ==="
