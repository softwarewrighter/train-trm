//! Training panel component

use crate::data::tasks::CopyTask;
use crate::data::TrainingExample;
use crate::model::{TRMConfig, TRMModel};
use crate::training::{Trainer, TrainingConfig};
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TrainingPanelProps {
    #[prop_or_default]
    pub on_training_complete: Callback<()>,
}

pub struct TrainingPanel {
    // Training state
    trainer: Option<Trainer>,
    training_examples: Vec<TrainingExample>,
    training_active: bool,
    current_epoch: usize,

    // Training progress
    losses: Vec<f32>,
    current_loss: Option<f32>,

    // Configuration
    epochs: String,
    learning_rate: String,
    layers: String,
    h_cycles: String,
    l_cycles: String,

    // Training interval
    _interval: Option<Interval>,
}

pub enum Msg {
    StartTraining,
    StopTraining,
    TrainEpoch,
    UpdateEpochs(String),
    UpdateLearningRate(String),
    UpdateLayers(String),
    UpdateHCycles(String),
    UpdateLCycles(String),
    ResetModel,
}

impl Component for TrainingPanel {
    type Message = Msg;
    type Properties = TrainingPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        // Create training examples once (80 examples, 5-element sequences)
        let task = CopyTask::new(80, 5);
        let (training_examples, _) = task.split(1.0);

