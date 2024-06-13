use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{Evaluation, MoveType, Problem};

pub struct Arrayproblem {
    solution: Vec<usize>,
    best_solution: Vec<usize>,
    size: usize,
    move_type: MoveType,
    evaluation: Evaluation,
    rng: rand::rngs::SmallRng,
}

impl Problem for Arrayproblem {
    fn get_mov(&mut self) -> (usize, usize) {
        self.move_type.get_mov()
    }

    fn get_all_mov(&mut self) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for i in 1..self.size - 1 {
            for j in i + 1..self.size {
                moves.push((i, j))
            }
        }
        return moves;
    }

    fn do_mov(&mut self, indices: (usize, usize)) {
        self.move_type.do_move(&mut self.solution, indices);
    }

    fn delta_eval(&mut self, indices: (usize, usize)) -> isize {
        self.evaluation
            .delta_eval(indices, &self.move_type, &mut self.solution)
    }

    fn eval(&self) -> usize {
        self.evaluation.eval(&self.solution)
    }

    fn reset(&mut self) {
        self.solution = (0..self.size).collect();
        self.best_solution = (0..self.size).collect();
    }

    fn set_best(&mut self) {
        self.best_solution = self.solution.to_vec();
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.solution.hash(&mut hasher);
        hasher.finish()
    }
}
