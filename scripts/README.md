# TRM Scripts

This directory contains shell scripts for common development and operational tasks.

## Available Scripts

### Training and Evaluation

- **`train.sh`** - Train a TRM model
  ```bash
  ./scripts/train.sh [OPTIONS]

  Options:
    --epochs NUM        Number of training epochs (default: 1000)
    --lr RATE           Learning rate (default: 0.01)
    --layers NUM        Number of layers (default: 2)
    --h-cycles NUM      Number of outer cycles (default: 3)
    --l-cycles NUM      Number of inner cycles (default: 4)
    -o, --output PATH   Output model path (default: model.trm)
    -h, --help          Show help message

  Examples:
    ./scripts/train.sh
    ./scripts/train.sh --epochs 200 --lr 0.1 -o my_model.trm
  ```

- **`eval.sh`** - Evaluate a trained model
  ```bash
  ./scripts/eval.sh [OPTIONS]

  Options:
    -m, --model PATH    Path to trained model (default: model.trm)
    -i, --input PATH    Path to input file (optional)
    -h, --help          Show help message

  Examples:
    ./scripts/eval.sh
    ./scripts/eval.sh --model my_model.trm
  ```

- **`demo.sh`** - Run a complete training and evaluation demo
  ```bash
  ./scripts/demo.sh
  ```

### Development

- **`build.sh`** - Build the project
  ```bash
  ./scripts/build.sh [OPTIONS]

  Options:
    --release    Build in release mode (optimized)
    -h, --help   Show help message

  Examples:
    ./scripts/build.sh
    ./scripts/build.sh --release
  ```

- **`test.sh`** - Run all tests (unit tests, clippy, formatting checks)
  ```bash
  ./scripts/test.sh
  ```

- **`format.sh`** - Format all Rust code
  ```bash
  ./scripts/format.sh
  ```

- **`clean.sh`** - Clean build artifacts and temporary files
  ```bash
  ./scripts/clean.sh
  ```

### Web UI

- **`web-serve.sh`** - Start development server for web UI
  ```bash
  ./scripts/web-serve.sh

  # Opens http://127.0.0.1:8080 automatically
  # Hot-reload enabled for development
  ```

- **`web-build.sh`** - Build web UI for production
  ```bash
  ./scripts/web-build.sh

  # Output in dist/ directory
  # Optimized WASM build
  ```

## Quick Start

1. **Train a model:**
   ```bash
   ./scripts/train.sh --epochs 100 --lr 0.05 -o my_model.trm
   ```

2. **Evaluate the model:**
   ```bash
   ./scripts/eval.sh --model my_model.trm
   ```

3. **Run the complete demo:**
   ```bash
   ./scripts/demo.sh
   ```

## Development Workflow

Before committing changes:

```bash
# Format code
./scripts/format.sh

# Run tests
./scripts/test.sh

# Build in release mode
./scripts/build.sh --release
```

## Notes

- All scripts use `set -e` to exit on error
- Training and evaluation scripts use `--release` mode for better performance
- The `test.sh` script runs clippy with `-D warnings` to treat warnings as errors
- Temporary model files (*.trm) are cleaned by `clean.sh`
