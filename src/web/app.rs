use crate::web::components::{EvaluationPanel, MazeVisualizer, TrainingPanel};
use yew::prelude::*;

// Build information from build.rs
const BUILD_COMMIT: &str = env!("BUILD_GIT_COMMIT");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
const BUILD_HOSTNAME: &str = env!("BUILD_HOSTNAME");
const COPYRIGHT: &str = "Copyright (c) 2025 Michael A Wright";
const LICENSE_URL: &str = "https://github.com/softwarewrighter/train-trm/blob/main/LICENSE";

pub struct App {
    show_train_callout: bool,
    show_eval_callout: bool,
}

pub enum Msg {
    HideTrainCallout,
    ShowEvalCallout,
    HideEvalCallout,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            show_train_callout: true,
            show_eval_callout: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HideTrainCallout => {
                self.show_train_callout = false;
                self.show_eval_callout = true;
                true
            }
            Msg::ShowEvalCallout => {
                self.show_eval_callout = true;
                true
            }
            Msg::HideEvalCallout => {
                self.show_eval_callout = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <header class="app-header">
                    <div style="display: flex; align-items: center;">
                        <h1>{ "TRM" }</h1>
                        <span class="subtitle">{ "Tiny Recursive Model for Maze Navigation" }</span>
                    </div>
                    <a href="https://github.com/softwarewrighter/train-trm" class="github-corner" target="_blank" title="View source on GitHub">
                        <svg viewBox="0 0 250 250" aria-hidden="true">
                            <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                            <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                            <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                        </svg>
                    </a>
                </header>

                <main class="app-main">
                    // Training Panel
                    <div class="panel panel-train">
                        { if self.show_train_callout {
                            html! {
                                <div class="callout">
                                    { "👈 Start here: Train your model!" }
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                        <div class="panel-header">{ "1. Train" }</div>
                        <div class="panel-content">
                            <TrainingPanel on_training_complete={ctx.link().callback(|_| Msg::HideTrainCallout)} />
                        </div>
                    </div>

                    // Evaluation Panel
                    <div class="panel panel-eval">
                        { if self.show_eval_callout {
                            html! {
                                <div class="callout">
                                    { "👈 Next: Evaluate your model!" }
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                        <div class="panel-header">{ "2. Evaluate" }</div>
                        <div class="panel-content">
                            <EvaluationPanel on_evaluation_complete={ctx.link().callback(|_| Msg::HideEvalCallout)} />
                        </div>
                    </div>

                    // Visualization Panel
                    <div class="panel panel-viz">
                        <div class="panel-header">{ "3. Visualize" }</div>
                        <div class="panel-content">
                            <MazeVisualizer />
                        </div>
                    </div>
                </main>

                <footer class="app-footer">
                    <div class="footer-left">
                        <span>{ COPYRIGHT }</span>
                        <a href={ LICENSE_URL } target="_blank" class="footer-link">{ "License" }</a>
                    </div>
                    <div class="footer-right">
                        <span class="build-info">
                            { format!("Built on {} | Commit {} | {}",
                                BUILD_HOSTNAME,
                                &BUILD_COMMIT[..7],
                                BUILD_TIMESTAMP) }
                        </span>
                    </div>
                </footer>
            </>
        }
    }
}
