// main.rs

mod maze;
mod genetics;
mod visualizer;

use macroquad::prelude::*;
use std::time::Duration;
use std::thread::sleep;
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
        1024,
        2,
    );

    ga.update_fitness(|route| maze.test_route(route));

    loop {
        ga.epoch(|genome| {
            let decoded = genome.clone();
            maze.test_route(decoded)
        });

        let best_genome = &ga.population[ga.fittest_index];
        let best_route = ga.decode(&best_genome.bits);
        let mut pos = maze.start_pos().unwrap();
        let mut path = vec![pos];

        for dir in best_route {
            let (dx, dy) = match dir {
                0 => (0, -1),
                1 => (1, 0),
                2 => (0, 1),
                3 => (-1, 0),
                _ => (0, 0),
            };
            let new_x = (pos.0 as isize + dx).clamp(0, maze.grid()[0].len() as isize - 1) as usize;
            let new_y = (pos.1 as isize + dy).clamp(0, maze.grid().len() as isize - 1) as usize;

            if maze.grid()[new_y][new_x] != '#' {
                pos = (new_x, new_y);
                path.push(pos);
                if Some(pos) == maze.end_pos() {
                    break;
                }
            }
        }

        visualizer.animate(&path).await;

        sleep(Duration::from_millis(1000));

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }

}

