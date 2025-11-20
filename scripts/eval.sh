#!/bin/bash
# Evaluate a trained TRM model

set -e

# Default parameters
MODEL="model.trm"
INPUT=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--model)
            MODEL="$2"
            shift 2
            ;;
        -i|--input)
            INPUT="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -m, --model PATH    Path to trained model (default: model.trm)"
            echo "  -i, --input PATH    Path to input file (optional)"
            echo "  -h, --help          Show this help message"
            echo ""
            echo "If no input file is provided, the script will run a validation"
            echo "test with the copy task."
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

if [ ! -f "$MODEL" ]; then
    echo "Error: Model file '$MODEL' not found"
    echo "Please train a model first using ./scripts/train.sh"
    exit 1
fi

echo "=== Evaluating TRM Model ==="
echo "Model: $MODEL"
if [ -n "$INPUT" ]; then
    echo "Input: $INPUT"
fi
echo ""

if [ -n "$INPUT" ]; then
    cargo run --release --bin train-trm -- eval --model "$MODEL" --input "$INPUT"
else
    cargo run --release --bin train-trm -- eval --model "$MODEL"
fi
