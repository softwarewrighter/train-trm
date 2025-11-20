#!/bin/bash
# Format all Rust code in the project

set -e

echo "=== Formatting Rust Code ==="
echo ""

cargo fmt

echo ""
echo "=== Formatting complete ==="
