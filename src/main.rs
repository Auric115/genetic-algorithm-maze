mod maze;
mod genetics;

use genetics::GenAlgo;
use maze::Maze;

fn main() {
    println!("Program started...");

    let maze = Maze::new(10, 10);

    let mut ga = GenAlgo::new(
        0.7,
        0.0001,
        140,
        70,
        2
    );

    ga.run(|route| maze.test_route(route));
}
