//! Maze generation and solving task

use rand::Rng;
use std::collections::VecDeque;

/// Direction in the maze
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Cell type in the maze
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Wall,
    Path,
    Start,
    Goal,
}

impl Cell {
    /// Convert cell to f32 for neural network
    pub fn to_f32(self) -> f32 {
        match self {
            Cell::Wall => 0.0,
            Cell::Path => 0.25,
            Cell::Start => 0.5,
            Cell::Goal => 1.0,
        }
    }

    /// Convert from f32 (for decoding)
    pub fn from_f32(value: f32) -> Self {
        if value < 0.125 {
            Cell::Wall
        } else if value < 0.375 {
            Cell::Path
        } else if value < 0.75 {
            Cell::Start
        } else {
            Cell::Goal
        }
    }
}

/// A maze instance
#[derive(Debug, Clone)]
pub struct Maze {
    /// Maze grid
    pub grid: Vec<Vec<Cell>>,
    /// Width of maze
    pub width: usize,
    /// Height of maze
    pub height: usize,
    /// Start position (row, col)
    pub start: (usize, usize),
    /// Goal position (row, col)
    pub goal: (usize, usize),
    /// Solution path (if computed)
    pub solution: Option<Vec<(usize, usize)>>,
}

impl Maze {
    /// Create a new empty maze
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::Wall; width]; height];
        Self {
            grid,
            width,
            height,
            start: (1, 1),
            goal: (height - 2, width - 2),
            solution: None,
        }
    }

    /// Generate a random maze using recursive backtracking
    pub fn generate_random(width: usize, height: usize) -> Self {
        let mut maze = Self::new(width, height);
        let mut rng = rand::thread_rng();

        // Start from (1, 1)
        maze.carve_path(1, 1, &mut rng);

        // Set start and goal
        maze.grid[maze.start.0][maze.start.1] = Cell::Start;
        maze.grid[maze.goal.0][maze.goal.1] = Cell::Goal;

        maze
    }

    /// Carve a path through the maze recursively
    fn carve_path(&mut self, row: usize, col: usize, rng: &mut impl Rng) {
        self.grid[row][col] = Cell::Path;

        let mut directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        // Shuffle directions
        for i in (1..directions.len()).rev() {
            let j = rng.gen_range(0..=i);
            directions.swap(i, j);
        }

        for dir in directions {
            let (new_row, new_col) = match dir {
                Direction::Up if row >= 2 => (row - 2, col),
                Direction::Down if row + 2 < self.height => (row + 2, col),
                Direction::Left if col >= 2 => (row, col - 2),
                Direction::Right if col + 2 < self.width => (row, col + 2),
                _ => continue,
            };

            if self.grid[new_row][new_col] == Cell::Wall {
                // Carve path between current and new cell
                let (mid_row, mid_col) = match dir {
                    Direction::Up => (row - 1, col),
                    Direction::Down => (row + 1, col),
                    Direction::Left => (row, col - 1),
                    Direction::Right => (row, col + 1),
                };

                self.grid[mid_row][mid_col] = Cell::Path;
                self.carve_path(new_row, new_col, rng);
            }
        }
    }

    /// Solve the maze using BFS
    pub fn solve(&mut self) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut parent = vec![vec![None; self.width]; self.height];

        queue.push_back(self.start);
        visited[self.start.0][self.start.1] = true;

        while let Some((row, col)) = queue.pop_front() {
            if (row, col) == self.goal {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current = Some(self.goal);

                while let Some(pos) = current {
                    path.push(pos);
                    current = parent[pos.0][pos.1];
                }

                path.reverse();
                self.solution = Some(path);
                return true;
            }

            for dir in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let (new_row, new_col) = match dir {
                    Direction::Up if row > 0 => (row - 1, col),
                    Direction::Down if row + 1 < self.height => (row + 1, col),
                    Direction::Left if col > 0 => (row, col - 1),
                    Direction::Right if col + 1 < self.width => (row, col + 1),
                    _ => continue,
                };

                if !visited[new_row][new_col] && self.grid[new_row][new_col] != Cell::Wall {
                    visited[new_row][new_col] = true;
                    parent[new_row][new_col] = Some((row, col));
                    queue.push_back((new_row, new_col));
                }
            }
        }

        false
    }

    /// Convert maze to flat array for neural network
    pub fn to_array(&self) -> Vec<f32> {
        self.grid
            .iter()
            .flat_map(|row| row.iter().map(|&cell| cell.to_f32()))
            .collect()
    }

    /// Convert solution path to direction array
    pub fn solution_to_directions(&self) -> Option<Vec<Direction>> {
        let solution = self.solution.as_ref()?;
        let mut directions = Vec::new();

        for i in 0..solution.len() - 1 {
            let (r1, c1) = solution[i];
            let (r2, c2) = solution[i + 1];

            let dir = if r2 > r1 {
                Direction::Down
            } else if r2 < r1 {
                Direction::Up
            } else if c2 > c1 {
                Direction::Right
            } else {
                Direction::Left
            };

            directions.push(dir);
        }

        Some(directions)
    }

    /// Convert to SVG string
    pub fn to_svg(&self, cell_size: usize) -> String {
        let width_px = self.width * cell_size;
        let height_px = self.height * cell_size;

        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            width_px, height_px, width_px, height_px
        );

        svg.push_str("\n  <!-- Maze grid -->");

        // Draw cells
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let x = col_idx * cell_size;
                let y = row_idx * cell_size;

                let (fill, stroke) = match cell {
                    Cell::Wall => ("#2c3e50", "#34495e"),
                    Cell::Path => ("#ecf0f1", "#bdc3c7"),
                    Cell::Start => ("#2ecc71", "#27ae60"),
                    Cell::Goal => ("#e74c3c", "#c0392b"),
                };

                svg.push_str(&format!(
                    "\n  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>",
                    x, y, cell_size, cell_size, fill, stroke
                ));
            }
        }

        // Draw solution path if available
        if let Some(path) = &self.solution {
            svg.push_str("\n  <!-- Solution path -->");
            if path.len() > 1 {
                svg.push_str("\n  <polyline points=\"");

                for &(row, col) in path {
                    let x = col * cell_size + cell_size / 2;
                    let y = row * cell_size + cell_size / 2;
                    svg.push_str(&format!("{},{} ", x, y));
                }

                svg.push_str("\" fill=\"none\" stroke=\"#3498db\" stroke-width=\"3\" stroke-linecap=\"round\" stroke-linejoin=\"round\" opacity=\"0.8\"/>");
            }
        }

        svg.push_str("\n</svg>");
        svg
    }

    /// Print maze as ASCII art
    pub fn print(&self) {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let on_path = self
                    .solution
                    .as_ref()
                    .map(|path| path.contains(&(row_idx, col_idx)))
                    .unwrap_or(false);

                let ch = if on_path && cell == Cell::Path {
                    '·'
                } else {
                    match cell {
                        Cell::Wall => '█',
                        Cell::Path => ' ',
                        Cell::Start => 'S',
                        Cell::Goal => 'G',
                    }
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

/// Maze solving task
pub struct MazeTask {
    mazes: Vec<Maze>,
    #[allow(dead_code)]
    width: usize,
    #[allow(dead_code)]
    height: usize,
}

impl MazeTask {
    /// Create a new maze task
    pub fn new(num_mazes: usize, width: usize, height: usize) -> Self {
        let mut mazes = Vec::new();

        for _ in 0..num_mazes {
            let mut maze = Maze::generate_random(width, height);
            maze.solve();
            mazes.push(maze);
        }

        Self {
            mazes,
            width,
            height,
        }
    }

    /// Get all mazes
    pub fn mazes(&self) -> &[Maze] {
        &self.mazes
    }

    /// Get a specific maze
    pub fn get(&self, index: usize) -> Option<&Maze> {
        self.mazes.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation() {
        let maze = Maze::new(5, 5);
        assert_eq!(maze.width, 5);
        assert_eq!(maze.height, 5);
        assert_eq!(maze.grid.len(), 5);
        assert_eq!(maze.grid[0].len(), 5);
    }

    #[test]
    fn test_maze_generation() {
        let maze = Maze::generate_random(11, 11);
        assert_eq!(maze.grid[maze.start.0][maze.start.1], Cell::Start);
        assert_eq!(maze.grid[maze.goal.0][maze.goal.1], Cell::Goal);

        // Check that there are some paths
        let path_count = maze
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == Cell::Path)
            .count();
        assert!(path_count > 0);
    }

    #[test]
    fn test_maze_solving() {
        let mut maze = Maze::generate_random(11, 11);
        assert!(maze.solve());
        assert!(maze.solution.is_some());

        let solution = maze.solution.unwrap();
        assert_eq!(solution.first(), Some(&maze.start));
        assert_eq!(solution.last(), Some(&maze.goal));
    }

    #[test]
    fn test_maze_to_array() {
        let maze = Maze::new(3, 3);
        let array = maze.to_array();
        assert_eq!(array.len(), 9);
    }

    #[test]
    fn test_cell_conversion() {
        assert_eq!(Cell::from_f32(Cell::Wall.to_f32()), Cell::Wall);
        assert_eq!(Cell::from_f32(Cell::Path.to_f32()), Cell::Path);
        assert_eq!(Cell::from_f32(Cell::Start.to_f32()), Cell::Start);
        assert_eq!(Cell::from_f32(Cell::Goal.to_f32()), Cell::Goal);
    }

    #[test]
    fn test_maze_task_creation() {
        let task = MazeTask::new(5, 11, 11);
        assert_eq!(task.mazes().len(), 5);
        assert_eq!(task.width, 11);
        assert_eq!(task.height, 11);
    }

    #[test]
    fn test_svg_generation() {
        let mut maze = Maze::generate_random(7, 7);
        maze.solve();
        let svg = maze.to_svg(20);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("rect"));
    }
}
