mod maze;
mod genetics;
mod visualizer;

use macroquad::prelude::*;
use maze::Maze;
use visualizer::Visualizer;
use genetics::GeneticAlgorithm;

#[macroquad::main("Genetic Maze Solver")]
async fn main() {
    let maze = Maze::new(10, 10);
    let visualizer = Visualizer::new(maze.get_grid());

    let mut ga = GeneticAlgorithm::new(
        0.7,
        0.01,
        50,
        70,
        2,
    );

    ga.update_fitness(|route| 0.0); // dry run

    loop {
        ga.epoch(|genome| {
            let decoded = genome.clone();
            futures::executor::block_on(maze.test_route_visual(decoded, &visualizer))
        });

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}
