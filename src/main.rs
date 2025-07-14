//main.rs

mod maze;
mod genetics;
mod visualizer;

use macroquad::prelude::*;
use maze::Maze;
use visualizer::Visualizer;
use genetics::GeneticAlgorithm;
use macroquad::window::Conf;

const MAZE_WIDTH: usize = 15;
const MAZE_HEIGHT: usize = 15;
const CELL_SIZE: f32 = 20.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Genetic Maze Solver".to_string(),
        window_width: ((MAZE_WIDTH * 2 + 1) as f32 * CELL_SIZE) as i32,
        window_height: ((MAZE_HEIGHT * 2 + 1) as f32 * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT);
    let visualizer = Visualizer::new(maze.get_grid());

    let mut ga = GeneticAlgorithm::new(
        0.7,
        0.15,
        600,
        0.03,
        500,
        2048,
        2,
    );

    ga.update_fitness(|route| maze.test_route(route));

    let mut goal_reached = false;
    let mut final_path = Vec::new();
    let mut generation = 0;

    println!("{:>4} | {:>12} | {:<36}", "Gen.", "Fitness", "Genome");
    println!("{:-<4}-+-{:-<12}-+-{:-<36}", "", "", "");

    loop {
        if !goal_reached {
            ga.epoch(|genome| {
                let decoded = genome.clone();
                maze.test_route(decoded)
            });

            let (best_route, best_bits, fitness) = {
                if ga.population.is_empty() {
                    continue;
                }

                let best = &ga.population[ga.fittest_index];
                (ga.decode(&best.bits), best.bits.clone(), best.fitness)
            };

            //let adaptive_mutation = 0.15 * (1.0 - (generation as f32 / 100.0).clamp(0.05, 1.0)) as f64;
            //ga.set_mutation_rate(adaptive_mutation);

            let min_mutation = 0.05;
            let max_mutation = 0.3;
            let target_diversity = (ga.chromo_length as f64) * 0.1; // e.g., 10% bits differ on average among elites

            ga.adapt_mutation_rate(min_mutation, max_mutation, target_diversity);

            if generation % 10 == 0 {
                ga.inject_random_individuals(5);
            }

            let hex_str = |bits: &[u8]| -> String {
                let byte_chunks = bits.chunks(8).collect::<Vec<_>>();

                let start_hex: String = byte_chunks.iter().take(8).map(|chunk| {
                    let byte = chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b);
                    format!("{byte:02X}")
                }).collect();

                let end_hex: String = byte_chunks.iter().rev().take(8).collect::<Vec<_>>().iter().rev().map(|chunk| {
                    let byte = chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b);
                    format!("{byte:02X}")
                }).collect();

                format!("{start_hex}...{end_hex}")
            };

            println!(
                "{:>4} | {:>12.2} | {:<36}",
                generation,
                fitness,
                hex_str(&best_bits)
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
