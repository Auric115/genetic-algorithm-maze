//genetics.rs

use rand::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Debug)]
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

    fn tournament_selection(&self, k: usize) -> &Genome {
        let mut rng = thread_rng();
        let mut best = &self.population[rng.gen_range(0..self.pop_size)];

        for _ in 1..k {
            let contender = &self.population[rng.gen_range(0..self.pop_size)];
            if contender.fitness > best.fitness {
                best = contender;
            }
        }
        best
    }

    pub fn decode(&self, bits: &[u8]) -> Vec<u8> {
        bits.chunks(2).map(|chunk| {
            match chunk {
                [0, 0] => 0,
                [0, 1] => 1,
                [1, 0] => 2,
                [1, 1] => 3,
                _ => 0,
            }
        }).collect()
    }

    pub fn update_fitness<F>(&mut self, test_route: F)
    where
        F: Fn(Vec<u8>) -> f64 + Send + Sync + Copy,
    {
        let decoded_paths: Vec<Vec<u8>> = self
            .population
            .par_iter()
            .map(|genome| self.decode(&genome.bits))
            .collect();

        let fitness_scores: Vec<f64> = decoded_paths
            .par_iter()
            .map(|decoded| test_route(decoded.clone()))
            .collect();

        self.total_fitness = 0.0;
        self.best_fitness = f64::NEG_INFINITY;
        self.fittest_index = 0;

        for (i, (genome, fitness)) in self.population.iter_mut().zip(fitness_scores).enumerate() {
            genome.fitness = fitness;
            self.total_fitness += fitness;
            if fitness > self.best_fitness {
                self.best_fitness = fitness;
                self.fittest_index = i;
            }
        }
    }

    pub fn epoch<F>(&mut self, test_route: F)
    where
        F: Fn(Vec<u8>) -> f64 + Send + Sync + Copy,
    {
        let mut new_population = Vec::with_capacity(self.pop_size);
        
        let elite = self.population[self.fittest_index].clone();
        new_population.push(elite);

        while new_population.len() + 1 < self.pop_size {
            let mom = self.roulette_selection();
            let dad = self.roulette_selection();
            let (mut baby1_bits, mut baby2_bits) = self.crossover(&mom.bits, &dad.bits);
            self.mutate(&mut baby1_bits);
            self.mutate(&mut baby2_bits);

            new_population.push(Genome {
                bits: baby1_bits,
                fitness: 0.0,
            });

            if new_population.len() < self.pop_size {
                new_population.push(Genome {
                    bits: baby2_bits,
                    fitness: 0.0,
                });
            }
        }

        self.population = new_population;
        self.generation += 1;
        self.update_fitness(test_route);
    }


    pub fn inject_random_individuals(&mut self, count: usize) {
        use rand::Rng;
        for _ in 0..count {
            let random_bits: Vec<u8> = (0..self.chromo_length)
                .map(|_| rand::thread_rng().gen_range(0..4))
                .collect();

            let fitness = 0.0;

            self.population.push(Genome {
                bits: random_bits,
                fitness,
            });
        }

        if self.population.len() > self.pop_size {
            self.population.truncate(self.pop_size);
        }
    }

    pub fn set_mutation_rate(&mut self, rate: f64) {
        self.mutation_rate = rate;
    }

}
