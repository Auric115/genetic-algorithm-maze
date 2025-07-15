mod maze;
mod visualizer;
mod runner;

use macroquad::prelude::*;
use maze::Maze;
use visualizer::Visualizer;
use runner::Runner;

#[macroquad::main("Free Maze Runner")]
async fn main() {
    let cell_size = 20.0;
    let maze = Maze::new(20, 20, cell_size);
    let visualizer = Visualizer::new(maze.grid.clone(), cell_size);

    let mut runner = Runner::new(maze.start_position().unwrap(), 5.0, 100.0);

    loop {
        let dt = get_frame_time();
        runner.update(dt, &maze);

        clear_background(BLACK);
        visualizer.draw_maze();
        visualizer.draw_runner(runner.position, runner.radius);
        next_frame().await;
    }
}
