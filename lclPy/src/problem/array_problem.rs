use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::{Evaluation, MoveType, Problem};

pub struct ArrayProblem {
    state: Vec<usize>,
    best_solution: Vec<usize>,
    move_type: MoveType,
    evaluation: Evaluation,
}
impl ArrayProblem {
    pub fn new(move_type: &MoveType, evaluation: &Evaluation) -> Self {
        let len = evaluation.length();
        let array_problem = ArrayProblem {
            state: (0..len).collect(),
            best_solution: (0..len).collect(),
            move_type: move_type.clone(),
            evaluation: evaluation.clone(),
        };
        array_problem
    }

    pub fn state(&self) -> &Vec<usize> {
        &self.state
    }

    pub fn best_solution(&self) -> &Vec<usize> {
        &self.best_solution
    }
}
impl Problem for ArrayProblem {
    fn get_mov(&mut self) -> (usize, usize) {
        self.move_type.get_mov()
    }

    fn get_all_mov(&mut self) -> Vec<(usize, usize)> {
        self.move_type.get_all_mov()
    }

    fn do_mov(&mut self, indices: (usize, usize), move_type: Option<&MoveType>) {
        match move_type {
            Some(x) => x.do_move(&mut self.state, indices),
            None => self.move_type.do_move(&mut self.state, indices),
        }
    }

    fn delta_eval(&mut self, indices: (usize, usize), move_type: Option<&MoveType>) -> isize {
        match move_type {
            Some(x) => self.evaluation.delta_eval(indices, x, &mut self.state),
            None => self
                .evaluation
                .delta_eval(indices, &self.move_type, &mut self.state),
        }
    }

    fn eval(&self) -> usize {
        self.evaluation.eval(&self.state)
    }

    fn reset(&mut self) {
        self.state = (0..self.state.len()).collect();
        self.best_solution = (0..self.state.len()).collect();
    }

    fn set_best(&mut self) {
        self.best_solution = self.state.to_vec();
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.state.hash(&mut hasher);
        hasher.finish()
    }

    fn get_move_type(&self) -> &MoveType {
        &self.move_type
    }
}
