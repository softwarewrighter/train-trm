//! Training demonstration for TRM model
//!
//! This example shows how to train a TRM model on a simple copy task.
//!
//! Run with: cargo run --example train_demo

use train_trm::data::tasks::CopyTask;
use train_trm::model::{TRMConfig, TRMModel};
use train_trm::training::{Trainer, TrainingConfig};

fn main() {
    println!("=== TRM Training Demonstration ===\n");

    // Create a simple copy task
    println!("Creating copy task with 100 examples (dim=5)...");
    let task = CopyTask::new(100, 5);
    let (train_examples, val_examples) = task.split(0.8);
    println!("Training examples: {}", train_examples.len());
    println!("Validation examples: {}\n", val_examples.len());

    // Configure TRM model
    let model_config = TRMConfig {
        input_dim: 5,
        output_dim: 5,
        hidden_dim: 16,
        latent_dim: 16,
        l_layers: 2,
        h_cycles: 2,
        l_cycles: 2,
    };

    println!("Model configuration:");
    println!("  Input dim: {}", model_config.input_dim);
    println!("  Output dim: {}", model_config.output_dim);
    println!("  Hidden dim: {}", model_config.hidden_dim);
    println!("  Latent dim: {}", model_config.latent_dim);
    println!("  Layers: {}", model_config.l_layers);
    println!("  H-cycles (outer): {}", model_config.h_cycles);
    println!("  L-cycles (inner): {}\n", model_config.l_cycles);

    // Create model
    let model = TRMModel::new(model_config);
    println!("Model created with {} parameters\n", model.num_parameters());

    // Configure training
    let train_config = TrainingConfig {
        learning_rate: 0.01,
        epochs: 50,
        batch_size: 16,
        ..Default::default()
    };

    println!("Training configuration:");
    println!("  Learning rate: {}", train_config.learning_rate);
    println!("  Epochs: {}", train_config.epochs);
    println!("  Batch size: {}\n", train_config.batch_size);

    // Create trainer
    let mut trainer = Trainer::new(model, train_config);

    // Evaluate before training
    let initial_train_loss = trainer.evaluate(&train_examples);
    let initial_val_loss = trainer.evaluate(&val_examples);
    println!("Initial train loss: {:.6}", initial_train_loss);
    println!("Initial validation loss: {:.6}\n", initial_val_loss);

    // Train
    println!("Starting training...\n");
    let metrics = trainer.train(&train_examples);

    // Evaluate after training
    let final_val_loss = trainer.evaluate(&val_examples);
    println!("\n=== Training Complete ===");
    println!("Initial loss: {:.6}", metrics.initial_loss);
    println!("Final train loss: {:.6}", metrics.final_loss);
    println!("Final validation loss: {:.6}", final_val_loss);

    // Note about limitations
    println!("\nNote: This is a demonstration of the training infrastructure.");
    println!("Full gradient-based learning requires implementing backpropagation");
    println!("through the recursive TRM architecture, which is beyond the MVP scope.");
    println!("The model structure and forward pass are fully functional.");
}
