#!/bin/bash
# Train a TRM model with default or custom parameters

set -e

# Default parameters
EPOCHS=1000
LEARNING_RATE=0.01
LAYERS=2
H_CYCLES=3
L_CYCLES=4
OUTPUT="model.trm"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --epochs)
            EPOCHS="$2"
            shift 2
            ;;
        --lr)
            LEARNING_RATE="$2"
            shift 2
            ;;
        --layers)
            LAYERS="$2"
            shift 2
            ;;
        --h-cycles)
            H_CYCLES="$2"
            shift 2
            ;;
        --l-cycles)
            L_CYCLES="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --epochs NUM        Number of training epochs (default: 1000)"
            echo "  --lr RATE           Learning rate (default: 0.01)"
            echo "  --layers NUM        Number of layers (default: 2)"
            echo "  --h-cycles NUM      Number of outer cycles (default: 3)"
            echo "  --l-cycles NUM      Number of inner cycles (default: 4)"
            echo "  -o, --output PATH   Output model path (default: model.trm)"
            echo "  -h, --help          Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "=== Training TRM Model ==="
echo "Configuration:"
echo "  Epochs: $EPOCHS"
echo "  Learning rate: $LEARNING_RATE"
echo "  Layers: $LAYERS"
echo "  H-cycles: $H_CYCLES"
echo "  L-cycles: $L_CYCLES"
echo "  Output: $OUTPUT"
echo ""

cargo run --release --bin train-trm -- train \
    --epochs "$EPOCHS" \
    --lr "$LEARNING_RATE" \
    --layers "$LAYERS" \
    --h-cycles "$H_CYCLES" \
    --l-cycles "$L_CYCLES" \
    -o "$OUTPUT"
