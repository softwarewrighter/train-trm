//! Data structures and task definitions

pub mod tasks;

use ndarray::Array2;

pub use tasks::{CopyTask, SequenceTask};

/// Trait for problem definitions
pub trait Problem {
    /// Get the input tensor for this problem
    fn input(&self) -> &Array2<f32>;

    /// Get the target output tensor
    fn target(&self) -> &Array2<f32>;

    /// Validate if a solution is correct
    fn validate_solution(&self, output: &Array2<f32>) -> bool;

    /// Get input dimension
    fn input_dim(&self) -> usize;

    /// Get output dimension
    fn output_dim(&self) -> usize;
}

/// A training example with input and target
#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub input: Array2<f32>,
    pub target: Array2<f32>,
}

impl TrainingExample {
    /// Create a new training example
    pub fn new(input: Array2<f32>, target: Array2<f32>) -> Self {
        Self { input, target }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_training_example_creation() {
        let input = array![[1.0, 2.0], [3.0, 4.0]];
        let target = array![[5.0], [6.0]];

        let example = TrainingExample::new(input.clone(), target.clone());

        assert_eq!(example.input, input);
        assert_eq!(example.target, target);
    }
}
