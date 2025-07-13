//main.rs

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
        0.15,
        150,
        1024,
        2,
    );

    ga.update_fitness(|route| maze.test_route(route));

    let mut goal_reached = false;
    let mut final_path = Vec::new();
    let mut generation = 0;

    loop {
        if !goal_reached {
            ga.epoch(|genome| {
                let decoded = genome.clone();
                maze.test_route(decoded)
            });

            let (best_route, best_bits, fitness) = {
                let best = &ga.population[ga.fittest_index];
                (ga.decode(&best.bits), best.bits.clone(), best.fitness)
            };

            let adaptive_mutation = 0.15 * (1.0 - (generation as f32 / 100.0)).clamp(0.05, 1.0) as f64;
            ga.set_mutation_rate(adaptive_mutation);

            if generation % 10 == 0 {
                ga.inject_random_individuals(5);
            }

            let genome_hex: String = best_bits
                .chunks(8)
                .take(8)
                .map(|chunk| {
                    let byte = chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b as u8);
                    format!("{:02X}", byte)
                })
                .collect();

            println!(
                "Generation {:>4} | Best Fitness: {:>6.2} | Best Genome: {}",
                generation, fitness, genome_hex
            );

            generation += 1;

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
                        goal_reached = true;
                        final_path = path.clone();
                        break;
                    }
                }
            }

            visualizer.animate(&path).await;
        } else {
            visualizer.animate(&final_path).await;
        }

        next_frame().await;
    }
}
