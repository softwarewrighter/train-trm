//! Training infrastructure

use crate::model::TRMModel;

/// Trainer for TRM models
pub struct Trainer {
    #[allow(dead_code)]
    model: TRMModel,
}

impl Trainer {
    /// Create a new trainer
    pub fn new(model: TRMModel) -> Self {
        Self { model }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TRMConfig;

    #[test]
    fn test_trainer_creation() {
        let config = TRMConfig::default();
        let model = TRMModel::new(config);
        let _trainer = Trainer::new(model);
        // Just verify it compiles and constructs
    }
}
