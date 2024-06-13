use super::{tabu_list, LocalSearch};
use crate::lcl_rust::problem::Problem;
use crate::lcl_rust::termination::TerminationFunction;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::vec;

pub struct TabuSearch {
    // pub(crate) problem: &'a mut dyn Problem,
    // pub(crate) termination: &'a mut dyn TerminationFunction,
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: Arc<Mutex<dyn TerminationFunction>>,
    minimize: bool,
    // pub(crate) tabu_list: TabuList<T>,
}
impl TabuSearch {}

impl LocalSearch for TabuSearch {
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
        let mut tabu_list: Vec<u64> = vec![];
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }

        while termination.keep_running() {
            let mut best_mov: Option<(usize, usize)> = None;
            let mut best_delta = isize::MAX;
            let mut best_hash: u64 = 0;

            for mov in problem.get_all_mov() {
                let delta = problem.delta_eval(mov);
                let hash = problem.hash();
                if !tabu_list.contains(&hash) && (best_delta < delta) == self.minimize {
                    best_delta = delta;
                    best_mov = Some(mov);
                    best_hash = hash;
                }
            }
            current = current + best_delta;
            if best_mov.is_some() {
                problem.do_mov(best_mov.unwrap());
                if (best < current) == self.minimize {
                    best = current;
                }
                tabu_list.push(best_hash);
                if log {
                    data.push((now.elapsed().as_nanos(), best, current, iterations));
                }
                termination.check_variable(current);
            } else {
                break;
            }
            iterations += 1;
            termination.iteration_done();
        }
        data.push((now.elapsed().as_nanos(), best, current, iterations));
        return data;
    }
}
