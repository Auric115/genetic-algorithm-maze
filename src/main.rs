use rand::prelude::*;

// Constants
const MAZE_WIDTH: usize = 10;
const MAZE_HEIGHT: usize = 10;
const POPULATION_SIZE: usize = 100;
const GENE_LENGTH: usize = 100;
const MAX_GENERATIONS: usize = 1000;
const MUTATION_RATE: f64 = 0.01;

// Directions
#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn random(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

type Position = (usize, usize);

#[derive(Clone)]
struct Chromosome {
    genes: Vec<Direction>,
    fitness: i32,
    end_position: Position,
}

impl Chromosome {
    fn new_random(rng: &mut ThreadRng) -> Self {
        let genes = (0..GENE_LENGTH).map(|_| Direction::random(rng)).collect();
        Chromosome {
            genes,
            fitness: 0,
            end_position: (0, 0),
        }
    }

    fn evaluate_fitness(&mut self, maze: &Vec<Vec<bool>>, start: Position, goal: Position) {
        let mut pos = start;

        for &dir in &self.genes {
            let new_pos = match dir {
                Direction::Up if pos.1 > 0 => (pos.0, pos.1 - 1),
                Direction::Down if pos.1 < MAZE_HEIGHT - 1 => (pos.0, pos.1 + 1),
                Direction::Left if pos.0 > 0 => (pos.0 - 1, pos.1),
                Direction::Right if pos.0 < MAZE_WIDTH - 1 => (pos.0 + 1, pos.1),
                _ => pos,
            };

            if maze[new_pos.1][new_pos.0] {
                pos = new_pos;
            }
        }

        self.end_position = pos;
        self.fitness = -((goal.0 as i32 - pos.0 as i32).abs() + (goal.1 as i32 - pos.1 as i32).abs());
    }
}

fn evolve(maze: Vec<Vec<bool>>, start: Position, goal: Position) {
    let mut rng = rand::thread_rng();
    let mut population: Vec<Chromosome> = (0..POPULATION_SIZE)
        .map(|_| Chromosome::new_random(&mut rng))
        .collect();

    for generation in 0..MAX_GENERATIONS {
        for indiv in &mut population {
            indiv.evaluate_fitness(&maze, start, goal);
        }

        population.sort_by(|a, b| b.fitness.cmp(&a.fitness));

        println!(
            "Gen {:3}: Best fitness = {:3} | Distance to goal = {} | Pos: {:?}",
            generation,
            population[0].fitness,
            -population[0].fitness,
            population[0].end_position
        );

        if population[0].fitness == 0 {
            println!("Goal reached in generation {}!", generation);
            print_path(&maze, &population[0], start, goal);
            break;
        }

        let mut new_population = vec![population[0].clone()]; // Elitism

        while new_population.len() < POPULATION_SIZE {
            let parent1 = tournament_selection(&population, &mut rng);
            let parent2 = tournament_selection(&population, &mut rng);
            let mut child = crossover(parent1, parent2, &mut rng);
            mutate(&mut child, &mut rng);
            new_population.push(child);
        }

        population = new_population;
    }
}

fn tournament_selection<'a>(pop: &'a [Chromosome], rng: &mut ThreadRng) -> &'a Chromosome {
    let a = rng.gen_range(0..pop.len());
    let b = rng.gen_range(0..pop.len());
    if pop[a].fitness > pop[b].fitness {
        &pop[a]
    } else {
        &pop[b]
    }
}

fn crossover(parent1: &Chromosome, parent2: &Chromosome, rng: &mut ThreadRng) -> Chromosome {
    let crossover_point = rng.gen_range(0..GENE_LENGTH);
    let genes = parent1.genes[..crossover_point]
        .iter()
        .chain(parent2.genes[crossover_point..].iter())
        .cloned()
        .collect();
    Chromosome {
        genes,
        fitness: 0,
        end_position: (0, 0),
    }
}

fn mutate(chrom: &mut Chromosome, rng: &mut ThreadRng) {
    for gene in &mut chrom.genes {
        if rng.gen_bool(MUTATION_RATE) {
            *gene = Direction::random(rng);
        }
    }
}

fn generate_maze() -> Vec<Vec<bool>> {
    let mut maze = vec![vec![true; MAZE_WIDTH]; MAZE_HEIGHT];

    // Add some walls
    for y in 2..8 {
        maze[y][5] = false;
    }
    for x in 2..5 {
        maze[4][x] = false;
    }

    maze
}

fn print_maze(maze: &Vec<Vec<bool>>, path: &[Position], start: Position, goal: Position) {
    for y in 0..MAZE_HEIGHT {
        for x in 0..MAZE_WIDTH {
            let pos = (x, y);
            if pos == start {
                print!("S ");
            } else if pos == goal {
                print!("G ");
            } else if path.contains(&pos) {
                print!("* ");
            } else if maze[y][x] {
                print!(". ");
            } else {
                print!("# ");
            }
        }
        println!();
    }
}

fn print_path(maze: &Vec<Vec<bool>>, chrom: &Chromosome, start: Position, goal: Position) {
    let mut pos = start;
    let mut path = vec![pos];

    for &dir in &chrom.genes {
        let new_pos = match dir {
            Direction::Up if pos.1 > 0 => (pos.0, pos.1 - 1),
            Direction::Down if pos.1 < MAZE_HEIGHT - 1 => (pos.0, pos.1 + 1),
            Direction::Left if pos.0 > 0 => (pos.0 - 1, pos.1),
            Direction::Right if pos.0 < MAZE_WIDTH - 1 => (pos.0 + 1, pos.1),
            _ => pos,
        };

        if maze[new_pos.1][new_pos.0] {
            pos = new_pos;
            path.push(pos);
        }
    }

    println!("\nFinal path:");
    print_maze(maze, &path, start, goal);
}

fn main() {
    let maze = generate_maze();
    let start = (0, 0);
    let goal = (9, 9);

    println!("Initial Maze:");
    print_maze(&maze, &[], start, goal);
    println!("\nEvolving...\n");

    evolve(maze, start, goal);
}

