use super::Problem;
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct BinProblem {
    scoring: DeltaRating,
    weights: Vec<usize>,
    solution: Vec<usize>,
    best_solution: Vec<usize>,
    size: usize,
    max_fill: usize,
    rng: SmallRng,
}

pub(crate) enum DeltaRating {
    ExponentialEmpty,
    Empty,
    NumOffBins,
}
impl BinProblem {
    pub fn new(
        rating: DeltaRating,
        max_fill: usize,
        weights: Vec<usize>,
        seed: Option<u64>,
    ) -> Self {
        let size = weights.len();
        let rng;
        if seed.is_none() {
            rng = rand::rngs::SmallRng::from_entropy();
        } else {
            rng = rand::rngs::SmallRng::seed_from_u64(seed.unwrap());
        }

        BinProblem {
            scoring: rating,
            weights,
            solution: (0..size).collect(),
            best_solution: (0..size).collect(),
            size,
            max_fill,
            rng,
        }
    }
}
impl Problem for BinProblem {
    fn mov(&mut self) -> (usize, usize) {
        let i = self.rng.gen_range(1..self.size);
        let mut j = self.rng.gen_range(1..self.size);
        while i == j {
            j = self.rng.gen_range(1..self.size);
        }
        if j < i {
            return (j, i);
        }
        return (i, j);
    }

    fn all_mov(&mut self) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for i in 0..self.size - 1 {
            for j in i + 1..self.size {
                moves.append(&mut vec![(i, j)])
            }
        }
        return moves;
    }

    fn domov(&mut self, indices: (usize, usize)) {
        self.solution.swap(indices.0, indices.1);
    }

    fn delta(&mut self, indices: (usize, usize)) -> isize {
        let initialscore = self.eval();
        self.domov(indices);
        let nextscore = self.eval();
        self.domov(indices);
        return (nextscore - initialscore) as isize;
    }

    fn eval(&self) -> usize {
        let mut score = 0usize;
        let mut filllevel = 0usize;
        match self.scoring {
            DeltaRating::ExponentialEmpty => {
                for i in 0..self.size {
                    if filllevel + self.weights[self.solution[i]] > self.max_fill {
                        score += (self.max_fill - filllevel).pow(2);
                        filllevel = 0;
                    } else {
                        filllevel += self.weights[self.solution[i]];
                    }
                }
            }
            DeltaRating::Empty => {
                for i in 0..self.size {
                    if filllevel + self.weights[self.solution[i]] > self.max_fill {
                        score += self.max_fill - filllevel;
                        filllevel = 0;
                    } else {
                        filllevel += self.weights[self.solution[i]];
                    }
                }
            }
            DeltaRating::NumOffBins => {
                for i in 0..self.size {
                    if filllevel + self.weights[self.solution[i]] > self.max_fill {
                        score += 1;
                        filllevel = 0;
                    } else {
                        filllevel += self.weights[self.solution[i]];
                    }
                }
            }
        }
        return score;
    }

    fn reset(&mut self) {
        self.solution = (0..self.size).collect()
    }

    fn set_best(&mut self) {
        self.best_solution = self.solution.to_vec();
    }
}
