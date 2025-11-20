#!/bin/bash
# Build the web UI for production

set -e

echo "=== Building TRM Web UI for Production ==="
echo ""

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "Error: trunk is not installed"
    echo "Install with: cargo install --locked trunk"
    exit 1
fi

# Check if wasm32-unknown-unknown target is installed
if ! rustup target list | grep "wasm32-unknown-unknown (installed)" &> /dev/null; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

echo "Building for production..."
trunk build --release

echo ""
echo "=== Build Complete ==="
echo "Output directory: dist/"
echo ""
echo "To serve the built files, use:"
echo "  python3 -m http.server -d dist 8080"
echo "  or"
echo "  cd dist && http-server"
