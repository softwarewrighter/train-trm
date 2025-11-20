#!/bin/bash
# Clean build artifacts and temporary files

set -e

echo "=== Cleaning Project ==="
echo ""

echo "Removing build artifacts..."
cargo clean

echo "Removing temporary model files..."
rm -f *.trm
rm -f test_model*.trm
rm -f trained_model.trm

echo ""
echo "=== Clean complete ==="
