# Development Process

## Overview

This document defines the development process for train-trm, following Test-Driven Development (TDD) with strict quality gates before each commit.

## Test-Driven Development (TDD)

### Red-Green-Refactor Cycle

1. **Red**: Write a failing test first
   - Define expected behavior
   - Write minimal test code
   - Verify test fails for right reason

2. **Green**: Make the test pass
   - Write minimal implementation
   - Focus on making test pass
   - Don't optimize yet

3. **Refactor**: Improve the code
   - Clean up implementation
   - Remove duplication
   - Improve naming and structure
   - Ensure tests still pass

### Example Workflow

```rust
// RED: Write failing test
#[test]
fn test_layer_forward_pass() {
    let layer = Layer::new(3, 2, ActivationType::ReLU);
    let input = Array1::from_vec(vec![1.0, 2.0, 3.0]);
    let output = layer.forward(&input);
    assert_eq!(output.len(), 2);
}

// GREEN: Implement to pass
impl Layer {
    pub fn forward(&self, input: &Array1<f32>) -> Array1<f32> {
        self.weights.dot(input) + &self.bias
    }
}

// REFACTOR: Clean up, add activation
impl Layer {
    pub fn forward(&self, input: &Array1<f32>) -> Array1<f32> {
        let linear = self.weights.dot(input) + &self.bias;
        self.activation.apply(&linear)
    }
}
```

## Pre-Commit Checklist

Before **every** commit, complete all steps:

### 1. Format Code
```bash
cargo fmt --all
```
- Ensures consistent code style
- No manual formatting needed
- Must pass without changes

### 2. Run Tests
```bash
# All tests must pass
cargo test --all

# WASM tests (when applicable)
wasm-pack test --headless --firefox
```
- Unit tests
- Integration tests
- WASM tests (for web features)
- All tests must pass (no skipped tests)

### 3. Run Clippy
```bash
# Fix all clippy warnings
cargo clippy --all-targets --all-features -- -D warnings
```
- Must have zero warnings
- Do not bypass clippy checks
- Use allow attributes only when necessary

### 4. Handle Dead Code

For code that appears dead but is needed for WASM or future features:

```rust
// Good: Specific, documented
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn wasm_only_function() {
    // ...
}

// Good: Feature-gated
#[cfg(feature = "web")]
pub fn web_feature() {
    // ...
}

// Avoid: Blanket allows
#[allow(dead_code)]  // Don't do this without reason
```

**Best Practices**:
- Use `#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]` for WASM-specific code
- Use feature flags for optional functionality
- Document why code appears unused
- Remove truly unused code

### 5. Validate .gitignore

Check that build artifacts are ignored:
```bash
git status --ignored
```

Expected ignored files:
- `target/`
- `Cargo.lock` (for libraries)
- `*.wasm`
- `dist/`
- `pkg/`
- `.DS_Store`
- `*.swp`
- IDE config (`.idea/`, `.vscode/`)

### 6. Update Documentation

Before committing:
- Update `docs/status.md` with progress
- Add/update inline documentation for new public APIs
- Update README.md if user-facing changes
- Add examples for new features

### 7. Write Commit Message

Follow conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code change that neither fixes bug nor adds feature
- `perf`: Performance improvement
- `chore`: Maintenance tasks

**Example**:
```
feat(model): implement TRM forward pass

Add recursive think/act cycles to TRM model:
- Think step updates latent state z
- Act step updates solution y
- Configurable inner/outer cycle counts

Tests verify correct tensor shapes and gradient flow.

Closes #123
```

## Testing Guidelines

### Unit Tests

- Test each function/method independently
- Mock dependencies
- Use descriptive test names
- Test edge cases and error conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_forward_computes_correct_output_shape() {
        // Arrange
        let layer = Layer::new(10, 5, ActivationType::ReLU);
        let input = Array1::zeros(10);

        // Act
        let output = layer.forward(&input);

        // Assert
        assert_eq!(output.len(), 5);
    }

    #[test]
    fn layer_forward_applies_relu_activation() {
        let layer = Layer::new(2, 2, ActivationType::ReLU);
        let input = array![1.0, -1.0];

        let output = layer.forward(&input);

        // ReLU should zero negative values
        assert!(output.iter().all(|&x| x >= 0.0));
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn layer_forward_panics_on_wrong_input_size() {
        let layer = Layer::new(10, 5, ActivationType::ReLU);
        let input = Array1::zeros(5); // Wrong size

        layer.forward(&input); // Should panic
    }
}
```

### Integration Tests

- Test component interactions
- Use realistic data
- Place in `tests/` directory

```rust
// tests/training_integration.rs
use train_trm::*;

