//! Tiny Recursive Model implementation

use super::network::{ActivationType, Layer, Network};
use ndarray::{Array2, Axis};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

/// Configuration for TRM model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TRMConfig {
    /// Number of layers in the network (typically 2)
    pub l_layers: usize,
    /// Number of outer cycles (H)
    pub h_cycles: usize,
    /// Number of inner cycles (L) - think steps
    pub l_cycles: usize,
    /// Hidden dimension
    pub hidden_dim: usize,
    /// Latent state dimension
    pub latent_dim: usize,
    /// Input dimension
    pub input_dim: usize,
    /// Output dimension
    pub output_dim: usize,
}

impl Default for TRMConfig {
    fn default() -> Self {
        Self {
            l_layers: 2,
            h_cycles: 3,
            l_cycles: 4,
            hidden_dim: 64,
            latent_dim: 64,
            input_dim: 10,
            output_dim: 10,
        }
    }
}

/// Tiny Recursive Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TRMModel {
    /// Model configuration
    pub config: TRMConfig,
    /// Network for think and act operations
    pub network: Network,
}

impl TRMModel {
    /// Create a new TRM model
    pub fn new(config: TRMConfig) -> Self {
        // Build network architecture
        // For think: concat(x, y, z) -> latent_dim
        // For act: concat(y, z) -> output_dim
        // We use a single network that can handle both by taking max input size

        let think_input_dim = config.input_dim + config.output_dim + config.latent_dim;
        let act_input_dim = config.output_dim + config.latent_dim;
        let max_input_dim = think_input_dim.max(act_input_dim);

        // Build layers
        let mut layers = Vec::new();

        // First layer
        layers.push(Layer::new(
            max_input_dim,
            config.hidden_dim,
            ActivationType::ReLU,
        ));

        // Hidden layers
        for _ in 1..config.l_layers {
            layers.push(Layer::new(
                config.hidden_dim,
                config.hidden_dim,
                ActivationType::ReLU,
            ));
        }

        // Output layer for think (latent_dim)
        // We'll use this for both think and act by adapting the final layer
        layers.push(Layer::new(
            config.hidden_dim,
            config.latent_dim.max(config.output_dim),
            ActivationType::Tanh,
        ));

        let network = Network::new(layers);

        Self { config, network }
    }

    /// Think step: update latent state z given input x, current answer y, and previous z
    fn think(&mut self, x: &Array2<f32>, y: &Array2<f32>, z: &Array2<f32>) -> Array2<f32> {
        // Concatenate [x, y, z] and pad to network input size
        let batch_size = x.shape()[0];
        let think_input_dim =
            self.config.input_dim + self.config.output_dim + self.config.latent_dim;
        let act_input_dim = self.config.output_dim + self.config.latent_dim;
        let max_input_dim = think_input_dim.max(act_input_dim);

        let mut input = Array2::zeros((batch_size, max_input_dim));

        // Fill in the values
        for i in 0..batch_size {
            let mut offset = 0;
            for j in 0..x.shape()[1] {
                input[[i, offset]] = x[[i, j]];
                offset += 1;
            }
            for j in 0..y.shape()[1] {
                input[[i, offset]] = y[[i, j]];
                offset += 1;
            }
            for j in 0..z.shape()[1] {
                input[[i, offset]] = z[[i, j]];
                offset += 1;
            }
            // Remaining elements are already zero (padding)
        }

        // Pass through network and extract latent_dim output
        let output = self.network.forward(&input);
        output
            .slice_axis(Axis(1), ndarray::Slice::from(0..self.config.latent_dim))
            .to_owned()
    }

    /// Act step: update answer y given current y and latent state z
    fn act(&mut self, y: &Array2<f32>, z: &Array2<f32>) -> Array2<f32> {
        // Concatenate [y, z] and pad to network input size
        let batch_size = y.shape()[0];
        let think_input_dim =
            self.config.input_dim + self.config.output_dim + self.config.latent_dim;
        let act_input_dim = self.config.output_dim + self.config.latent_dim;
        let max_input_dim = think_input_dim.max(act_input_dim);

        let mut input = Array2::zeros((batch_size, max_input_dim));

        for i in 0..batch_size {
            let mut offset = 0;
            for j in 0..y.shape()[1] {
                input[[i, offset]] = y[[i, j]];
                offset += 1;
            }
            for j in 0..z.shape()[1] {
                input[[i, offset]] = z[[i, j]];
                offset += 1;
            }
            // Remaining elements are already zero (padding)
        }

        // Pass through network and extract output_dim
        let output = self.network.forward(&input);
        output
            .slice_axis(Axis(1), ndarray::Slice::from(0..self.config.output_dim))
            .to_owned()
    }

