#!/bin/bash
# Run a complete demo: train and evaluate a model

set -e

echo "=== TRM Training Demo ==="
echo ""
echo "This script will:"
echo "  1. Train a model with reasonable parameters"
echo "  2. Evaluate the trained model"
echo ""

MODEL="demo_model.trm"

echo "Step 1: Training model..."
echo ""
./scripts/train.sh --epochs 500 --lr 0.01 -o "$MODEL"

echo ""
echo "Step 2: Evaluating model..."
echo ""
./scripts/eval.sh --model "$MODEL"

echo ""
echo "=== Demo complete ==="
echo "Trained model saved to: $MODEL"
