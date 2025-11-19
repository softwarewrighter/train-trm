//! Neural network layer implementations

use ndarray::{Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use serde::{Deserialize, Serialize};

/// Activation function type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ActivationType {
    /// Rectified Linear Unit
    ReLU,
    /// Hyperbolic tangent
    Tanh,
    /// No activation (identity)
    Identity,
}

impl ActivationType {
    /// Apply activation function element-wise
    pub fn apply(&self, x: &Array2<f32>) -> Array2<f32> {
        match self {
            ActivationType::ReLU => x.mapv(|v| v.max(0.0)),
            ActivationType::Tanh => x.mapv(|v| v.tanh()),
            ActivationType::Identity => x.clone(),
        }
    }

    /// Compute derivative of activation function
    pub fn derivative(&self, x: &Array2<f32>) -> Array2<f32> {
        match self {
            ActivationType::ReLU => x.mapv(|v| if v > 0.0 { 1.0 } else { 0.0 }),
            ActivationType::Tanh => {
                let tanh_x = x.mapv(|v| v.tanh());
                tanh_x.mapv(|v| 1.0 - v * v)
            }
            ActivationType::Identity => Array2::ones(x.dim()),
        }
    }
}

/// A single neural network layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Weight matrix (output_dim x input_dim)
    pub weights: Array2<f32>,
    /// Bias vector (output_dim)
    pub bias: Array1<f32>,
    /// Activation function
    pub activation: ActivationType,
}

impl Layer {
    /// Create a new layer with random initialization
    pub fn new(input_dim: usize, output_dim: usize, activation: ActivationType) -> Self {
        // Xavier/Glorot initialization
        let scale = (2.0 / (input_dim + output_dim) as f32).sqrt();
        let weights = Array2::random((output_dim, input_dim), Uniform::new(-scale, scale));
        let bias = Array1::zeros(output_dim);

        Self {
            weights,
            bias,
            activation,
        }
    }

    /// Forward pass through the layer
    pub fn forward(&self, input: &Array2<f32>) -> Array2<f32> {
        // input shape: (batch_size, input_dim)
        // weights shape: (output_dim, input_dim)
        // output shape: (batch_size, output_dim)

        // Linear transformation: input @ weights.T + bias
        let linear = input.dot(&self.weights.t()) + &self.bias;

        // Apply activation
        self.activation.apply(&linear)
    }
}

/// Multi-layer neural network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// Layers in the network
    pub layers: Vec<Layer>,
}

impl Network {
    /// Create a new network
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    /// Forward pass through all layers
    pub fn forward(&self, input: &Array2<f32>) -> Array2<f32> {
        let mut x = input.clone();
        for layer in &self.layers {
            x = layer.forward(&x);
        }
        x
    }

    /// Get total number of parameters
    pub fn num_parameters(&self) -> usize {
        self.layers
            .iter()
            .map(|layer| layer.weights.len() + layer.bias.len())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use ndarray::array;

    #[test]
    fn test_activation_relu() {
        let input = array![[-1.0, 0.0], [1.0, 2.0]];
        let output = ActivationType::ReLU.apply(&input);
        let expected = array![[0.0, 0.0], [1.0, 2.0]];
        assert_abs_diff_eq!(output, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_activation_tanh() {
        let input = array![[0.0], [1.0]];
        let output = ActivationType::Tanh.apply(&input);
        assert_abs_diff_eq!(output[[0, 0]], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(output[[1, 0]], 1.0_f32.tanh(), epsilon = 1e-6);
    }

    #[test]
    fn test_activation_identity() {
        let input = array![[-1.0, 2.0], [3.0, -4.0]];
        let output = ActivationType::Identity.apply(&input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_relu_derivative() {
        let input = array![[-1.0, 0.0], [1.0, 2.0]];
        let deriv = ActivationType::ReLU.derivative(&input);
        let expected = array![[0.0, 0.0], [1.0, 1.0]];
        assert_abs_diff_eq!(deriv, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new(10, 5, ActivationType::ReLU);
        assert_eq!(layer.weights.shape(), &[5, 10]);
        assert_eq!(layer.bias.len(), 5);
        assert_eq!(layer.activation, ActivationType::ReLU);
    }

    #[test]
    fn test_layer_forward_shape() {
        let layer = Layer::new(3, 2, ActivationType::ReLU);
        let input = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]; // batch_size=2, input_dim=3
        let output = layer.forward(&input);
        assert_eq!(output.shape(), &[2, 2]); // batch_size=2, output_dim=2
    }

    #[test]
    fn test_layer_forward_applies_relu() {
        let mut layer = Layer::new(2, 2, ActivationType::ReLU);
        // Set weights and bias to known values
        layer.weights = array![[-1.0, 1.0], [1.0, 1.0]];
        layer.bias = array![0.0, 0.0];

        let input = array![[1.0, 1.0]];
        let output = layer.forward(&input);

        // First neuron: -1*1 + 1*1 = 0, ReLU -> 0
        // Second neuron: 1*1 + 1*1 = 2, ReLU -> 2
        assert_abs_diff_eq!(output[[0, 0]], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(output[[0, 1]], 2.0, epsilon = 1e-6);
    }

    #[test]
    fn test_network_creation() {
        let layer1 = Layer::new(10, 5, ActivationType::ReLU);
        let layer2 = Layer::new(5, 2, ActivationType::Tanh);
        let network = Network::new(vec![layer1, layer2]);
        assert_eq!(network.layers.len(), 2);
    }

    #[test]
    fn test_network_forward() {
        let layer1 = Layer::new(3, 4, ActivationType::ReLU);
        let layer2 = Layer::new(4, 2, ActivationType::Identity);
        let network = Network::new(vec![layer1, layer2]);

        let input = array![[1.0, 2.0, 3.0]];
        let output = network.forward(&input);

        assert_eq!(output.shape(), &[1, 2]);
    }

    #[test]
    fn test_network_num_parameters() {
        let layer1 = Layer::new(10, 5, ActivationType::ReLU);
        // weights: 10*5 = 50, bias: 5 = 55 total
        let layer2 = Layer::new(5, 2, ActivationType::Tanh);
        // weights: 5*2 = 10, bias: 2 = 12 total
        let network = Network::new(vec![layer1, layer2]);

        assert_eq!(network.num_parameters(), 55 + 12);
    }
}
