use crate::game::{Game, StandardGame, WIDTH};
use crate::tetromino::Direction;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;

pub mod genes;

const SCORE_LIMIT: u32 = 1000;

pub trait Gene {
    fn evaluate(&self, state: &StandardGame) -> f64;
}

#[derive(Clone, Debug, Default)]
pub struct DNA(pub Vec<f64>);

impl DNA {
    pub fn new_random(size: usize) -> Self {
        let rng = &mut rand::thread_rng();
        DNA((0..size).map(|_| rng.gen_range(-1., 1.)).collect()).normalize()
    }

    pub fn normalize(mut self) -> Self {
        let norm = self.0.iter().map(|x| x * x).sum::<f64>().sqrt();
        self.0.iter_mut().for_each(|x| *x /= norm);
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
            if rng.gen_range(0., 1.) < rate {
                *x += rng.gen_range(-0.1, 0.1);
            }
        }
        self.normalize()
    }
}

pub struct Population {
    dna: Vec<DNA>,
    genes: Vec<Box<dyn Gene + Sync>>,
}

impl Population {
    pub fn single(dna: DNA, genes: Vec<Box<dyn Gene + Sync>>) -> Self {
        assert_eq!(genes.len(), dna.0.len());
        Self {
            dna: vec![dna],
            genes,
        }
    }

    pub fn new_random(size: usize, genes: Vec<Box<dyn Gene + Sync>>) -> Self {
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

    pub fn best_actions(&self, index: usize, game: &StandardGame) -> (usize, usize) {
        let states = game.all_possible_states();
        let (_, &shifts, &rotations) = states
            .iter()
            .map(|(state, shifts, rotations)| (self.instinct(index, state), shifts, rotations))
            .max_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap())
            .unwrap();
        (shifts, rotations)
    }

    pub fn simulate(&self, index: usize) -> u32 {
        // TODO: fix seed for each game
        let mut game = StandardGame::new();
        while !game.over {
            let states = game.all_possible_states();
            let (instinct, &shifts, &rotations) = states
                .iter()
                .map(|(state, shifts, rotations)| (self.instinct(index, state), shifts, rotations))
                .max_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap())
                .unwrap();
            for _ in 0..WIDTH {
                game.shift(Direction::Left);
            }
            for _ in 0..shifts {
                game.shift(Direction::Right);
            }
            for _ in 0..rotations {
                game.turn();
            }
            game.hard_drop();
            assert_eq!(instinct, self.instinct(index, &game));
            game.tick();
            if game.score >= SCORE_LIMIT {
                break;
            }
        }
        log::debug!("Score: {}", game.score);
        game.score
    }

    pub fn rank_generation(&self) -> Vec<u32> {
        self.dna
            .par_iter()
            .enumerate()
            .map(|(index, _)| self.simulate(index))
            .collect()
    }

    pub fn next_generation(&mut self) {
        let rng = &mut rand::thread_rng();
        let mut new_dna = Vec::with_capacity(self.dna.len());
        let rank = self.rank_generation();
        log::info!(
            "Champion (score {}): {:?}",
            *rank.iter().max().unwrap(),
            self.champion(&rank)
        );
        let dist = WeightedIndex::new(rank).expect("This generation is shit");
        for _ in 0..self.dna.len() {
            let a = dist.sample(rng);
            let b = dist.sample(rng);
            // TODO: vanishing rate
            new_dna.push(self.dna[a].crossover(&self.dna[b]).mutate(0.1));
        }
        self.dna = new_dna;
    }

    pub fn evolve(&mut self, generations: usize) {
        for gen in 0..generations {
            log::info!("Growing generation #{gen}");
            self.next_generation();
        }
        log::info!("Evolution complete");
    }

    pub fn champion(&self, rank: &[u32]) -> DNA {
        self.dna
            .iter()
            .enumerate()
            .max_by_key(|(index, _)| rank[*index])
            .unwrap()
            .1
            .clone()
    }
    // TODO: add a lot of logs
}

impl<const WIDTH: usize, const HEIGHT: usize> Game<WIDTH, HEIGHT> {
    fn all_possible_states(&self) -> Vec<(Self, usize, usize)> {
        // we assume we have the state where the new tetromino has just spawned
        // we need to check all possible shifts to the right combined with
        // all possible rotations
        let mut states = vec![];
        // always do shifts first, then rotations! FIXME still check if it rotates correctly
        for shifts in 0..WIDTH {
            let mut game = self.clone();
            // first, shift it all the way to the left.
            for _ in 0..WIDTH {
                game.shift(Direction::Left);
            }
            for _ in 0..shifts {
                game.shift(Direction::Right);
            }
            for rotations in 0..4 {
                let mut game = game.clone();
                for _ in 0..rotations {
                    game.turn();
                }
                game.hard_drop();
                // game.tick(); // Ticking is done inside Gene.evaluate
                states.push((game, shifts, rotations));
            }
        }
        states
    }
}
