// main.rs

mod maze;
mod visualizer;
mod runner;

use macroquad::prelude::*;
use runner::Runner;
use visualizer::Visualizer;

#[macroquad::main("Maze Runner")]
async fn main() {
    let maze = maze::Maze::new(20, 20);
    let mut visualizer = Visualizer::new(maze.grid().clone());
    let start_pos = maze.start_pos().expect("No start position found");

    let mut runner = Runner::new(start_pos);

    loop {
        clear_background(BLACK);
        visualizer.draw_static_maze();

        // Handle movement input
        if is_key_pressed(KeyCode::Up) {
            runner.try_move(0, -1, &maze);
        }
        if is_key_pressed(KeyCode::Down) {
            runner.try_move(0, 1, &maze);
        }
        if is_key_pressed(KeyCode::Left) {
            runner.try_move(-1, 0, &maze);
        }
        if is_key_pressed(KeyCode::Right) {
            runner.try_move(1, 0, &maze);
        }

        visualizer.draw_runner(runner.pos);
        next_frame().await;
    }
}
