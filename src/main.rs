mod maze;

fn main() {
    println!("--- Maze Visualization ---");
    maze::draw_maze(None);

    println!("\nAgent at START:");
    maze::draw_maze(Some((maze::START_X, maze::START_Y)));

    println!("\nAgent at GOAL:");
    maze::draw_maze(Some((maze::GOAL_X, maze::GOAL_Y)));
}