    /// Forward pass: recursive reasoning
    pub fn forward(&mut self, x: &Array2<f32>) -> Array2<f32> {
        let batch_size = x.shape()[0];

        // Initialize latent state z and answer y
        let mut z = Array2::zeros((batch_size, self.config.latent_dim));
        let mut y = Array2::zeros((batch_size, self.config.output_dim));

        // Recursive improvement loop
        for _ in 0..self.config.h_cycles {
            // Think phase: update latent state for L cycles
            for _ in 0..self.config.l_cycles {
                z = self.think(x, &y, &z);
            }

            // Act phase: update answer
            y = self.act(&y, &z);
        }

        y
    }

    /// Backward pass and weight update
    pub fn backward_and_update(&mut self, grad_output: &Array2<f32>, learning_rate: f32) {
        // For now, we'll do a simplified backward pass
        // A full implementation would need to backpropagate through all think/act cycles
        // For MVP, we'll just update based on the final output gradient

        // Pad the gradient to match network output size
        let batch_size = grad_output.shape()[0];
        let network_output_dim = self.config.latent_dim.max(self.config.output_dim);
        let mut padded_grad = Array2::zeros((batch_size, network_output_dim));

        // Copy gradient for the output dimensions
        for i in 0..batch_size {
            for j in 0..self.config.output_dim {
                padded_grad[[i, j]] = grad_output[[i, j]];
            }
        }

        self.network
            .backward_and_update(&padded_grad, learning_rate);
    }

    /// Get total number of parameters
    pub fn num_parameters(&self) -> usize {
        self.network.num_parameters()
    }

    /// Save model to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string_pretty(self)?;
        writer.write_all(json.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    /// Load model from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut json = String::new();
        reader.read_to_string(&mut json)?;
        let model: TRMModel = serde_json::from_str(&json)?;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_config_default() {
        let config = TRMConfig::default();
        assert_eq!(config.l_layers, 2);
        assert_eq!(config.h_cycles, 3);
        assert_eq!(config.l_cycles, 4);
    }

    #[test]
    fn test_trm_creation() {
        let config = TRMConfig::default();
        let model = TRMModel::new(config.clone());
        assert_eq!(model.config.l_layers, config.l_layers);
        assert!(!model.network.layers.is_empty());
    }

    #[test]
    fn test_trm_forward_shape() {
        let config = TRMConfig {
            input_dim: 5,
            output_dim: 3,
            hidden_dim: 8,
            latent_dim: 4,
            l_layers: 2,
            h_cycles: 2,
            l_cycles: 2,
        };

        let mut model = TRMModel::new(config);
        let batch_size = 2;
        let input = Array2::zeros((batch_size, 5));

        let output = model.forward(&input);

        assert_eq!(output.shape(), &[batch_size, 3]);
    }

    #[test]
    fn test_trm_forward_runs() {
        let config = TRMConfig::default();
        let mut model = TRMModel::new(config);

        let input = Array2::ones((1, 10));
        let output = model.forward(&input);

        // Should produce some output
        assert_eq!(output.shape(), &[1, 10]);
    }

    #[test]
    fn test_trm_deterministic() {
        let config = TRMConfig::default();
        let mut model = TRMModel::new(config);

        let input = Array2::ones((1, 10));
        let output1 = model.forward(&input);
        let output2 = model.forward(&input);

        // Same input should produce same output
        assert_abs_diff_eq!(output1, output2, epsilon = 1e-6);
    }

    #[test]
    fn test_trm_batch_processing() {
        let config = TRMConfig {
            input_dim: 4,
            output_dim: 2,
            hidden_dim: 8,
            latent_dim: 4,
            l_layers: 2,
            h_cycles: 1,
            l_cycles: 1,
        };

        let mut model = TRMModel::new(config);
        let batch_size = 3;
        let input = Array2::ones((batch_size, 4));

        let output = model.forward(&input);

        assert_eq!(output.shape(), &[batch_size, 2]);
    }

    #[test]
    fn test_num_parameters() {
        let config = TRMConfig::default();
        let model = TRMModel::new(config);

        let num_params = model.num_parameters();

        // Should have parameters from all layers
        assert!(num_params > 0);
    }
}
