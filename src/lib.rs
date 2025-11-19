//! # train-trm
//!
//! A Rust implementation of the Tiny Recursive Model (TRM) for recursive reasoning tasks.
//!
//! ## Overview
//!
//! TRM is a small neural network that uses recursive reasoning to solve complex tasks.
//! Unlike traditional deep networks, TRM uses iteration (think/act cycles) rather than
//! depth to achieve reasoning capabilities.
//!
//! ## Architecture
//!
//! - **Think Step**: Updates latent state based on problem and current solution
//! - **Act Step**: Updates solution based on latent state
//! - **Recursive Loop**: Iterates think/act cycles to refine the answer
//!
//! ## Modules
//!
//! - `model`: Core TRM model and neural network components
//! - `training`: Training infrastructure and optimizers
//! - `data`: Data structures and task definitions
//! - `utils`: Helper functions and utilities
//! - `web`: Web UI components (requires `web` feature)

pub mod data;
pub mod model;
pub mod training;
pub mod utils;

#[cfg(feature = "web")]
pub mod web;

pub use model::{TRMConfig, TRMModel};
pub use training::Trainer;

/// Re-export common types
pub mod prelude {
    pub use crate::data::Problem;
    pub use crate::model::{TRMConfig, TRMModel};
    pub use crate::training::Trainer;
    pub use crate::utils::Result;
}
