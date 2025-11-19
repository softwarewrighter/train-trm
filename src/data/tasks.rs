//! Task implementations for training and evaluation

use super::{Problem, TrainingExample};
use ndarray::Array2;
use rand::Rng;

/// Simple pattern matching task: predict next element in sequence
/// Input: sequence of n elements
/// Output: next element
pub struct SequenceTask {
    examples: Vec<TrainingExample>,
    input_dim: usize,
    output_dim: usize,
}

impl SequenceTask {
    /// Create a new sequence task with generated examples
    pub fn new(num_examples: usize, sequence_length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut examples = Vec::new();

        // Input: sequence_length numbers, Output: next number
        let input_dim = sequence_length;
        let output_dim = 1;

        for _ in 0..num_examples {
            // Generate simple arithmetic sequence: a, a+d, a+2d, ..., a+nd
            let start: f32 = rng.gen_range(-10.0..10.0);
            let step: f32 = rng.gen_range(-2.0..2.0);

            let mut sequence = Vec::new();
            for i in 0..=sequence_length {
                sequence.push(start + step * i as f32);
            }

            // Input is first sequence_length elements
            let input =
                Array2::from_shape_vec((1, sequence_length), sequence[..sequence_length].to_vec())
                    .unwrap();

            // Target is the next element
            let target =
                Array2::from_shape_vec((1, output_dim), vec![sequence[sequence_length]]).unwrap();

            examples.push(TrainingExample::new(input, target));
        }

        Self {
            examples,
            input_dim,
            output_dim,
        }
    }

    /// Get all training examples
    pub fn examples(&self) -> &[TrainingExample] {
        &self.examples
    }

    /// Split into training and validation sets
    pub fn split(&self, train_ratio: f32) -> (Vec<TrainingExample>, Vec<TrainingExample>) {
        let train_size = (self.examples.len() as f32 * train_ratio) as usize;
        let train = self.examples[..train_size].to_vec();
        let val = self.examples[train_size..].to_vec();
        (train, val)
    }
}

impl Problem for SequenceTask {
    fn input(&self) -> &Array2<f32> {
        &self.examples[0].input
    }

    fn target(&self) -> &Array2<f32> {
        &self.examples[0].target
    }

    fn validate_solution(&self, output: &Array2<f32>) -> bool {
        // Check if output is close to target (within 10% error)
        let target = self.target();
        let error = (output[[0, 0]] - target[[0, 0]]).abs();
        let relative_error = error / target[[0, 0]].abs().max(1.0);
        relative_error < 0.1
    }

    fn input_dim(&self) -> usize {
        self.input_dim
    }

    fn output_dim(&self) -> usize {
        self.output_dim
    }
}

/// Simple copy task: copy input to output
/// Useful for testing if model can learn basic input-output mapping
pub struct CopyTask {
    examples: Vec<TrainingExample>,
    dim: usize,
}

impl CopyTask {
    /// Create a new copy task
    pub fn new(num_examples: usize, dim: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut examples = Vec::new();

        for _ in 0..num_examples {
            let mut values = Vec::new();
            for _ in 0..dim {
                values.push(rng.gen_range(-1.0..1.0));
            }

            let input = Array2::from_shape_vec((1, dim), values.clone()).unwrap();
            let target = Array2::from_shape_vec((1, dim), values).unwrap();

            examples.push(TrainingExample::new(input, target));
        }

        Self { examples, dim }
    }

    /// Get all training examples
    pub fn examples(&self) -> &[TrainingExample] {
        &self.examples
    }

    /// Split into training and validation sets
    pub fn split(&self, train_ratio: f32) -> (Vec<TrainingExample>, Vec<TrainingExample>) {
        let train_size = (self.examples.len() as f32 * train_ratio) as usize;
        let train = self.examples[..train_size].to_vec();
        let val = self.examples[train_size..].to_vec();
        (train, val)
    }
}

impl Problem for CopyTask {
    fn input(&self) -> &Array2<f32> {
        &self.examples[0].input
    }

    fn target(&self) -> &Array2<f32> {
        &self.examples[0].target
    }

    fn validate_solution(&self, output: &Array2<f32>) -> bool {
        let target = self.target();
        let mse = output
            .iter()
            .zip(target.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            / output.len() as f32;
        mse < 0.01
    }

    fn input_dim(&self) -> usize {
        self.dim
    }

    fn output_dim(&self) -> usize {
        self.dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_task_creation() {
        let task = SequenceTask::new(10, 5);
        assert_eq!(task.examples().len(), 10);
        assert_eq!(task.input_dim(), 5);
        assert_eq!(task.output_dim(), 1);
    }

    #[test]
    fn test_sequence_task_shape() {
        let task = SequenceTask::new(5, 3);
        let examples = task.examples();
        for example in examples {
            assert_eq!(example.input.shape(), &[1, 3]);
            assert_eq!(example.target.shape(), &[1, 1]);
        }
    }

    #[test]
    fn test_sequence_task_split() {
        let task = SequenceTask::new(100, 5);
        let (train, val) = task.split(0.8);
        assert_eq!(train.len(), 80);
        assert_eq!(val.len(), 20);
    }

    #[test]
    fn test_copy_task_creation() {
        let task = CopyTask::new(10, 5);
        assert_eq!(task.examples().len(), 10);
        assert_eq!(task.input_dim(), 5);
        assert_eq!(task.output_dim(), 5);
    }

    #[test]
    fn test_copy_task_correctness() {
        let task = CopyTask::new(5, 3);
        let examples = task.examples();
        for example in examples {
            // Input and target should be identical
            assert_eq!(example.input, example.target);
        }
    }

    #[test]
    fn test_copy_task_split() {
        let task = CopyTask::new(100, 5);
        let (train, val) = task.split(0.7);
        assert_eq!(train.len(), 70);
        assert_eq!(val.len(), 30);
    }
}
