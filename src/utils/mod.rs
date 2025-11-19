//! Utility functions and common types

use thiserror::Error;

/// Custom error type for TRM operations
#[derive(Debug, Error)]
pub enum TRMError {
    #[error("Invalid input dimension: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Model not initialized")]
    NotInitialized,

    #[error("Training error: {0}")]
    TrainingError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Result type alias using TRMError
pub type Result<T> = std::result::Result<T, TRMError>;
