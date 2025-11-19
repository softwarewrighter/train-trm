//! Maze visualization demonstration
//!
//! This example generates a maze, solves it, and creates visual representations.
//!
//! Run with: cargo run --example maze_demo

use std::fs;
use train_trm::data::maze::Maze;

fn main() {
    println!("=== TRM Maze Solving Demonstration ===\n");

    // Generate different sized mazes
    let sizes = [(11, 11), (15, 15), (21, 21)];

    for (idx, (width, height)) in sizes.iter().enumerate() {
        println!("Generating maze {}x{}...", width, height);
        let mut maze = Maze::generate_random(*width, *height);

        println!("Solving maze...");
        if maze.solve() {
            let solution_len = maze.solution.as_ref().unwrap().len();
            println!("Solution found! Path length: {}\n", solution_len);

            println!("Maze as matrix (S=start, G=goal, ·=solution path):");
            println!("{}", "=".repeat(*width));
            maze.print();
            println!("{}\n", "=".repeat(*width));

            // Generate SVG
            let svg = maze.to_svg(30);
            let filename = format!("maze_{}x{}_{}.svg", width, height, idx);

            match fs::write(&filename, &svg) {
                Ok(_) => println!("✓ Saved SVG to {}", filename),
                Err(e) => eprintln!("✗ Error saving SVG: {}", e),
            }

            // Show numerical representation
            println!("Numerical representation:");
            print_numerical_matrix(&maze);

            println!("\n{}\n", "=".repeat(60));
        } else {
            println!("No solution found!\n");
        }
    }

    println!("=== Demonstration Complete ===");
    println!("\nOpen the generated .svg files in a web browser to see the");
    println!("visual representation of the mazes with solution paths!");
    println!("\nColor legend:");
    println!("  - Dark gray: Walls");
    println!("  - Light gray: Open paths");
    println!("  - Green: Start position");
    println!("  - Red: Goal position");
    println!("  - Blue line: Solution path");
}

fn print_numerical_matrix(maze: &Maze) {
    for row in &maze.grid {
        for &cell in row {
            let val = cell.to_f32();
            print!("{:.2} ", val);
        }
        println!();
    }
}
