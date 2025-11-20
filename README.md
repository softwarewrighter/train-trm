# train-trm

A Rust implementation of the Tiny Recursive Model (TRM), a refinement of the Hierarchical Reasoning Model (HRM), with CLI tools for training and evaluation.

## Overview

TRM is a neural architecture that performs recursive reasoning through alternating "think" and "act" phases:
- **Think phase**: Updates internal latent state by processing input, current answer, and previous state
- **Act phase**: Updates the answer based on the refined latent state
- This process repeats for multiple cycles to progressively improve the output

## Features

- ✅ Fully functional TRM implementation with backpropagation
- ✅ CLI for training and evaluation
- ✅ Web UI with maze visualization, training, and evaluation
- ✅ Model serialization/deserialization
- ✅ Copy task for basic testing
- ✅ Configurable architecture (layers, cycles, dimensions)
- ✅ Shell scripts for common operations
- ✅ 100% Rust (including web UI via WASM)

## Quick Start

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd train-trm

# Build the project
./scripts/build.sh --release
```

### Training a Model

```bash
# Train with default parameters (saves to model.trm)
./scripts/train.sh

# Train with custom parameters
./scripts/train.sh --epochs 500 --lr 0.01 --layers 2 -o my_model.trm
```

### Evaluating a Model

```bash
# Evaluate the default trained model (model.trm)
./scripts/eval.sh

# Evaluate a specific model
./scripts/eval.sh --model my_model.trm
```

### Training and Evaluation Pipeline

Both scripts use `model.trm` as the default path, so you can chain them:

```bash
# Train and immediately evaluate
./scripts/train.sh && ./scripts/eval.sh

# Or use the demo script
./scripts/demo.sh
```

### Web UI

Launch the interactive web interface:

```bash
# Install trunk (first time only)
cargo install --locked trunk
rustup target add wasm32-unknown-unknown

# Start development server
./scripts/web-serve.sh
```

Then open http://127.0.0.1:8080 in your browser.

**Features**:
- 🎯 Interactive maze visualization with solution paths
- 📊 Real-time training with live loss charts
- 📈 Model evaluation with detailed results

See [WEB_UI.md](WEB_UI.md) for complete web UI documentation.

## Usage

### Command Line Interface

#### Training

```bash
cargo run --release -- train [OPTIONS]

Options:
  --layers <NUM>      Number of layers (default: 2)
  --h-cycles <NUM>    Number of outer cycles (default: 3)
  --l-cycles <NUM>    Number of inner cycles (default: 4)
  --lr <RATE>         Learning rate (default: 0.01)
  --epochs <NUM>      Number of epochs (default: 1000)
  -o, --output <PATH> Output model path (default: model.trm)
```

#### Evaluation

```bash
cargo run --release -- eval [OPTIONS]

Options:
  -m, --model <PATH>  Path to trained model
  -i, --input <PATH>  Optional input file
