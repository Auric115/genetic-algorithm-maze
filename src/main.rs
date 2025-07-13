mod maze;
mod path;

use path::{Direction, Path};

fn main() {
    println!("--- Maze ---");
    maze::draw_maze(None);

    // Sample path (arbitrary moves)
    let test_path = Path::new(vec![
        Direction::Left,
        Direction::Left,
        Direction::Up,
        Direction::Up,
        Direction::Up,
        Direction::Left,
        Direction::Left,
        Direction::Up,
    ]);

    println!("\n--- Simulating Path ---");
    let result = test_path.simulate(maze::START_X, maze::START_Y);
    println!(
        "End position: ({}, {}), Reached goal: {}, Steps: {}, Final distance: {:.2}",
        result.end_x, result.end_y, result.reached_goal, result.steps_taken, result.final_distance
    );

    let fitness = test_path.fitness(maze::START_X, maze::START_Y);
    println!("Fitness score: {:.4}", fitness);

    // Visualize path end location
    maze::draw_maze(Some((result.end_x, result.end_y)));
}

