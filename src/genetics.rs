//genetics.rs

use rand::prelude::*;
use rayon::prelude::*;

fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
}

#[derive(Clone, Debug)]
pub struct Genome {
    pub bits: Vec<u8>,
    pub fitness: f64,
    pub stagnation: usize,
}

impl Genome {
    pub fn new_random(num_bits: usize) -> Self {
        let mut rng = thread_rng();
        let bits: Vec<u8> = (0..num_bits).map(|_| rng.gen_range(0..=1)).collect();
        Self {
            bits,
            fitness: 0.0,
            stagnation: 0,
        }
    }
}

pub struct GeneticAlgorithm {
    pub population: Vec<Genome>,
    pub pop_size: usize,
    pub elitism: f64,
    pub stagnation_limit: usize,
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
    pub fn new(crossover_rate: f64, mutation_rate: f64, pop_size: usize, elitism: f64, stagnation_limit: usize, chromo_length: usize, gene_length: usize) -> Self {
        let mut algo = Self {
            population: Vec::with_capacity(pop_size),
            pop_size,
            elitism,
            stagnation_limit,
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
        let mut decoded = Vec::with_capacity(bits.len() / 2);
        for chunk in bits.chunks(2) {
            if chunk.len() != 2 {
                // Log warning or skip last bit if chunk is incomplete
                continue;
            }
            let dir = match chunk {
                [0, 0] => 0,
                [0, 1] => 1,
                [1, 0] => 2,
                [1, 1] => 3,
                _ => 0, // default fallback
            };
            decoded.push(dir);
        }
        decoded
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
            if fitness > genome.fitness {
                genome.stagnation = 0;
            } else {
                genome.stagnation += 1;
            }

            genome.fitness = fitness;

            self.total_fitness += genome.fitness;

            if genome.fitness > self.best_fitness {
                self.best_fitness = genome.fitness;
                self.fittest_index = i;
            }
        }
    }

    pub fn epoch<F>(&mut self, test_route: F)
    where
        F: Fn(Vec<u8>) -> f64 + Send + Sync + Copy,
    {
        self.population.retain(|g| g.stagnation < self.stagnation_limit);
        let culled_count = self.pop_size - self.population.len();
        self.inject_random_individuals(culled_count);

        let mut new_population = Vec::with_capacity(self.pop_size);
        
        let mut sorted = self.population.clone();
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        for elite in sorted.iter().take((self.elitism * self.pop_size as f64).floor() as usize) {
            new_population.push(elite.clone());
        }

        while new_population.len() + 1 < self.pop_size {
            let mom = self.tournament_selection(3);
            let dad = self.tournament_selection(3);
            let (mut baby1_bits, mut baby2_bits) = self.crossover(&mom.bits, &dad.bits);
            self.mutate(&mut baby1_bits);
            self.mutate(&mut baby2_bits);

             let avg_stagnation = ((mom.stagnation + dad.stagnation) / 2).max(0);

            new_population.push(Genome {
                bits: baby1_bits,
                fitness: 0.0,
                stagnation: avg_stagnation,
            });

            if new_population.len() < self.pop_size {
                new_population.push(Genome {
                    bits: baby2_bits,
                    fitness: 0.0,
                    stagnation: avg_stagnation,
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
                .map(|_| rand::thread_rng().gen_range(0..=1))
                .collect();

            let fitness = 0.0;
            let stagnation = 0;

            self.population.push(Genome {
                bits: random_bits,
                fitness,
                stagnation,
            });
        }

        if self.population.len() > self.pop_size {
            self.population.truncate(self.pop_size);
        }
    }

    pub fn reset(&mut self) {
        self.fittest_index = 0;
        self.best_fitness = 0.0;
        self.total_fitness = 0.0;
        self.generation = 0;
        self.create_start_population();
    }

    pub fn set_mutation_rate(&mut self, rate: f64) {
        self.mutation_rate = rate;
    }

    pub fn average_hamming_distance(&self, top_n: usize) -> f64 {
        let sorted = {
            let mut pop = self.population.clone();
            pop.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            pop.into_iter().take(top_n).collect::<Vec<_>>()
        };

        if sorted.len() < 2 {
            return 0.0;
        }

        let mut total_distance = 0usize;
        let mut count = 0usize;

        for i in 0..sorted.len() {
            for j in (i + 1)..sorted.len() {
                let dist = hamming_distance(&sorted[i].bits, &sorted[j].bits);
                total_distance += dist;
                count += 1;
            }
        }

        total_distance as f64 / count as f64
    }

    pub fn adapt_mutation_rate(&mut self, min_rate: f64, max_rate: f64, target_diversity: f64) {
        let diversity = self.average_hamming_distance((self.elitism * self.pop_size as f64).ceil() as usize);

        if diversity < target_diversity {
            self.mutation_rate = (self.mutation_rate * 1.1).min(max_rate);
        } else {
            self.mutation_rate = (self.mutation_rate * 0.9).max(min_rate);
        }
    }

}
