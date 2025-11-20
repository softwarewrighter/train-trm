#!/bin/bash
# Build the TRM project

set -e

# Parse command line arguments
RELEASE=""
while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            RELEASE="--release"
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --release    Build in release mode (optimized)"
            echo "  -h, --help   Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "=== Building TRM Project ==="
if [ -n "$RELEASE" ]; then
    echo "Build mode: Release (optimized)"
else
    echo "Build mode: Debug"
fi
echo ""

cargo build $RELEASE

echo ""
echo "=== Build complete ==="
if [ -n "$RELEASE" ]; then
    echo "Binary: target/release/train-trm"
else
    echo "Binary: target/debug/train-trm"
fi
