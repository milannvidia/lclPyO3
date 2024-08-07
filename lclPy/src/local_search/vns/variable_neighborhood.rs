use super::LocalSearch;
use crate::problem::Problem;
use crate::termination::TerminationFunction;
use crate::MoveType;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::vec;

pub struct VariableNeighborhood {
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: Arc<Mutex<dyn TerminationFunction>>,
    minimize: bool,
    neighborhood: usize,
}

impl VariableNeighborhood {
    pub fn new(
        problem: &Arc<Mutex<dyn Problem>>,
        termination: &Arc<Mutex<dyn TerminationFunction>>,
        minimize: bool,
    ) -> Self {
        VariableNeighborhood {
            problem: problem.clone(),
            termination: termination.clone(),
            minimize,
            neighborhood: 0,
        }
    }

    fn get_all_mov_select(&self, move_type: &MoveType) -> Vec<(usize, usize)> {
        match move_type {
            MoveType::Reverse { rng: _, size: _ } => return move_type.get_all_mov(),
            MoveType::Swap { rng: _, size: _ } => return move_type.get_all_mov(),
            MoveType::Tsp { rng: _, size: _ } => return move_type.get_all_mov(),
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => return move_types[self.neighborhood].get_all_mov(),
        }
    }

    fn delta_eval(&self, problem: &mut dyn Problem, mov: (usize, usize)) -> isize {
        match problem.get_move_type().to_owned() {
            MoveType::Reverse { .. } | MoveType::Swap { .. } | MoveType::Tsp { .. } => {
                problem.delta_eval(mov, None)
            }
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => {
                let move_type = Some(&move_types[self.neighborhood]);
                problem.delta_eval(mov, move_type)
            }
        }
    }

    fn do_move(&self, problem: &mut dyn Problem, best_move: (usize, usize)) {
        match problem.get_move_type().to_owned() {
            MoveType::Reverse { .. } | MoveType::Swap { .. } | MoveType::Tsp { .. } => {
                problem.do_mov(best_move, None)
            }
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => {
                let move_type = Some(&move_types[self.neighborhood]);
                problem.do_mov(best_move, move_type)
            }
        }
    }
}
impl LocalSearch for VariableNeighborhood {
    fn reset(&mut self) {
        self.problem.lock().unwrap().reset()
    }

    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let mut problem = self.problem.lock().unwrap();
        let mut termination = self.termination.lock().unwrap();
        let mut current: isize = problem.eval() as isize;
        let mut best: isize = current;
        let now = Instant::now();
        let mut iterations = 0;
        let mut data: Vec<(u128, isize, isize, usize)> = vec![];
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }
        termination.init();
        while termination.keep_running() {
            let mut best_delta = if self.minimize {
                isize::MAX
            } else {
                isize::MIN
            };
            let mut best_move: Option<(usize, usize)> = None;

            for mov in self.get_all_mov_select(problem.get_move_type()) {
                let delta: isize = self.delta_eval(&mut *(problem), mov);
                if (delta < best_delta) == self.minimize {
                    best_delta = delta;
                    best_move = Some(mov);
                    continue;
                }
            }
            current += best_delta;

            termination.check_new_variable(current);

            if (current < best) == self.minimize {
                self.do_move(&mut *problem, best_move.unwrap());
                problem.set_best();
                best = current;
                if log {
                    data.push((now.elapsed().as_nanos(), best, current, iterations))
                }
            } else {
                current -= best_delta;
                match problem.get_move_type() {
                    MoveType::Reverse { rng: _, size: _ }
                    | MoveType::Swap { rng: _, size: _ }
                    | MoveType::Tsp { rng: _, size: _ } => break,
                    MoveType::MultiNeighbor {
                        move_types,
                        weights: _,
                    } => {
                        if self.neighborhood + 1 >= move_types.len() {
                            break;
                        } else {
                            self.neighborhood += 1;
                        }
                    }
                }
            }
            iterations += 1;
            termination.iteration_done();
        }
        data.push((now.elapsed().as_nanos(), best, current, iterations));
        data
    }
}
