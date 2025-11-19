//! CLI entry point for train-trm

use clap::{Parser, Subcommand};
use train_trm::data::tasks::CopyTask;
use train_trm::model::{TRMConfig, TRMModel};
use train_trm::training::{Trainer, TrainingConfig};

#[derive(Parser)]
#[command(name = "train-trm")]
#[command(about = "Tiny Recursive Model training and inference", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Train a TRM model
    Train {
        /// Number of layers
        #[arg(short, long, default_value_t = 2)]
        layers: usize,

        /// Number of outer cycles (H)
        #[arg(long, default_value_t = 3)]
        h_cycles: usize,

        /// Number of inner cycles (L)
        #[arg(long, default_value_t = 4)]
        l_cycles: usize,

        /// Learning rate
        #[arg(long, default_value_t = 0.001)]
        lr: f32,

        /// Number of epochs
        #[arg(short, long, default_value_t = 100)]
        epochs: usize,

        /// Output model path
        #[arg(short, long, default_value = "model.trm")]
        output: String,
    },

    /// Evaluate a trained model
    Eval {
        /// Model path
        #[arg(short, long)]
        model: String,

        /// Input file path
        #[arg(short, long)]
        input: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Train {
            layers,
            h_cycles,
            l_cycles,
            lr,
            epochs,
            output,
        } => {
            println!("=== Training TRM Model ===\n");

            // Create task
            println!("Creating copy task with 100 examples (dim=5)...");
            let task = CopyTask::new(100, 5);
            let (train_examples, val_examples) = task.split(0.8);
            println!("Training examples: {}", train_examples.len());
            println!("Validation examples: {}\n", val_examples.len());

            // Configure model
            let model_config = TRMConfig {
                input_dim: 5,
                output_dim: 5,
                hidden_dim: 16,
                latent_dim: 16,
                l_layers: layers,
                h_cycles,
                l_cycles,
            };

            println!("Model configuration:");
            println!("  Input/Output dim: 5");
            println!("  Hidden dim: 16");
            println!("  Latent dim: 16");
            println!("  Layers: {}", layers);
            println!("  H-cycles: {}", h_cycles);
            println!("  L-cycles: {}\n", l_cycles);

            let model = TRMModel::new(model_config);
            println!("Model created with {} parameters\n", model.num_parameters());

            // Configure training
            let train_config = TrainingConfig {
                learning_rate: lr,
                epochs,
                batch_size: 16,
                ..Default::default()
            };

            println!("Training configuration:");
            println!("  Learning rate: {}", lr);
            println!("  Epochs: {}", epochs);
            println!("  Batch size: 16\n");

            // Create trainer and train
            let mut trainer = Trainer::new(model, train_config);

            let initial_train_loss = trainer.evaluate(&train_examples);
            let initial_val_loss = trainer.evaluate(&val_examples);
            println!("Initial train loss: {:.6}", initial_train_loss);
            println!("Initial validation loss: {:.6}\n", initial_val_loss);

            println!("Training...\n");
            let metrics = trainer.train(&train_examples);

            let final_val_loss = trainer.evaluate(&val_examples);
            println!("\n=== Training Complete ===");
            println!("Initial loss: {:.6}", metrics.initial_loss);
            println!("Final train loss: {:.6}", metrics.final_loss);
            println!("Final validation loss: {:.6}\n", final_val_loss);

            // Save the trained model
            println!("Saving model to: {}", output);
            match trainer.model().save(&output) {
                Ok(_) => println!("Model saved successfully!"),
                Err(e) => {
                    eprintln!("Error saving model: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Eval { model, input } => {
            println!("=== Evaluating Model ===\n");

            // Load the model
            println!("Loading model from: {}", model);
            let loaded_model = match TRMModel::load(&model) {
                Ok(m) => {
                    println!("Model loaded successfully!\n");
                    m
                }
                Err(e) => {
                    eprintln!("Error loading model: {}", e);
                    std::process::exit(1);
                }
            };

            // Display model information
            println!("Model configuration:");
            println!("  Input dim: {}", loaded_model.config.input_dim);
            println!("  Output dim: {}", loaded_model.config.output_dim);
            println!("  Hidden dim: {}", loaded_model.config.hidden_dim);
            println!("  Latent dim: {}", loaded_model.config.latent_dim);
            println!("  Layers: {}", loaded_model.config.l_layers);
            println!("  H-cycles: {}", loaded_model.config.h_cycles);
            println!("  L-cycles: {}", loaded_model.config.l_cycles);
            println!("  Parameters: {}\n", loaded_model.num_parameters());

            if let Some(input_path) = input {
                println!("Input file evaluation: {}", input_path);
                println!("(Custom input evaluation not yet implemented)");
            } else {
                // Run a simple test with the copy task
                println!("Running validation test with copy task...");
                let task = CopyTask::new(20, loaded_model.config.input_dim);
                let examples = task.examples();

                let mut total_loss = 0.0;
                let mut correct = 0;

                for example in examples {
                    let prediction = loaded_model.forward(&example.input);

                    // Compute MSE loss
                    let diff = &prediction - &example.target;
                    let loss = diff.mapv(|x| x * x).sum() / (prediction.len() as f32);
                    total_loss += loss;

                    // Check if prediction is close to target (within threshold)
                    let max_diff = diff.mapv(|x| x.abs()).iter().cloned().fold(0.0f32, f32::max);
                    if max_diff < 0.5 {
                        correct += 1;
                    }
                }

                let avg_loss = total_loss / examples.len() as f32;
                let accuracy = (correct as f32 / examples.len() as f32) * 100.0;

                println!("  Average loss: {:.6}", avg_loss);
                println!("  Accuracy: {}/{} ({:.2}%)", correct, examples.len(), accuracy);
            }
        }
    }
}