```

### Shell Scripts

All scripts are located in the `./scripts` directory:

- **`train.sh`** - Train a model with configurable parameters
- **`eval.sh`** - Evaluate a trained model
- **`demo.sh`** - Run complete training and evaluation demo
- **`build.sh`** - Build the project (debug or release)
- **`test.sh`** - Run all tests (unit tests, clippy, formatting)
- **`format.sh`** - Format Rust code
- **`clean.sh`** - Clean build artifacts

See [scripts/README.md](scripts/README.md) for detailed documentation.

## Architecture

### Model Configuration

```rust
TRMConfig {
    input_dim: 5,       // Input dimension
    output_dim: 5,      // Output dimension
    hidden_dim: 16,     // Hidden layer dimension
    latent_dim: 16,     // Latent state dimension
    l_layers: 2,        // Number of network layers
    h_cycles: 3,        // Number of outer (think-act) cycles
    l_cycles: 4,        // Number of inner (think) cycles
}
```

### Training Loop

1. **Forward pass**: Input → Think cycles → Act → Output
2. **Loss computation**: MSE between prediction and target
3. **Backward pass**: Compute gradients via backpropagation
4. **Weight update**: Gradient descent with configured learning rate

### Example Training Results

**Recommended settings for high accuracy:**
- Learning rate: `0.01` (lower is more stable)
- Epochs: `1000` (patience is key)

With `lr=0.01` and 1000 epochs:
- Initial loss: **0.327** → Final loss: **0.022** (93% reduction)
- Validation loss: **0.029**
- **Accuracy: 95%** on copy task evaluation

**Key insights:**
- Lower learning rates (0.01) provide more stable training than higher rates (0.05-0.1)
- Training improves significantly after epoch 500-600
- The model learns to accurately copy 5-dimensional vectors

## Zero-Shot and Generalization

### Current Implementation

This TRM implementation is **task-specific** and requires supervised training:
- Models are trained on specific tasks (e.g., copy task)
- Training requires labeled input-output pairs
- The model learns through backpropagation on the training data

### Zero-Shot Capabilities

**Current status**: This implementation does **not** have zero-shot capabilities out of the box.

The model must be trained on a task before it can perform that task. For example:
- A model trained on the copy task can copy vectors
- The same model cannot solve arithmetic or other tasks without retraining
- No pre-training or transfer learning is currently implemented

### Future Enhancements for Zero-Shot

To enable zero-shot or few-shot capabilities, you would need:

1. **Pre-training on diverse tasks**: Train on a wide variety of tasks to learn general reasoning patterns
2. **Task conditioning**: Modify the architecture to accept task descriptions as input
3. **Meta-learning**: Implement techniques like MAML or Reptile for fast adaptation
4. **Prompt-based learning**: Add a mechanism to condition behavior on natural language prompts
5. **Larger scale**: Increase model capacity and training data diversity

### Generalization Within Task

The model does generalize **within** its training task:
- Trained on 80 random copy examples
- Evaluates on 20 new random examples
- Achieves 95% accuracy on unseen examples from the same distribution

This shows the model learns the underlying pattern, not just memorization.

## Development

### Running Tests

```bash
# Run all tests
./scripts/test.sh

# Or manually:
cargo test
cargo clippy
cargo fmt -- --check
```

### Code Structure

```
src/
├── data/           # Training tasks and datasets
│   ├── maze.rs     # Maze navigation task
│   └── tasks.rs    # Copy task and sequence prediction
├── model/          # TRM model implementation
│   ├── network.rs  # Neural network layers with backprop
│   ├── trm.rs      # TRM architecture
│   └── mod.rs
├── training/       # Training infrastructure
│   ├── loss.rs     # Loss functions and gradients
│   └── mod.rs      # Trainer implementation
├── utils/          # Utility functions
├── main.rs         # CLI entry point
└── lib.rs          # Library root
```

## Documentation

- [Architecture](docs/architecture.md) - System architecture and design
- [Design](docs/design.md) - Detailed design decisions
- [PRD](docs/prd.md) - Product requirements
- [Plan](docs/plan.md) - Implementation plan
- [Process](docs/process.md) - Development process and guidelines
- [Status](docs/status.md) - Project status and progress
- [Scripts](scripts/README.md) - Shell scripts documentation

## Requirements

- Rust 1.70+ (2021 edition)
- Cargo

## Dependencies

- `ndarray` - N-dimensional arrays
- `ndarray-rand` - Random array generation
- `serde` / `serde_json` - Serialization
- `clap` - CLI argument parsing
- `rand` - Random number generation

## License

See [LICENSE](LICENSE) file.

## Contributing

Before committing:

1. Format code: `./scripts/format.sh`
2. Run tests: `./scripts/test.sh`
3. Ensure clippy passes without warnings
4. Update documentation as needed

Follow the TDD process documented in [docs/process.md](docs/process.md).
