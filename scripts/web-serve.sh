#!/bin/bash
# Serve the web UI locally

set -e

echo "=== Building and Serving TRM Web UI ==="
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

echo "Starting development server..."
echo "The web UI will be available at: http://127.0.0.1:1421"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

trunk serve --port 1421 --address 127.0.0.1 --open
