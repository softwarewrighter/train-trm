//! TRM model and neural network components

mod network;
mod trm;

pub use network::{ActivationType, Layer, Network};
pub use trm::{TRMConfig, TRMModel};
