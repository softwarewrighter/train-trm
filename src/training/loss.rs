//! Loss functions for training

use ndarray::Array2;

/// Loss function types
#[derive(Debug, Clone, Copy)]
pub enum LossType {
    /// Mean Squared Error
    MSE,
    /// Mean Absolute Error
    MAE,
}

/// Compute loss between predictions and targets
pub fn compute_loss(predictions: &Array2<f32>, targets: &Array2<f32>, loss_type: LossType) -> f32 {
    match loss_type {
        LossType::MSE => mse_loss(predictions, targets),
        LossType::MAE => mae_loss(predictions, targets),
    }
}

/// Mean Squared Error loss
fn mse_loss(predictions: &Array2<f32>, targets: &Array2<f32>) -> f32 {
    let diff = predictions - targets;
    let squared = diff.mapv(|x| x * x);
    squared.sum() / (predictions.len() as f32)
}

/// Mean Absolute Error loss
fn mae_loss(predictions: &Array2<f32>, targets: &Array2<f32>) -> f32 {
    let diff = predictions - targets;
    let abs_diff = diff.mapv(|x| x.abs());
    abs_diff.sum() / (predictions.len() as f32)
}

/// Compute gradient of MSE loss with respect to predictions
pub fn mse_gradient(predictions: &Array2<f32>, targets: &Array2<f32>) -> Array2<f32> {
    // d/dx (x - t)^2 = 2(x - t) / n
    let n = predictions.len() as f32;
    (predictions - targets) * (2.0 / n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use ndarray::array;

    #[test]
    fn test_mse_loss_zero() {
        let predictions = array![[1.0, 2.0], [3.0, 4.0]];
        let targets = predictions.clone();
        let loss = mse_loss(&predictions, &targets);
        assert_abs_diff_eq!(loss, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_mse_loss_nonzero() {
        let predictions = array![[1.0], [2.0]];
        let targets = array![[2.0], [3.0]];
        // MSE = ((1-2)^2 + (2-3)^2) / 2 = (1 + 1) / 2 = 1.0
        let loss = mse_loss(&predictions, &targets);
        assert_abs_diff_eq!(loss, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_mae_loss_zero() {
        let predictions = array![[1.0, 2.0], [3.0, 4.0]];
        let targets = predictions.clone();
        let loss = mae_loss(&predictions, &targets);
        assert_abs_diff_eq!(loss, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_mae_loss_nonzero() {
        let predictions = array![[1.0], [3.0]];
        let targets = array![[2.0], [4.0]];
        // MAE = (|1-2| + |3-4|) / 2 = (1 + 1) / 2 = 1.0
        let loss = mae_loss(&predictions, &targets);
        assert_abs_diff_eq!(loss, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_compute_loss_mse() {
        let predictions = array![[1.0], [2.0]];
        let targets = array![[2.0], [3.0]];
        let loss = compute_loss(&predictions, &targets, LossType::MSE);
        assert_abs_diff_eq!(loss, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_compute_loss_mae() {
        let predictions = array![[1.0], [3.0]];
        let targets = array![[2.0], [4.0]];
        let loss = compute_loss(&predictions, &targets, LossType::MAE);
        assert_abs_diff_eq!(loss, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_mse_gradient() {
        let predictions = array![[1.0, 2.0]];
        let targets = array![[2.0, 1.0]];
        let grad = mse_gradient(&predictions, &targets);

        // d/dx MSE = 2(x - t) / n
        // For x=1, t=2: 2(1-2)/2 = -1.0
        // For x=2, t=1: 2(2-1)/2 = 1.0
        assert_abs_diff_eq!(grad[[0, 0]], -1.0, epsilon = 1e-6);
        assert_abs_diff_eq!(grad[[0, 1]], 1.0, epsilon = 1e-6);
    }
}
