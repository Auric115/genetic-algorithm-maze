use rand::Rng;
use std::io;

const MAP_HEIGHT: usize = 10;
const MAP_WIDTH: usize = 15;

fn int_to_str(num: i32) -> String {
    num.to_string()
}

fn draw_screen(arri_map: &[[i32; MAP_WIDTH]; MAP_HEIGHT], map_size: usize, pos_x: usize, pos_y: usize) {
    for i in 0..MAP_HEIGHT {
        for _ in 0..map_size {
            for j in 0..MAP_WIDTH {
                if i == pos_y && j == pos_x {
                    print!("{}", "*".repeat(map_size));
                } else {
                    match arri_map[i][j] {
                        0 => print!("{}", " ".repeat(map_size)),
                        1 => print!("{}", "#".repeat(map_size)),
                        5 => print!("{}", "-".repeat(map_size)),
                        8 => print!("{}", "=".repeat(map_size)),
                        _ => print!("{}", "?".repeat(map_size)),
                    }
                }
            }
            println!();
        }
    }
}

struct MazeMap {
    arri_map: [[i32; MAP_WIDTH]; MAP_HEIGHT],
    start_x: usize,
    start_y: usize,
    exit_x: usize,
    exit_y: usize,
}

impl MazeMap {
    fn new() -> Self {
        MazeMap {
            arri_map: [
                [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
                [1,0,1,0,0,0,0,0,1,1,1,0,0,0,1],
                [8,0,0,0,0,0,0,0,1,1,1,0,0,0,1],
                [1,0,0,0,1,1,1,0,0,1,0,0,0,0,1],
                [1,0,0,0,1,1,1,0,0,0,0,0,1,0,1],
                [1,1,0,0,1,1,1,0,0,0,0,0,1,0,1],
                [1,0,0,0,0,1,0,0,0,0,1,1,1,0,1],
                [1,0,1,1,0,0,0,1,0,0,0,0,0,0,5],
                [1,0,1,1,0,0,0,1,0,0,0,0,0,0,1],
                [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            ],
            start_x: 14,
            start_y: 7,
            exit_x: 0,
            exit_y: 2,
        }
    }

    fn test_route(&self, path: &[i32]) -> f64 {
        let mut pos_x = self.start_x;
        let mut pos_y = self.start_y;

        for &dir in path {
            match dir {
                0 if pos_y > 0 && self.arri_map[pos_y - 1][pos_x] != 1 => pos_y -= 1,
                1 if pos_x + 1 < MAP_WIDTH && self.arri_map[pos_y][pos_x + 1] != 1 => pos_x += 1,
                2 if pos_y + 1 < MAP_HEIGHT && self.arri_map[pos_y + 1][pos_x] != 1 => pos_y += 1,
                3 if pos_x > 0 && self.arri_map[pos_y][pos_x - 1] != 1 => pos_x -= 1,
                _ => {}
            }
        }

        let dx = (pos_x as i32 - self.exit_x as i32).abs();
        let dy = (pos_y as i32 - self.exit_y as i32).abs();
        1.0 / ((dx + dy + 1) as f64)
    }
}

#[derive(Clone)]
struct Genome {
    bits: Vec<i32>,
    fitness: f64,
}

impl Genome {
    fn new_random(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        Genome {
            bits: (0..length).map(|_| rng.gen_range(0..=1)).collect(),
            fitness: 0.0,
        }
    }
}

// -- Decode functions moved out of GenAlgo --

fn bin_to_int(v: &[i32]) -> i32 {
    v.iter().rev().enumerate().map(|(i, &bit)| bit * 2_i32.pow(i as u32)).sum()
}

fn decode(bits: &[i32]) -> Vec<i32> {
    bits.chunks(2).map(|chunk| bin_to_int(chunk)).collect()
}

struct GenAlgo {
    genomes: Vec<Genome>,
    pop_size: usize,
    crossover_rate: f64,
    mutation_rate: f64,
    chromo_length: usize,
    gene_length: usize,
    fittest_index: usize,
    best_score: f64,
    total_score: f64,
    generation: usize,
    maze: MazeMap,
    running: bool,
}

impl GenAlgo {
    fn new(crossover_rate: f64, mutation_rate: f64, pop_size: usize, chromo_length: usize, gene_length: usize) -> Self {
        let mut algo = GenAlgo {
            genomes: Vec::new(),
            pop_size,
            crossover_rate,
            mutation_rate,
            chromo_length,
            gene_length,
            fittest_index: 0,
            best_score: 0.0,
            total_score: 0.0,
            generation: 0,
            maze: MazeMap::new(),
            running: false,
        };
        algo.create_start_population();
        algo
    }

    fn create_start_population(&mut self) {
        for _ in 0..self.pop_size {
            self.genomes.push(Genome::new_random(self.chromo_length));
        }
    }

    fn mutate(&self, bits: &mut Vec<i32>) {
        let mut rng = rand::thread_rng();
        for bit in bits.iter_mut() {
            if rng.r#gen::<f64>() < self.mutation_rate {
                *bit = 1 - *bit;
            }
        }
    }

    fn crossover(&self, mom: &[i32], dad: &[i32]) -> (Vec<i32>, Vec<i32>) {
        let mut rng = rand::thread_rng();
        if rng.r#gen::<f64>() > self.crossover_rate || mom == dad {
            return (mom.to_vec(), dad.to_vec());
        }

        let cp = rng.gen_range(0..self.chromo_length);
        let mut baby1 = Vec::new();
        let mut baby2 = Vec::new();

        baby1.extend_from_slice(&mom[..cp]);
        baby1.extend_from_slice(&dad[cp..]);
        baby2.extend_from_slice(&dad[..cp]);
        baby2.extend_from_slice(&mom[cp..]);

        (baby1, baby2)
    }

