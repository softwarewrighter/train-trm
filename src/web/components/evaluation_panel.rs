//! Evaluation panel component

use crate::data::tasks::CopyTask;
use crate::model::{TRMConfig, TRMModel};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EvaluationPanelProps {
    #[prop_or_default]
    pub on_evaluation_complete: Callback<()>,
}

pub struct EvaluationPanel {
    model: Option<TRMModel>,
    results: Option<EvalResults>,
    num_examples: String,
}

#[derive(Clone)]
struct EvalResults {
    total: usize,
    correct: usize,
    average_loss: f32,
    examples: Vec<ExampleResult>,
}

#[derive(Clone)]
struct ExampleResult {
    input: Vec<f32>,
    target: Vec<f32>,
    prediction: Vec<f32>,
    correct: bool,
}

pub enum Msg {
    CreateModel,
    RunEvaluation,
    UpdateNumExamples(String),
}

impl Component for EvaluationPanel {
    type Message = Msg;
    type Properties = EvaluationPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        // Auto-create model with same config as training default
        let config = TRMConfig {
            input_dim: 5,
            output_dim: 5,
            hidden_dim: 16,
            latent_dim: 16,
            l_layers: 2,
            h_cycles: 3,
            l_cycles: 4,
        };

        Self {
            model: Some(TRMModel::new(config)),
            results: None,
            num_examples: "20".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateModel => {
                let config = TRMConfig {
                    input_dim: 5,
                    output_dim: 5,
                    hidden_dim: 16,
                    latent_dim: 16,
                    l_layers: 2,
                    h_cycles: 3,
                    l_cycles: 4,
                };
                self.model = Some(TRMModel::new(config));
                true
            }
            Msg::RunEvaluation => {
                if let Some(ref mut model) = self.model {
                    let num_examples: usize = self.num_examples.parse().unwrap_or(20);
                    let task = CopyTask::new(num_examples, 5);
                    let examples = task.examples();

                    let mut total_loss = 0.0;
                    let mut correct = 0;
                    let mut eval_examples = Vec::new();

                    for example in examples {
                        let prediction = model.forward(&example.input);

                        // Compute loss
                        let diff = &prediction - &example.target;
                        let loss = diff.mapv(|x| x * x).sum() / prediction.len() as f32;
                        total_loss += loss;

                        // Check correctness (within 0.5 threshold)
                        let max_diff = diff
                            .mapv(|x| x.abs())
                            .iter()
                            .cloned()
                            .fold(0.0f32, f32::max);
                        let is_correct = max_diff < 0.5;
                        if is_correct {
                            correct += 1;
                        }

                        // Store all results for scrollable display
                        eval_examples.push(ExampleResult {
                            input: example.input.iter().cloned().collect(),
                            target: example.target.iter().cloned().collect(),
                            prediction: prediction.iter().cloned().collect(),
                            correct: is_correct,
                        });
                    }

                    self.results = Some(EvalResults {
                        total: examples.len(),
                        correct,
                        average_loss: total_loss / examples.len() as f32,
                        examples: eval_examples,
                    });

                    // Notify parent that evaluation is complete
                    ctx.props().on_evaluation_complete.emit(());
                    return true;
                }
                false
            }
            Msg::UpdateNumExamples(value) => {
                self.num_examples = value;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="evaluation-panel">
                <h2>{ "Model Evaluation" }</h2>

                <div class="eval-config">
                    <h3>{ "Configuration" }</h3>
                    <div class="form-group">
                        <label>{ "Number of Examples:" }</label>
                        <input
                            type="number"
                            value={self.num_examples.clone()}
                            onchange={ctx.link().callback(|e: Event| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                Msg::UpdateNumExamples(input.map(|i| i.value()).unwrap_or_default())
                            })}
                        />
                    </div>
                </div>

                <button
                    onclick={ctx.link().callback(|_| Msg::RunEvaluation)}
                    class="btn-primary"
                >
                    { "Run Evaluation" }
                </button>

                {if let Some(ref model) = self.model {
                    html! {
                        <div class="model-info">
                            <h3>{ "Model Information" }</h3>
                            <p>{ format!("Input Dimension: {}", model.config.input_dim) }</p>
                            <p>{ format!("Output Dimension: {}", model.config.output_dim) }</p>
                            <p>{ format!("Hidden Dimension: {}", model.config.hidden_dim) }</p>
                            <p>{ format!("Latent Dimension: {}", model.config.latent_dim) }</p>
                            <p>{ format!("Layers: {}", model.config.l_layers) }</p>
                            <p>{ format!("H-Cycles: {}", model.config.h_cycles) }</p>
                            <p>{ format!("L-Cycles: {}", model.config.l_cycles) }</p>
                            <p>{ format!("Parameters: {}", model.num_parameters()) }</p>
                        </div>
                    }
                } else {
                    html! {}
                }}

                {if let Some(ref results) = self.results {
                    html! {
                        <div class="eval-results">
                            <h3>{ "Evaluation Results" }</h3>
                            <div class="results-summary">
                                <div class="metric">
                                    <span class="metric-label">{ "Accuracy:" }</span>
                                    <span class="metric-value">
                                        { format!("{}/{} ({:.1}%)", results.correct, results.total,
                                            (results.correct as f32 / results.total as f32) * 100.0) }
                                    </span>
                                </div>
                                <div class="metric">
                                    <span class="metric-label">{ "Average Loss:" }</span>
                                    <span class="metric-value">{ format!("{:.6}", results.average_loss) }</span>
                                </div>
                            </div>

                            <h4>{ "Example Predictions" }</h4>
                            <div class="examples-container">
                                <div class="examples-table">
                                    <table>
                                        <thead>
                                            <tr>
                                                <th>{ "#" }</th>
                                                <th>{ "Input" }</th>
                                                <th>{ "Target" }</th>
                                                <th>{ "Prediction" }</th>
                                                <th>{ "✓/✗" }</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {for results.examples.iter().enumerate().map(|(i, ex)| {
                                                html! {
                                                    <tr class={if ex.correct { "correct" } else { "incorrect" }}>
                                                        <td>{ i + 1 }</td>
                                                        <td>{ Self::format_vector(&ex.input) }</td>
                                                        <td>{ Self::format_vector(&ex.target) }</td>
                                                        <td>{ Self::format_vector(&ex.prediction) }</td>
                                                        <td>
                                                            {if ex.correct {
                                                                html! { <span class="status-correct">{ "✓" }</span> }
                                                            } else {
                                                                html! { <span class="status-incorrect">{ "✗" }</span> }
                                                            }}
                                                        </td>
                                                    </tr>
                                                }
                                            })}
                                        </tbody>
                                    </table>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <p class="placeholder">{ "No evaluation results yet. Create a model and run evaluation." }</p>
                    }
                }}
            </div>
        }
    }
}

impl EvaluationPanel {
    fn format_vector(vec: &[f32]) -> String {
        format!(
            "[{}]",
            vec.iter()
                .map(|&v| format!("{:.2}", v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
