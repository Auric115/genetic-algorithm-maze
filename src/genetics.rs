use rand::prelude::*;
use std::fs::File;
use std::io::{self, Write};

#[derive(Clone)]
pub struct Genome {
    pub bits: Vec<u8>,
    pub fitness: f64,
}

impl Genome {
    pub fn new_random(num_bits: usize) -> Self {
        let mut rng = thread_rng();
        let bits: Vec<u8> = (0..num_bits).map(|_| rng.gen_range(0..=1)).collect();
        Self {
            bits,
            fitness: 0.0,
        }
    }
}

pub struct GeneticAlgorithm {
    pub population: Vec<Genome>,
    pub pop_size: usize,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub chromo_length: usize,
    pub gene_length: usize,

    pub fittest_index: usize,
    pub best_fitness: f64,
    pub total_fitness: f64,
    pub generation: usize,

    pub running: bool,
}

impl GeneticAlgorithm {
    pub fn new(crossover_rate: f64, mutation_rate: f64, pop_size: usize, chromo_length: usize, gene_length: usize) -> Self {
        let mut algo = Self {
            population: Vec::with_capacity(pop_size),
            pop_size,
            crossover_rate,
            mutation_rate,
            chromo_length,
            gene_length,
            fittest_index: 0,
            best_fitness: 0.0,
            total_fitness: 0.0,
            generation: 0,
            running: false,
        };
        algo.create_start_population();
        algo
    }

    fn create_start_population(&mut self) {
        self.population = (0..self.pop_size)
            .map(|_| Genome::new_random(self.chromo_length))
            .collect();
    }

    fn mutate(&self, bits: &mut [u8]) {
        let mut rng = thread_rng();
        for bit in bits.iter_mut() {
            if rng.r#gen::<f64>() < self.mutation_rate {
                *bit ^= 1;
            }
        }
    }

    fn crossover(&self, mom: &[u8], dad: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut rng = thread_rng();
        if rng.r#gen::<f64>() > self.crossover_rate || mom == dad {
            return (mom.to_vec(), dad.to_vec());
        }

        let cp = rng.gen_range(0..self.chromo_length);
        let mut baby1 = mom[..cp].to_vec();
        baby1.extend_from_slice(&dad[cp..]);
        let mut baby2 = dad[..cp].to_vec();
        baby2.extend_from_slice(&mom[cp..]);
        (baby1, baby2)
    }

    fn roulette_selection(&self) -> &Genome {
        let mut rng = thread_rng();
        let slice = rng.r#gen::<f64>() * self.total_fitness;
        let mut total = 0.0;
        for genome in &self.population {
            total += genome.fitness;
            if total > slice {
                return genome;
            }
        }
        &self.population[0]
    }

    fn bin_to_int(bits: &[u8]) -> u8 {
        bits.iter().fold(0, |acc, &b| (acc << 1) | b)
    }

    pub fn decode(&self, bits: &[u8]) -> Vec<u8> {
        bits.chunks(2).map(Self::bin_to_int).collect()
    }

    pub fn update_fitness<F>(&mut self, test_route: F)
    where
        F: Fn(Vec<u8>) -> f64,
    {
        let decoded_paths: Vec<Vec<u8>> = self
            .population
            .iter()
            .map(|genome| self.decode(&genome.bits))
            .collect();

        let mut best = 0.0;
        let mut index = 0;
        let mut total = 0.0;

        for (i, (genome, decoded)) in self.population.iter_mut().zip(decoded_paths).enumerate() {
            genome.fitness = test_route(decoded);
            total += genome.fitness;
            if genome.fitness > best {
                best = genome.fitness;
                index = i;
            }
        }

        self.best_fitness = best;
        self.fittest_index = index;
        self.total_fitness = total;

        self.save_generation_data();
    }


    pub fn epoch<F>(&mut self, test_route: F)
    where
        F: Fn(Vec<u8>) -> f64 + Copy,
    {
        self.update_fitness(test_route);

        let mut new_population = Vec::with_capacity(self.pop_size);

        while new_population.len() < self.pop_size {
            let mom = self.roulette_selection();
            let dad = self.roulette_selection();
            let (mut baby1_bits, mut baby2_bits) = self.crossover(&mom.bits, &dad.bits);
            self.mutate(&mut baby1_bits);
            self.mutate(&mut baby2_bits);
            new_population.push(Genome {
                bits: baby1_bits,
                fitness: 0.0,
            });
            new_population.push(Genome {
                bits: baby2_bits,
                fitness: 0.0,
            });
        }

        self.population = new_population;
        self.generation += 1;
    }

    pub fn save_generation_data(&self) {
        let file_name = format!("data/Generation_{}_file.txt", self.generation);
        if let Ok(mut file) = File::create(&file_name) {
            let _ = writeln!(
                file,
                "{} {} {}",
                self.generation, self.fittest_index, self.best_fitness
            );
            for genome in &self.population {
                let directions: Vec<u8> = self.decode(&genome.bits);
                let _ = write!(file, " {} ", genome.fitness);
                let _ = writeln!(file, "{}", directions.iter().map(|d| d.to_string()).collect::<String>());
            }
        }
    }

    pub fn run<F>(&mut self, test_route: F)
    where
        F: Fn(Vec<u8>) -> f64 + Copy,
    {
        self.running = true;
        println!("Population initialized...");
        let mut continue_running = true;

        while continue_running {
            self.epoch(test_route);
            println!("Best Fitness Score: {}", self.best_fitness);
            println!(
                "Best Genome: {}",
                self.population[self.fittest_index]
                    .bits
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<String>()
            );

            if self.generation % 5 == 0 {
                println!("Run next 5 Generations (Y/N)? >");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                continue_running = input.trim().eq_ignore_ascii_case("Y");
            }
        }

        println!("Program Complete. Exit Success");
    }
}
