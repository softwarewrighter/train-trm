//! CLI entry point for train-trm

use clap::{Parser, Subcommand};

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
            println!("Training TRM model...");
            println!("  Layers: {}", layers);
            println!("  H-cycles: {}", h_cycles);
            println!("  L-cycles: {}", l_cycles);
            println!("  Learning rate: {}", lr);
            println!("  Epochs: {}", epochs);
            println!("  Output: {}", output);

            // TODO: Implement training
            eprintln!("Training not yet implemented");
            std::process::exit(1);
        }
        Commands::Eval { model, input } => {
            println!("Evaluating model: {}", model);
            if let Some(input_path) = input {
                println!("  Input: {}", input_path);
            }

            // TODO: Implement evaluation
            eprintln!("Evaluation not yet implemented");
            std::process::exit(1);
        }
    }
}