#[test]
fn train_model_on_pattern_task() {
    let config = TRMConfig::default();
    let mut model = TRMModel::new(config);
    let dataset = generate_pattern_dataset(100);

    let trainer = Trainer::new(model, OptimizerConfig::default());
    let metrics = trainer.train(&dataset, 10);

    // Model should learn (loss decreases)
    assert!(metrics.final_loss < metrics.initial_loss);
}
```

### WASM Tests

```rust
#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn model_runs_in_browser() {
        let model = TRMModel::new(TRMConfig::default());
        let input = vec![0.0; 10];

        let output = model.forward(&input);

        assert!(!output.is_empty());
    }
}
```

## Code Quality Standards

### Naming Conventions

- Types: `PascalCase`
- Functions/methods: `snake_case`
- Constants: `UPPER_SNAKE_CASE`
- Modules: `snake_case`

### Documentation

All public items must have documentation:

```rust
/// Performs forward pass through the layer.
///
/// # Arguments
///
/// * `input` - Input tensor with shape matching layer input size
///
/// # Returns
///
/// Output tensor after linear transformation and activation
///
/// # Examples
///
/// ```
/// use train_trm::network::Layer;
/// use ndarray::array;
///
/// let layer = Layer::new(3, 2, ActivationType::ReLU);
/// let input = array![1.0, 2.0, 3.0];
/// let output = layer.forward(&input);
/// ```
pub fn forward(&self, input: &Array1<f32>) -> Array1<f32> {
    // Implementation
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Define custom error types
- Provide context in errors

```rust
pub type Result<T> = std::result::Result<T, TRMError>;

#[derive(Debug, thiserror::Error)]
pub enum TRMError {
    #[error("Invalid input dimension: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Model not initialized")]
    NotInitialized,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Branching Strategy

- `main`: Stable code only
- Feature branches: `claude/feature-name-<session-id>`
- All development on feature branches
- Merge to main after review

## Continuous Integration (Future)

When CI is set up:
- Run tests on all PRs
- Check formatting
- Run clippy
- Build WASM
- Generate documentation

## Performance Guidelines

- Profile before optimizing
- Use `#[inline]` for hot paths
- Minimize allocations in tight loops
- Use `cargo bench` for benchmarks

## WASM-Specific Considerations

### Build for WASM
```bash
wasm-pack build --target web --out-dir pkg
```

### Test in Browser
```bash
wasm-pack test --headless --firefox
```

### Size Optimization
```toml
[profile.release]
opt-level = "z"  # Optimize for size
lto = true       # Link-time optimization
codegen-units = 1
```

### Memory Management
- Use `f32` instead of `f64`
- Clear large allocations promptly
- Avoid deep recursion (use iteration)

## Documentation Updates

Update these files regularly:

- `docs/status.md`: After each significant change
- `README.md`: When user-facing features change
- `CHANGELOG.md`: For release notes (future)
- Code comments: With every commit

## Review Checklist

Before marking feature complete:

- [ ] All tests pass
- [ ] Clippy clean
- [ ] Code formatted
- [ ] Documentation updated
- [ ] Examples added
- [ ] .gitignore validated
- [ ] Commit message written
- [ ] No TODO comments without issue references
- [ ] Performance acceptable
- [ ] Memory usage reasonable

## Example: Complete Feature Development

```bash
# 1. Write test (RED)
# Edit src/model/trm.rs - add test
cargo test  # Should fail

# 2. Implement feature (GREEN)
# Edit src/model/trm.rs - add implementation
cargo test  # Should pass

# 3. Refactor
# Clean up code
cargo test  # Still passes

# 4. Pre-commit checks
cargo fmt --all
cargo test --all
cargo clippy --all-targets --all-features -- -D warnings
git status --ignored  # Check .gitignore
# Update docs/status.md

# 5. Commit
git add .
git commit -m "feat(model): add think/act cycles to TRM

Implement recursive reasoning with configurable cycles:
- Think step: update latent state
- Act step: update solution
- Tests verify tensor shapes and gradient flow"

# 6. Push
git push -u origin claude/build-mvp-from-readme-<session-id>
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Yew Documentation](https://yew.rs/docs/getting-started/introduction)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
