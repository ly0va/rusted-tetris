use crate::game::{Game, StandardGame, WIDTH};
use crate::tetromino::Direction;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;

pub mod genes;

const SCORE_LIMIT: u32 = 1000;
const MOVE_LIMIT: u32 = 1000;

pub trait Gene {
    fn evaluate(&self, state: &StandardGame) -> f64;
}

#[derive(Clone, Debug, Default)]
pub struct DNA(pub Vec<f64>);

impl DNA {
    pub fn new_random(size: usize, rng: &mut impl Rng) -> Self {
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

    pub fn mutate(mut self, rate: f64, rng: &mut impl Rng) -> Self {
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
    rng: SmallRng,
}

impl Population {
    pub fn single(dna: DNA, genes: Vec<Box<dyn Gene + Sync>>) -> Self {
        assert_eq!(genes.len(), dna.0.len());
        Self {
            dna: vec![dna],
            genes,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn new(size: usize, genes: Vec<Box<dyn Gene + Sync>>, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut dna = Vec::with_capacity(size);
        for _ in 0..size {
            dna.push(DNA::new_random(genes.len(), &mut rng));
        }
        Population { dna, genes, rng }
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

    pub fn simulate(&self, index: usize, seed: u64) -> u32 {
        let mut game = StandardGame::new_with_seed(seed);
        let mut moves = 0;
        while !game.over {
            let (shifts, rotations) = self.best_actions(index, &game);
            for _ in 0..WIDTH {
                game.shift(Direction::Left);
            }
            for _ in 0..shifts {
                game.shift(Direction::Right);
            }
            for _ in 0..rotations {
                game.rotate();
            }
            game.hard_drop();
            game.tick();
            if game.score >= SCORE_LIMIT {
                break;
            }
            moves += 1;
            if moves >= MOVE_LIMIT {
                break;
            }
        }
        log::debug!("Score: {}", game.score);
        game.score
    }

    pub fn rank_generation(&self, seed: u64) -> Vec<u32> {
        self.dna
            .par_iter()
            .enumerate()
            .map(|(index, _)| self.simulate(index, seed))
            .collect()
    }

    pub fn next_generation(&mut self) {
        let mut new_dna = Vec::with_capacity(self.dna.len());
        // Do X rankings for each generation
        let mut global_rank = vec![0; self.dna.len()];
        const NUMBER_OF_SIMULATIONS: usize = 10;
        for _ in 0..NUMBER_OF_SIMULATIONS {
            // Seed is fixed for each generation
            let seed = self.rng.gen();
            let rank = self.rank_generation(seed);
            // log::info!(
            //     "Champion (score {}): {:?}",
            //     *rank.iter().max().unwrap(),
            //     self.champion(&rank)
            // );
            global_rank
                .iter_mut()
                .zip(rank.iter())
                .for_each(|(a, b)| *a += b);
        }
        log::info!(
            "Champion (score {}): {:?}",
            *global_rank.iter().max().unwrap() / NUMBER_OF_SIMULATIONS as u32,
            self.champion(&global_rank)
        );

        let dist = WeightedIndex::new(global_rank /* .into_iter().map(|x| x * x) */)
            .expect("This generation is shit");
        for _ in 0..self.dna.len() {
            let a = dist.sample(&mut self.rng);
            let b = dist.sample(&mut self.rng);
            // TODO: vanishing rate
            new_dna.push(
                self.dna[a]
                    .crossover(&self.dna[b])
                    .mutate(0.2, &mut self.rng),
            );
        }
        self.dna = new_dna;
    }

    pub fn evolve(&mut self, generations: usize) {
        for gen in 0..generations {
            log::info!("Growing generation #{gen}");
            let now = std::time::Instant::now();
            self.next_generation();
            log::info!("Generation grew up in {:?}", now.elapsed());
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
        // always do shifts first, then rotations
        // first, shift it all the way to the left
        let mut game = self.clone();
        for _ in 0..WIDTH {
            game.shift(Direction::Left);
        }
        for shifts in 0..WIDTH {
            let mut game = game.clone();
            for _ in 0..shifts {
                game.shift(Direction::Right);
            }
            for rotations in 0..4 {
                let mut game = game.clone();
                for _ in 0..rotations {
                    game.rotate();
                }
                game.hard_drop();
                states.push((game, shifts, rotations));
            }
        }
        states
    }
}
