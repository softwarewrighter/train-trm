# Design Document

## System Design

### Core Components

#### 1. Neural Network Layer

```rust
pub struct Layer {
    weights: Tensor,
    bias: Tensor,
    activation: ActivationType,
}

pub enum ActivationType {
    ReLU,
    Tanh,
    Identity,
}
```

**Responsibilities**:
- Forward propagation
- Backward propagation (gradient computation)
- Parameter updates

#### 2. TRM Model

```rust
pub struct TRMModel {
    network: Network,
    config: TRMConfig,
}

pub struct TRMConfig {
    l_layers: usize,      // Number of layers (2)
    h_cycles: usize,      // Outer cycles
    l_cycles: usize,      // Inner cycles (think steps)
    hidden_dim: usize,    // Hidden layer size
    latent_dim: usize,    // Latent state dimension
}
```

**Responsibilities**:
- Manage recursive think/act cycles
- Maintain latent state
- Coordinate forward/backward passes

#### 3. Trainer

```rust
pub struct Trainer {
    model: TRMModel,
    optimizer: Optimizer,
    config: TrainingConfig,
}

pub struct TrainingConfig {
    learning_rate: f32,
    batch_size: usize,
    epochs: usize,
    validation_split: f32,
}
```

**Responsibilities**:
- Training loop execution
- Loss computation
- Gradient descent
- Checkpoint management

### Data Structures

#### Problem Representation

```rust
pub trait Problem {
    fn input(&self) -> Tensor;
    fn target(&self) -> Tensor;
    fn validate_solution(&self, output: &Tensor) -> bool;
}
```

#### Training Example

```rust
pub struct TrainingExample {
    input: Tensor,
    target: Tensor,
    metadata: HashMap<String, String>,
}
```

### Algorithms

#### Forward Pass

```
function forward(x: input, model: TRMModel) -> y:
    # Initialize
    y = zeros(output_dim)
    z = zeros(latent_dim)

    # Recursive improvement
    for h in 0..H_cycles:
        # Think phase (inner cycles)
        for l in 0..L_cycles:
            z = network.forward(concat(x, y, z))

        # Act phase
        y = network.forward(concat(y, z))

    return y
```

#### Backward Pass

```
function backward(loss: scalar, model: TRMModel):
    # Backpropagate through unrolled computation graph
    # Gradient flows through all think/act cycles
    # Update network parameters
```

#### Training Loop

```
function train(model: TRMModel, dataset: Dataset, config: TrainingConfig):
    for epoch in 0..config.epochs:
        for batch in dataset.batches(config.batch_size):
            # Forward
            outputs = []
            for example in batch:
                output = model.forward(example.input)
                outputs.push(output)

            # Loss
            loss = compute_loss(outputs, batch.targets)

            # Backward
            model.backward(loss)

            # Update
            optimizer.step()

        # Validation
        val_loss = evaluate(model, validation_set)
        log_metrics(epoch, loss, val_loss)
```

### Module Design

#### lib.rs
```rust
pub mod model;
pub mod training;
pub mod data;
pub mod utils;

#[cfg(target_arch = "wasm32")]
pub mod web;
```

#### model/mod.rs
```rust
pub mod network;
pub mod trm;
pub mod tensor;

pub use network::Network;
pub use trm::{TRMModel, TRMConfig};
```

#### CLI Design

```
train-trm
├── train
│   ├── --task <TASK>
│   ├── --layers <N>
│   ├── --h-cycles <N>
│   ├── --l-cycles <N>
│   ├── --lr <FLOAT>
│   ├── --epochs <N>
│   └── --output <PATH>
├── eval
│   ├── --model <PATH>
│   ├── --input <PATH>
│   └── --visualize
└── serve
    ├── --port <PORT>
    └── --model <PATH>
```

### Web UI Design

#### Component Hierarchy

```
App
├── Header
├── ProblemViewer
│   └── GridDisplay / InputDisplay
├── ModelControls
│   ├── ConfigPanel
│   └── RunButton
├── ReasoningVisualizer
│   ├── LatentStateView
│   ├── ThinkActTimeline
│   └── SolutionEvolution
└── Footer
```

#### State Management

```rust
pub struct AppState {
    model: Option<TRMModel>,
    current_problem: Option<Box<dyn Problem>>,
    inference_running: bool,
    reasoning_history: Vec<ReasoningStep>,
}

pub struct ReasoningStep {
    cycle: usize,
    latent_state: Vec<f32>,
    current_solution: Vec<f32>,
    timestamp: f64,
}
```

### Error Handling

```rust
pub enum TRMError {
    InvalidInput(String),
    ModelNotTrained,
    SerializationError(String),
    ComputationError(String),
}

pub type Result<T> = std::result::Result<T, TRMError>;
```

### Testing Strategy

#### Unit Tests
- Layer forward/backward correctness
- Tensor operations
- Loss functions
- Optimizer updates

#### Integration Tests
- Full forward pass
- Training convergence on toy problem
- Model save/load
- CLI command execution

#### WASM Tests
- Web UI initialization
- Browser inference
- State management

### Configuration

```toml
# trm.toml
[model]
l_layers = 2
h_cycles = 3
l_cycles = 4
hidden_dim = 256
latent_dim = 128

[training]
learning_rate = 0.001
batch_size = 32
epochs = 100
optimizer = "adam"

[data]
task = "pattern_match"
num_examples = 1000
augmentation = true
```

### Performance Considerations

1. **Memory**: Reuse tensors where possible
2. **Computation**: Batch operations
3. **WASM**: Minimize allocations, use f32 not f64
4. **Serialization**: Use efficient binary format (bincode)

### Security Considerations

1. Input validation for all user-provided data
2. Bounds checking on tensor operations
3. Safe deserialization of models
4. No unsafe code in core logic

## Design Decisions

### Decision 1: Use Simple Matrix Library
- **Options**: ndarray, burn, candle, custom
- **Choice**: ndarray for simplicity, with eye on burn migration
- **Rationale**: MVP needs basic operations, can upgrade later

### Decision 2: Shared Think/Act Network
- **Options**: Separate networks, shared weights, shared architecture
- **Choice**: Shared network with different input concatenations
- **Rationale**: Simpler implementation, fewer parameters

### Decision 3: No GPU Support in MVP
- **Rationale**: Focus on correctness first, CPU sufficient for demos

### Decision 4: Text-Based Initial Task
- **Options**: Sudoku, Maze, Pattern matching, ARC-AGI
- **Choice**: Pattern matching (e.g., sequence prediction)
- **Rationale**: Easier to implement and visualize

## Future Enhancements

1. GPU acceleration via burn or candle
2. Advanced tasks (Sudoku, Maze, ARC-AGI)
3. Attention mechanisms
4. Model pruning/quantization
5. Distributed training