    fn roulette_selection(&self) -> &Genome {
        let mut rng = rand::thread_rng();
        let slice = rng.r#gen::<f64>() * self.total_score;
        let mut total = 0.0;
        for genome in &self.genomes {
            total += genome.fitness;
            if total >= slice {
                return genome;
            }
        }
        &self.genomes[0]
    }

    fn update_fitness(&mut self) {
        self.total_score = 0.0;
        self.best_score = 0.0;
        self.fittest_index = 0;

        let maze = &self.maze;

        for (i, genome) in self.genomes.iter_mut().enumerate() {
            let route = decode(&genome.bits); // standalone decode function, no borrow of self
            genome.fitness = maze.test_route(&route);

            self.total_score += genome.fitness;
            if genome.fitness > self.best_score {
                self.best_score = genome.fitness;
                self.fittest_index = i;
            }
        }
    }

    fn epoch(&mut self) {
        self.update_fitness();
        let mut babies = Vec::new();
        while babies.len() < self.pop_size {
            let mom = self.roulette_selection();
            let dad = self.roulette_selection();

            let (mut baby1_bits, mut baby2_bits) = self.crossover(&mom.bits, &dad.bits);
            self.mutate(&mut baby1_bits);
            self.mutate(&mut baby2_bits);
            babies.push(Genome { bits: baby1_bits, fitness: 0.0 });
            babies.push(Genome { bits: baby2_bits, fitness: 0.0 });
        }
        self.genomes = babies;
        self.generation += 1;
    }

    fn run(&mut self) {
        println!("Population initialized...");
        let mut input = String::new();
        loop {
            self.epoch();
            println!("Best Fitness Score: {}", self.best_score);
            println!("Best Genome: {:?}", self.genomes[self.fittest_index].bits);

            if self.generation % 5 == 0 {
                println!("Run next 5 Generations (Y/N)? >");
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let answer = input.trim().to_uppercase();
                if answer != "Y" {
                    break;
                }
            }
        }
        println!("Program Complete. Exit Success");
    }
}

fn main() {
    println!("Program started...");
    let mut ga = GenAlgo::new(0.7, 0.0001, 140, 70, 2);
    ga.run();
}

