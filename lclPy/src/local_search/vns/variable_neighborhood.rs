use super::LocalSearch;
use crate::problem::Problem;
use crate::termination::TerminationFunction;
use crate::MoveType;
use std::borrow::Borrow;
use std::error::Error;
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

    fn get_all_mov_select(&self, move_type: MoveType) -> Vec<(usize, usize)> {
        move_type.get_all_mov()
    }

    fn delta_eval(&self, mov: (usize, usize)) -> isize {}

    fn do_move(&self, best_move: Option<(usize, usize)>) {
        todo!()
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

            for mov in self.get_all_mov_select() {
                let delta: isize = self.delta_eval(mov);
                if (delta < best_delta) == self.minimize {
                    best_delta = delta;
                    best_move = Some(mov);
                }
            }
            current += best_delta;

            termination.check_new_variable(current);

            if (current < best) == self.minimize {
                self.do_move(best_move);
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
