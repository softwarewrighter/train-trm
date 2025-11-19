# Architecture

## Overview

The train-trm project implements a Tiny Recursive Model (TRM) for demonstrating recursive reasoning on complex tasks. The system is built using Rust for performance and WASM compatibility, with both CLI and web interfaces.

## System Components

### 1. Core TRM Engine (Rust)

The TRM engine consists of:

- **Network Module**: A simple 2-layer neural network that performs recursive reasoning
- **Latent State (`z`)**: Internal scratchpad for recursive thinking
- **Solution Embedding (`y`)**: Current answer representation
- **Recursive Loop**: Think/Act cycle that iterates to improve solutions

#### TRM Algorithm

```
For K improvement steps:
  For n inner steps:
    THINK: Update latent state z = f(x, y, z)
  ACT: Update answer y = g(y, z)
```

Where:
- `x` = input question/problem
- `y` = current answer
- `z` = latent scratchpad state
- `f` = think function (2-layer network)
- `g` = act function (2-layer network, may share weights with f)

### 2. CLI Interface (clap)

Command-line interface providing:
- Training commands
- Inference/evaluation commands
- Model configuration options
- Data loading and preprocessing

### 3. Web UI (Yew + WASM)

Browser-based interface featuring:
- Visual problem representation
- Real-time training visualization
- Interactive inference demonstrations
- Model parameter controls

## Architecture Diagram

```
┌─────────────────────────────────────────┐
│           User Interface                │
│  ┌──────────────┐    ┌──────────────┐  │
│  │ CLI (clap)   │    │ Web UI (Yew) │  │
│  └──────┬───────┘    └───────┬──────┘  │
│         │                    │          │
└─────────┼────────────────────┼──────────┘
          │                    │
          └────────┬───────────┘
                   │
         ┌─────────▼────────────┐
         │   TRM Engine (Rust)  │
         │                      │
         │  ┌────────────────┐  │
         │  │ Network        │  │
         │  │ (2 layers)     │  │
         │  └────────────────┘  │
         │                      │
         │  ┌────────────────┐  │
         │  │ Think Function │  │
         │  │ (update z)     │  │
         │  └────────────────┘  │
         │                      │
         │  ┌────────────────┐  │
         │  │ Act Function   │  │
         │  │ (update y)     │  │
         │  └────────────────┘  │
         │                      │
         │  ┌────────────────┐  │
         │  │ Trainer        │  │
         │  └────────────────┘  │
         └──────────────────────┘
```

## Data Flow

1. **Input**: Problem representation (e.g., grid, puzzle, question)
2. **Encoding**: Convert to tensor representation
3. **Recursive Reasoning**:
   - Initialize latent state `z` and answer `y`
   - For each outer cycle:
     - Think: Update `z` for n steps
     - Act: Update `y` based on `z`
4. **Output**: Final answer `y` decoded to problem space
5. **Loss**: Compare with ground truth, backpropagate

## Technology Stack

- **Core**: Rust (stable)
- **ML Framework**: burn (Rust ML framework) or candle (lightweight ML)
- **CLI**: clap v4
- **Web**: Yew framework
- **WASM**: wasm-bindgen, wasm-pack
- **Testing**: cargo test with wasm-bindgen-test for WASM
- **Build**: Cargo, trunk (for Yew)

## Module Organization

```
train-trm/
├── src/
│   ├── lib.rs           # Library root
│   ├── main.rs          # CLI entry point
│   ├── model/           # TRM model implementation
│   │   ├── mod.rs
│   │   ├── network.rs   # Neural network layers
│   │   ├── trm.rs       # TRM algorithm
│   │   └── trainer.rs   # Training logic
│   ├── cli/             # CLI commands
│   │   ├── mod.rs
│   │   ├── train.rs
│   │   └── eval.rs
│   └── web/             # Yew web UI
│       ├── mod.rs
│       ├── app.rs
│       └── components/
├── tests/               # Integration tests
└── docs/                # Documentation
```

## Key Design Decisions

1. **Single Network**: Unlike HRM, TRM uses one network for both think and act
2. **Minimal Parameters**: Target ~7M parameters for efficiency
3. **Recursive Design**: Leverage iteration over depth for reasoning
4. **WASM-First**: Design for both native and web deployment
5. **Test-Driven**: All features developed with tests first