        Self {
            trainer: None,
            training_examples,
            training_active: false,
            current_epoch: 0,
            losses: Vec::new(),
            current_loss: None,
            epochs: "100".to_string(),
            learning_rate: "0.01".to_string(),
            layers: "2".to_string(),
            h_cycles: "3".to_string(),
            l_cycles: "4".to_string(),
            _interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartTraining => {
                // Parse configuration
                let epochs: usize = self.epochs.parse().unwrap_or(100);
                let learning_rate: f32 = self.learning_rate.parse().unwrap_or(0.01);
                let layers: usize = self.layers.parse().unwrap_or(2);
                let h_cycles: usize = self.h_cycles.parse().unwrap_or(3);
                let l_cycles: usize = self.l_cycles.parse().unwrap_or(4);

                // Create model
                let model_config = TRMConfig {
                    input_dim: 5,
                    output_dim: 5,
                    hidden_dim: 16,
                    latent_dim: 16,
                    l_layers: layers,
                    h_cycles,
                    l_cycles,
                };

                let model = TRMModel::new(model_config);

                // Create training config
                let train_config = TrainingConfig {
                    learning_rate,
                    epochs,
                    batch_size: 16,
                    ..Default::default()
                };

                self.trainer = Some(Trainer::new(model, train_config));
                self.training_active = true;
                self.current_epoch = 0;
                self.losses.clear();
                self.current_loss = None;

                // Start training loop with interval
                let link = ctx.link().clone();
                let interval = Interval::new(10, move || {
                    link.send_message(Msg::TrainEpoch);
                });
                self._interval = Some(interval);

                true
            }
            Msg::StopTraining => {
                self.training_active = false;
                self._interval = None;
                // Notify parent that training is complete
                ctx.props().on_training_complete.emit(());
                true
            }
            Msg::TrainEpoch => {
                if let Some(ref mut trainer) = self.trainer {
                    if self.training_active
                        && self.current_epoch < self.epochs.parse().unwrap_or(100)
                    {
                        // Train one epoch with backpropagation using stored examples
                        let mut total_loss = 0.0;
                        let learning_rate: f32 = self.learning_rate.parse().unwrap_or(0.01);

                        for example in &self.training_examples {
                            let prediction = trainer.model_mut().forward(&example.input);
                            let diff = &prediction - &example.target;
                            let loss = diff.mapv(|x| x * x).sum() / prediction.len() as f32;
                            total_loss += loss;

                            // Backpropagation: compute gradient and update weights
                            let n = prediction.len() as f32;
                            let grad_output = &diff * (2.0 / n);
                            trainer
                                .model_mut()
                                .backward_and_update(&grad_output, learning_rate);
                        }

                        let avg_loss = total_loss / self.training_examples.len() as f32;
                        self.losses.push(avg_loss);
                        self.current_loss = Some(avg_loss);
                        self.current_epoch += 1;

                        // Stop if completed
                        if self.current_epoch >= self.epochs.parse().unwrap_or(100) {
                            ctx.link().send_message(Msg::StopTraining);
                        }

                        return true;
                    }
                }
                false
            }
            Msg::UpdateEpochs(value) => {
                self.epochs = value;
                true
            }
            Msg::UpdateLearningRate(value) => {
                self.learning_rate = value;
                true
            }
            Msg::UpdateLayers(value) => {
                self.layers = value;
                true
            }
            Msg::UpdateHCycles(value) => {
                self.h_cycles = value;
                true
            }
            Msg::UpdateLCycles(value) => {
                self.l_cycles = value;
                true
            }
            Msg::ResetModel => {
                // Regenerate training examples
                let task = CopyTask::new(80, 5);
                let (training_examples, _) = task.split(1.0);
                self.training_examples = training_examples;

                self.trainer = None;
                self.training_active = false;
                self.current_epoch = 0;
                self.losses.clear();
                self.current_loss = None;
                self._interval = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="training-panel">
                <h2>{ "Model Training" }</h2>

                <div class="training-config">
                    <h3>{ "Configuration" }</h3>
                    <div class="form-group">
                        <label>{ "Epochs:" }</label>
                        <input
                            type="number"
                            value={self.epochs.clone()}
                            onchange={ctx.link().callback(|e: Event| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                Msg::UpdateEpochs(input.map(|i| i.value()).unwrap_or_default())
                            })}
                            disabled={self.training_active}
                        />
                    </div>
                    <div class="form-group">
                        <label>{ "Learning Rate:" }</label>
                        <input
                            type="number"
                            step="0.001"
                            value={self.learning_rate.clone()}
                            onchange={ctx.link().callback(|e: Event| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                Msg::UpdateLearningRate(input.map(|i| i.value()).unwrap_or_default())
                            })}
                            disabled={self.training_active}
                        />
                    </div>
                    <div class="form-group">
                        <label>{ "Layers:" }</label>
                        <input
                            type="number"
                            value={self.layers.clone()}
                            onchange={ctx.link().callback(|e: Event| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                Msg::UpdateLayers(input.map(|i| i.value()).unwrap_or_default())
                            })}
                            disabled={self.training_active}
                        />
                    </div>
                    <div class="form-group">
                        <label>{ "H-Cycles:" }</label>
                        <input
                            type="number"
                            value={self.h_cycles.clone()}
                            onchange={ctx.link().callback(|e: Event| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                Msg::UpdateHCycles(input.map(|i| i.value()).unwrap_or_default())
                            })}
                            disabled={self.training_active}
                        />
                    </div>
                    <div class="form-group">
                        <label>{ "L-Cycles:" }</label>
                        <input
                            type="number"
                            value={self.l_cycles.clone()}
                            onchange={ctx.link().callback(|e: Event| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                Msg::UpdateLCycles(input.map(|i| i.value()).unwrap_or_default())
                            })}
                            disabled={self.training_active}
                        />
                    </div>
                </div>

                <div class="training-controls">
                    {if !self.training_active {
                        html! {
                            <>
                                <button
                                    onclick={ctx.link().callback(|_| Msg::StartTraining)}
                                    class="btn-primary"
                                >
                                    { "Start Training" }
                                </button>
                                {if self.trainer.is_some() {
                                    html! {
                                        <button
                                            onclick={ctx.link().callback(|_| Msg::ResetModel)}
                                            class="btn-secondary"
                                        >
                                            { "Reset Model" }
                                        </button>
                                    }
                                } else {
                                    html! {}
                                }}
                            </>
                        }
                    } else {
                        html! {
                            <button
                                onclick={ctx.link().callback(|_| Msg::StopTraining)}
                                class="btn-danger"
                            >
                                { "Stop Training" }
                            </button>
                        }
                    }}
                </div>

                <div class="training-progress">
                    <h3>{ "Progress" }</h3>
                    {if self.current_loss.is_some() {
                        html! {
                            <>
                                <div class="progress-info">
                                    <p>{ format!("Epoch: {}/{}", self.current_epoch, self.epochs) }</p>
                                    <p>{ format!("Current Loss: {:.6}", self.current_loss.unwrap()) }</p>
                                    {if !self.losses.is_empty() {
                                        html! {
                                            <p>{ format!("Initial Loss: {:.6}", self.losses[0]) }</p>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                                <div class="loss-chart">
                                    { self.render_loss_chart() }
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <p class="placeholder">{ "No training data yet. Click 'Start Training' to begin." }</p>
                        }
                    }}
                </div>
            </div>
        }
    }
}

impl TrainingPanel {
    fn render_loss_chart(&self) -> Html {
        if self.losses.is_empty() {
            return html! { <p>{ "No data" }</p> };
        }

        let max_loss = self.losses.iter().cloned().fold(0.0f32, f32::max);
        let min_loss = self.losses.iter().cloned().fold(f32::INFINITY, f32::min);
        let range = max_loss - min_loss;

        html! {
            <div class="chart-container">
                <svg width="400" height="200" viewBox="0 0 400 200">
                    <polyline
                        points={self.losses.iter().enumerate().map(|(i, &loss)| {
                            let x = (i as f32 / self.losses.len() as f32) * 380.0 + 10.0;
                            let y = if range > 0.0 {
                                190.0 - ((loss - min_loss) / range) * 170.0
                            } else {
                                100.0
                            };
                            format!("{},{}", x, y)
                        }).collect::<Vec<_>>().join(" ")}
                        fill="none"
                        stroke="#2196F3"
                        stroke-width="2"
                    />
                </svg>
                <div class="chart-labels">
                    <span>{ format!("Max: {:.4}", max_loss) }</span>
                    <span>{ format!("Min: {:.4}", min_loss) }</span>
                </div>
            </div>
        }
    }
}
