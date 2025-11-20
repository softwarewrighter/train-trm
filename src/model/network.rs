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
    /// Cached input from forward pass (for backprop)
    #[serde(skip)]
    cached_input: Option<Array2<f32>>,
    /// Cached pre-activation values (for backprop)
    #[serde(skip)]
    cached_linear: Option<Array2<f32>>,
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
            cached_input: None,
            cached_linear: None,
        }
    }

    /// Forward pass through the layer
    pub fn forward(&mut self, input: &Array2<f32>) -> Array2<f32> {
        // input shape: (batch_size, input_dim)
        // weights shape: (output_dim, input_dim)
        // output shape: (batch_size, output_dim)

        // Linear transformation: input @ weights.T + bias
        let linear = input.dot(&self.weights.t()) + &self.bias;

        // Cache values for backward pass
        self.cached_input = Some(input.clone());
        self.cached_linear = Some(linear.clone());

        // Apply activation
        self.activation.apply(&linear)
    }

    /// Backward pass through the layer
    /// Returns gradient with respect to input
    pub fn backward(&self, grad_output: &Array2<f32>) -> (Array2<f32>, Array2<f32>, Array1<f32>) {
        let input = self
            .cached_input
            .as_ref()
            .expect("Forward must be called before backward");
        let linear = self
            .cached_linear
            .as_ref()
            .expect("Forward must be called before backward");

        // Gradient through activation
        let activation_grad = self.activation.derivative(linear);
        let grad_linear = grad_output * &activation_grad;

        // Gradient with respect to weights: grad_linear.T @ input
        let grad_weights = grad_linear.t().dot(input);

        // Gradient with respect to bias: sum over batch dimension
        let grad_bias = grad_linear.sum_axis(ndarray::Axis(0));

        // Gradient with respect to input: grad_linear @ weights
        let grad_input = grad_linear.dot(&self.weights);

        (grad_input, grad_weights, grad_bias)
    }

    /// Update weights and biases using gradients
    pub fn update(
        &mut self,
        grad_weights: &Array2<f32>,
        grad_bias: &Array1<f32>,
        learning_rate: f32,
    ) {
        self.weights = &self.weights - &(grad_weights * learning_rate);
        self.bias = &self.bias - &(grad_bias * learning_rate);
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
    pub fn forward(&mut self, input: &Array2<f32>) -> Array2<f32> {
        let mut x = input.clone();
        for layer in &mut self.layers {
            x = layer.forward(&x);
        }
        x
    }

    /// Backward pass and update weights
    pub fn backward_and_update(&mut self, grad_output: &Array2<f32>, learning_rate: f32) {
        let mut grad = grad_output.clone();
        let mut gradients = Vec::new();

        // First pass: compute all gradients
        for layer in self.layers.iter().rev() {
            let (grad_input, grad_weights, grad_bias) = layer.backward(&grad);
            gradients.push((grad_weights, grad_bias));
            grad = grad_input;
        }

        // Second pass: update weights (in forward order)
        gradients.reverse();
        for (layer, (grad_weights, grad_bias)) in self.layers.iter_mut().zip(gradients.iter()) {
            layer.update(grad_weights, grad_bias, learning_rate);
        }
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
        let mut layer = Layer::new(3, 2, ActivationType::ReLU);
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
        let mut network = Network::new(vec![layer1, layer2]);

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
