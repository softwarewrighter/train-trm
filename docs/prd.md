# Product Requirements Document (PRD)

## Project: train-trm

### Vision
Create an accessible implementation of the Tiny Recursive Model (TRM) that demonstrates how small neural networks can solve complex reasoning tasks through recursive iteration.

## Goals

### Primary Goals
1. Implement a working TRM model in Rust
2. Provide both CLI and web interfaces for accessibility
3. Demonstrate training on at least one reasoning task
4. Achieve educational clarity while maintaining technical accuracy

### Secondary Goals
1. Optimize for WASM deployment
2. Support multiple task types (extensible architecture)
3. Provide visualization of the recursive reasoning process
4. Document the development process thoroughly

## Target Users

1. **ML Researchers**: Exploring alternative reasoning architectures
2. **Rust Developers**: Learning ML implementation in Rust
3. **Students**: Understanding recursive reasoning models
4. **Practitioners**: Experimenting with efficient reasoning models

## Features

### Core Features (MVP)

#### F1: TRM Model Implementation
- **Description**: Core recursive reasoning engine
- **Requirements**:
  - 2-layer neural network
  - Think/Act recursive loop
  - Latent state management
  - Forward pass implementation
  - Backward pass for training
- **Success Criteria**: Model can perform recursive updates

#### F2: CLI Interface
- **Description**: Command-line tool for training and inference
- **Requirements**:
  - `train` command with configurable parameters
  - `eval` command for inference
  - Progress reporting
  - Model save/load
- **Success Criteria**: Users can train and evaluate models via CLI

#### F3: Training Demonstration
- **Description**: Working example on a reasoning task
- **Requirements**:
  - At least one task implementation (e.g., simple pattern matching)
  - Training loop with loss tracking
  - Convergence on training data
  - Basic generalization test
- **Success Criteria**: Model learns to solve example tasks

#### F4: Web UI (Basic)
- **Description**: Browser-based interface for visualization
- **Requirements**:
  - WASM compilation support
  - Basic problem visualization
  - Inference demo
  - Parameter display
- **Success Criteria**: Users can run inference in browser

### Future Features (Post-MVP)

- F5: Advanced Tasks (Sudoku, Maze, ARC-AGI puzzles)
- F6: Training Visualization (live loss plots, attention maps)
- F7: Model Comparison Tools
- F8: Pre-trained Model Zoo
- F9: Interactive Training in Browser
- F10: Performance Benchmarks

## Technical Requirements

### Performance
- Training time: Reasonable on CPU for small demos (< 10 min)
- Inference: < 1s per problem on CPU
- WASM bundle: < 5MB compressed

### Compatibility
- Rust: stable channel
- Browsers: Chrome, Firefox, Safari (latest 2 versions)
- OS: Linux, macOS, Windows

### Quality
- Test coverage: > 70%
- All clippy warnings addressed
- Documented public APIs
- No unsafe code without justification

## Non-Requirements (Out of Scope)

1. GPU acceleration (future enhancement)
2. Distributed training
3. Production deployment infrastructure
4. Real-time collaborative features
5. Mobile app versions

## Success Metrics

### MVP Success Criteria
1. Model trains and converges on example task
2. CLI commands work as documented
3. Web UI loads and runs inference
4. All tests pass
5. Code passes clippy checks
6. Documentation complete

### Quality Metrics
- Unit test coverage > 70%
- Integration tests for all commands
- Documentation for all public APIs
- Zero critical clippy warnings
- Clean git history with descriptive commits

## Timeline

### Phase 1: Foundation (Current)
- [ ] Documentation setup
- [ ] Project structure
- [ ] Basic model implementation

### Phase 2: Core Implementation
- [ ] TRM algorithm
- [ ] Training loop
- [ ] CLI commands

### Phase 3: Demonstration
- [ ] Example task
- [ ] Training demo
- [ ] Testing

### Phase 4: Web Interface
- [ ] Yew setup
- [ ] WASM build
- [ ] Basic UI

## Risks and Mitigations

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| ML framework limitations in Rust | High | Medium | Use burn or candle, fallback to manual implementation |
| WASM performance issues | Medium | Low | Profile early, optimize hot paths |
| Complexity of TRM algorithm | High | Medium | Start simple, iterate based on tests |
| Browser compatibility | Low | Low | Use standard WASM features only |

## Open Questions

1. Which Rust ML framework: burn vs candle vs custom?
2. Example task complexity: pattern matching vs Sudoku?
3. Training data generation strategy?
4. Visualization approach for recursive steps?

## Stakeholder Sign-off

- [x] Project requirements defined
- [ ] Technical approach validated
- [ ] MVP scope agreed
