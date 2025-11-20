//! Maze visualization component

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<Vec<bool>>, // true = wall, false = path
    pub start: (usize, usize),
    pub goal: (usize, usize),
    pub solution: Option<Vec<(usize, usize)>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MazeConfig {
    pub size: usize,
    pub complexity: f64, // 0.0 to 1.0, higher = more complex
}

impl Maze {
    fn new_demo() -> Self {
        Self::new_with_config(10, 0.5)
    }

    fn new_with_config(size: usize, complexity: f64) -> Self {
        use js_sys::Math;

        let width = size;
        let height = size;

        // Start with all paths (no walls initially)
        let mut walls = vec![vec![false; width]; height];

        // Add walls based on complexity
        // complexity: 0.0 = almost no walls (easy), 1.0 = many walls (hard)
        let wall_density = 0.3 + (complexity * 0.4); // 30% to 70% walls

        for (y, row) in walls.iter_mut().enumerate().take(height) {
            for (x, cell) in row.iter_mut().enumerate().take(width) {
                // Don't put walls on start or goal
                if (x, y) == (0, 0) || (x, y) == (width - 1, height - 1) {
                    *cell = false;
                    continue;
                }

                // Randomly place walls based on density
                if Math::random() < wall_density {
                    *cell = true;
                }
            }
        }

        // Ensure there's a path from start to goal using DFS
        // Remove walls if necessary to create a valid path
        let mut path_exists = false;
        let mut attempts = 0;

        while !path_exists && attempts < 10 {
            // Check if path exists
            let mut visited = vec![vec![false; width]; height];
            let mut stack = vec![(0, 0)];
            visited[0][0] = true;

            while let Some((x, y)) = stack.pop() {
                if x == width - 1 && y == height - 1 {
                    path_exists = true;
                    break;
                }

                // Check neighbors
                let neighbors = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx < width && ny < height && !walls[ny][nx] && !visited[ny][nx] {
                        visited[ny][nx] = true;
                        stack.push((nx, ny));
                    }
                }
            }

            if !path_exists {
                // Carve a path by removing some random walls
                for _ in 0..((width + height) / 2) {
                    let rx = (Math::random() * width as f64) as usize % width;
                    let ry = (Math::random() * height as f64) as usize % height;
                    walls[ry][rx] = false;
                }
            }

            attempts += 1;
        }

        // Find solution using BFS
        let mut queue = vec![(0, 0, vec![(0, 0)])];
        let mut found_visited = vec![vec![false; width]; height];
        found_visited[0][0] = true;
        let mut solution = vec![(0, 0)];

        while let Some((x, y, path)) = queue.pop() {
            if x == width - 1 && y == height - 1 {
                solution = path;
                break;
            }

            // Check neighbors
            let neighbors = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];

            for (nx, ny) in neighbors {
                if nx < width && ny < height && !walls[ny][nx] && !found_visited[ny][nx] {
                    found_visited[ny][nx] = true;
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    queue.insert(0, (nx, ny, new_path)); // Insert at front for BFS
                }
            }
        }

        Self {
            width,
            height,
            walls,
            start: (0, 0),
            goal: (width - 1, height - 1),
            solution: Some(solution),
        }
    }
}

pub struct MazeVisualizer {
    maze: Maze,
    canvas_ref: NodeRef,
    show_solution: bool,
    maze_size: usize,
    complexity: f64,
}

pub enum Msg {
    ToggleSolution,
    GenerateNew,
    Rendered,
    UpdateSize(String),
    UpdateComplexity(String),
}

