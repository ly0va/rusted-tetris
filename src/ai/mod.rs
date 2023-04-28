use crate::game::StandardGame;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

mod genes;

const SCORE_LIMIT: u32 = 1000;

pub trait Gene {
    fn evaluate(&self, state: &StandardGame) -> f64;
}
// TODO: implement genes

#[derive(Clone, Debug, Default)]
pub struct DNA(pub Vec<f64>);

impl DNA {
    pub fn new_random(size: usize) -> Self {
        let rng = &mut rand::thread_rng();
        DNA(vec![rng.gen_range(-1.0, 1.0); size]).normalize()
    }

    pub fn normalize(mut self) -> Self {
        let sum: f64 = self.0.iter().sum();
        self.0.iter_mut().for_each(|x| *x /= sum);
        self
    }

    pub fn crossover(&self, other: &Self) -> Self {
        debug_assert!(self.0.len() == other.0.len());
        let mut child = Vec::with_capacity(self.0.len());
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            child.push(*a + *b);
        }
        DNA(child).normalize()
    }

    pub fn mutate(mut self, rate: f64) -> Self {
        let rng = &mut rand::thread_rng();
        for x in self.0.iter_mut() {
            if rng.gen::<f64>() < rate {
                *x += rng.gen_range(-0.1, 0.1);
            }
        }
        self.normalize()
    }
}

pub struct Population {
    dna: Vec<DNA>,
    genes: Vec<Box<dyn Gene>>,
}

impl Population {
    pub fn new_random(size: usize, genes: Vec<Box<dyn Gene>>) -> Self {
        let mut dna = Vec::with_capacity(size);
        for _ in 0..size {
            dna.push(DNA::new_random(genes.len()));
        }
        Population { dna, genes }
    }

    pub fn instinct(&self, index: usize, state: &StandardGame) -> f64 {
        self.genes
            .iter()
            .zip(self.dna[index].0.iter())
            .map(|(gene, weight)| gene.evaluate(state) * weight)
            .sum()
    }

    pub fn simulate(&self, index: usize) -> f64 {
        let mut game = StandardGame::new();
        while !game.over {
            // TODO: evaluaate all possible shifts/rotations
            // and choose the best one based on the evaluation
            // TODO: fix seed for each game
            game.tick();
            if game.score >= SCORE_LIMIT {
                break;
            }
        }
        game.score as f64
    }

    pub fn rank_generation(&self) -> Vec<f64> {
        self.dna
            .iter()
            .enumerate()
            .map(|(index, _)| self.simulate(index))
            .collect()
    }

    pub fn next_generation(&mut self) {
        let rng = &mut rand::thread_rng();
        let mut new_dna = Vec::with_capacity(self.dna.len());
        let rank = self.rank_generation();
        let dist = WeightedIndex::new(rank).expect("This generation is shit");
        for _ in 0..self.dna.len() {
            let a = dist.sample(rng);
            let b = dist.sample(rng);
            new_dna.push(self.dna[a].crossover(&self.dna[b]));
        }
        self.dna = new_dna;
    }

    pub fn evolve(&mut self, generations: usize) {
        for _ in 0..generations {
            self.next_generation();
        }
    }
    // TODO: add a lot of logs
}
