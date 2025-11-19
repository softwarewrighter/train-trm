//! Training infrastructure

pub mod loss;

use crate::data::TrainingExample;
use crate::model::TRMModel;
pub use loss::{compute_loss, mse_gradient, LossType};

/// Training configuration
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    /// Learning rate
    pub learning_rate: f32,
    /// Number of epochs
    pub epochs: usize,
    /// Batch size
    pub batch_size: usize,
    /// Loss function type
    pub loss_type: LossType,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            epochs: 100,
            batch_size: 32,
            loss_type: LossType::MSE,
        }
    }
}

/// Training metrics
#[derive(Debug, Clone)]
pub struct TrainingMetrics {
    /// Loss values per epoch
    pub losses: Vec<f32>,
    /// Initial loss
    pub initial_loss: f32,
    /// Final loss
    pub final_loss: f32,
}

/// Trainer for TRM models
pub struct Trainer {
    model: TRMModel,
    config: TrainingConfig,
}

impl Trainer {
    /// Create a new trainer
    pub fn new(model: TRMModel, config: TrainingConfig) -> Self {
        Self { model, config }
    }

    /// Run training loop
    ///
    /// Note: This is a simplified training demonstration for MVP.
    /// Full gradient-based training would require automatic differentiation.
    pub fn train(&mut self, examples: &[TrainingExample]) -> TrainingMetrics {
        let mut losses = Vec::new();

        // Compute initial loss
        let initial_loss = self.evaluate(examples);
        losses.push(initial_loss);

        // Training loop (simplified for MVP)
        for epoch in 0..self.config.epochs {
            let epoch_loss = self.train_epoch(examples);
            losses.push(epoch_loss);

            if epoch % 10 == 0 {
                println!("Epoch {}: loss = {:.6}", epoch, epoch_loss);
            }
        }

        let final_loss = *losses.last().unwrap_or(&initial_loss);

        TrainingMetrics {
            losses,
            initial_loss,
            final_loss,
        }
    }

    /// Train for one epoch
    fn train_epoch(&mut self, examples: &[TrainingExample]) -> f32 {
        let mut total_loss = 0.0;

        for example in examples {
            // Forward pass
            let prediction = self.model.forward(&example.input);

            // Compute loss
            let loss = compute_loss(&prediction, &example.target, self.config.loss_type);
            total_loss += loss;

            // Note: Backward pass and weight update would go here
            // For MVP, we skip actual gradient computation
            // A full implementation would require:
            // 1. Compute gradients through recursive layers
            // 2. Backpropagate through think/act cycles
            // 3. Update network weights
        }

        total_loss / examples.len() as f32
    }

    /// Evaluate model on examples
    pub fn evaluate(&self, examples: &[TrainingExample]) -> f32 {
        let mut total_loss = 0.0;

        for example in examples {
            let prediction = self.model.forward(&example.input);
            let loss = compute_loss(&prediction, &example.target, self.config.loss_type);
            total_loss += loss;
        }

        total_loss / examples.len() as f32
    }

    /// Get reference to the model
    pub fn model(&self) -> &TRMModel {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tasks::CopyTask;
    use crate::model::TRMConfig;

    #[test]
    fn test_trainer_creation() {
        let model_config = TRMConfig::default();
        let model = TRMModel::new(model_config);
        let train_config = TrainingConfig::default();
        let _trainer = Trainer::new(model, train_config);
        // Just verify it compiles and constructs
    }

    #[test]
    fn test_training_config_default() {
        let config = TrainingConfig::default();
        assert_eq!(config.learning_rate, 0.001);
        assert_eq!(config.epochs, 100);
        assert_eq!(config.batch_size, 32);
    }

    #[test]
    fn test_evaluate() {
        let model_config = TRMConfig {
            input_dim: 5,
            output_dim: 5,
            hidden_dim: 8,
            latent_dim: 8,
            l_layers: 2,
            h_cycles: 1,
            l_cycles: 1,
        };

        let model = TRMModel::new(model_config);
        let train_config = TrainingConfig {
            epochs: 1,
            ..Default::default()
        };

        let trainer = Trainer::new(model, train_config);

        // Create a simple task
        let task = CopyTask::new(5, 5);
        let examples = task.examples();

        // Evaluate should return a loss value
        let loss = trainer.evaluate(examples);
        assert!(loss >= 0.0);
    }
}