impl Component for MazeVisualizer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            maze: Maze::new_demo(),
            canvas_ref: NodeRef::default(),
            show_solution: false,
            maze_size: 10,
            complexity: 0.5,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSolution => {
                self.show_solution = !self.show_solution;
                // Re-render canvas
                ctx.link().send_message(Msg::Rendered);
                true
            }
            Msg::GenerateNew => {
                self.maze = Maze::new_with_config(self.maze_size, self.complexity);
                self.show_solution = false;
                ctx.link().send_message(Msg::Rendered);
                true
            }
            Msg::UpdateSize(val) => {
                if let Ok(size) = val.parse::<usize>() {
                    self.maze_size = size.clamp(5, 50);
                }
                true
            }
            Msg::UpdateComplexity(val) => {
                if let Ok(complexity) = val.parse::<f64>() {
                    self.complexity = complexity.clamp(0.0, 1.0);
                }
                true
            }
            Msg::Rendered => {
                self.draw_maze();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="maze-container">
                <div class="maze-controls">
                    <h2>{ "Maze Navigation" }</h2>
                    <div class="form-group">
                        <label for="maze-size">{ "Size:" }</label>
                        <input
                            id="maze-size"
                            type="number"
                            value={self.maze_size.to_string()}
                            min="5"
                            max="50"
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateSize(input.value())
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label for="maze-complexity">{ "Complexity:" }</label>
                        <input
                            id="maze-complexity"
                            type="range"
                            value={self.complexity.to_string()}
                            min="0"
                            max="1"
                            step="0.1"
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateComplexity(input.value())
                            })}
                        />
                        <span>{ format!("{:.1}", self.complexity) }</span>
                    </div>
                    <div class="button-group">
                        <button onclick={ctx.link().callback(|_| Msg::GenerateNew)}>
                            { "Generate New Maze" }
                        </button>
                        <button onclick={ctx.link().callback(|_| Msg::ToggleSolution)}>
                            { if self.show_solution { "Hide Solution" } else { "Show Solution" } }
                        </button>
                    </div>
                    <div class="maze-info">
                        <p>{ format!("Size: {}x{}", self.maze.width, self.maze.height) }</p>
                        <p>{ format!("Start: {:?}", self.maze.start) }</p>
                        <p>{ format!("Goal: {:?}", self.maze.goal) }</p>
                    </div>
                </div>
                <div class="maze-canvas-container">
                    <canvas
                        ref={self.canvas_ref.clone()}
                        width="500"
                        height="500"
                    />
                </div>
                <div class="maze-legend">
                    <div class="legend-item">
                        <div class="legend-color" style="background-color: #4CAF50;"></div>
                        <span>{ "Start" }</span>
                    </div>
                    <div class="legend-item">
                        <div class="legend-color" style="background-color: #F44336;"></div>
                        <span>{ "Goal" }</span>
                    </div>
                    <div class="legend-item">
                        <div class="legend-color" style="background-color: #333;"></div>
                        <span>{ "Wall" }</span>
                    </div>
                    <div class="legend-item">
                        <div class="legend-color" style="background-color: #FFF; border: 1px solid #ccc;"></div>
                        <span>{ "Path" }</span>
                    </div>
                    <div class="legend-item">
                        <div class="legend-color" style="background-color: #2196F3;"></div>
                        <span>{ "Solution" }</span>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.draw_maze();
        }
    }
}

impl MazeVisualizer {
    #[allow(deprecated)]
    fn draw_maze(&self) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>();
        if let Some(canvas) = canvas {
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            let width = canvas.width() as f64;
            let height = canvas.height() as f64;
            let cell_width = width / self.maze.width as f64;
            let cell_height = height / self.maze.height as f64;

            // Clear canvas
            context.clear_rect(0.0, 0.0, width, height);

            // Draw maze cells
            for y in 0..self.maze.height {
                for x in 0..self.maze.width {
                    let px = x as f64 * cell_width;
                    let py = y as f64 * cell_height;

                    // Determine cell color
                    let color = if (x, y) == self.maze.start {
                        "#4CAF50" // Green for start
                    } else if (x, y) == self.maze.goal {
                        "#F44336" // Red for goal
                    } else if self.maze.walls[y][x] {
                        "#333333" // Dark gray/black for walls
                    } else {
                        "#FFFFFF" // White for paths
                    };

                    context.set_fill_style(&wasm_bindgen::JsValue::from_str(color));
                    context.fill_rect(px, py, cell_width, cell_height);

                    // Draw grid lines
                    context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#CCCCCC"));
                    context.set_line_width(1.0);
                    context.stroke_rect(px, py, cell_width, cell_height);
                }
            }

            // Draw solution path if enabled
            if self.show_solution {
                if let Some(ref solution) = self.maze.solution {
                    context.set_stroke_style(&wasm_bindgen::JsValue::from_str("#2196F3"));
                    context.set_line_width(3.0);
                    context.begin_path();

                    for (i, &(x, y)) in solution.iter().enumerate() {
                        let px = (x as f64 + 0.5) * cell_width;
                        let py = (y as f64 + 0.5) * cell_height;

                        if i == 0 {
                            context.move_to(px, py);
                        } else {
                            context.line_to(px, py);
                        }
                    }

                    context.stroke();

                    // Draw circles on solution path
                    context.set_fill_style(&wasm_bindgen::JsValue::from_str("#2196F3"));
                    for &(x, y) in solution.iter() {
                        let px = (x as f64 + 0.5) * cell_width;
                        let py = (y as f64 + 0.5) * cell_height;
                        context.begin_path();
                        context
                            .arc(px, py, 4.0, 0.0, 2.0 * std::f64::consts::PI)
                            .unwrap();
                        context.fill();
                    }
                }
            }
        }
    }
}
