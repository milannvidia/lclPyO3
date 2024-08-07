use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{Evaluation, MoveType, Problem};

pub struct ArrayProblem {
    solution: Vec<usize>,
    best_solution: Vec<usize>,
    move_type: MoveType,
    evaluation: Evaluation,
}
impl ArrayProblem {
    pub fn new(move_type: &MoveType, evaluation: &Evaluation) -> Self {
        let len = evaluation.length();
        let array_problem = ArrayProblem {
            solution: (0..len).collect(),
            best_solution: (0..len).collect(),
            move_type: move_type.clone(),
            evaluation: evaluation.clone(),
        };
        return array_problem;
    }
}
impl Problem for ArrayProblem {
    fn get_mov(&mut self) -> (usize, usize) {
        self.move_type.get_mov()
    }

    fn get_all_mov(&mut self) -> Vec<(usize, usize)> {
        self.move_type.get_all_mov()
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
        self.solution = (0..self.solution.len()).collect();
        self.best_solution = (0..self.solution.len()).collect();
    }

    fn set_best(&mut self) {
        self.best_solution = self.solution.to_vec();
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.solution.hash(&mut hasher);
        hasher.finish()
    }

    fn get_move_type(&self) -> &MoveType {
        return &self.move_type;
    }
}
