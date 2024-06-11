use super::LocalSearch;
use crate::lcl_rust::problem::Problem;
use crate::lcl_rust::termination::TerminationFunction;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct TabuSearch {
    // pub(crate) problem: &'a mut dyn Problem,
    // pub(crate) termination: &'a mut dyn TerminationFunction,
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: Arc<Mutex<dyn TerminationFunction>>,
    // pub(crate) tabu_list: TabuList<T>,
}

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
        if log {
            data.append(&mut vec![(
                now.elapsed().as_nanos(),
                best,
                current,
                iterations,
            )]);
        }

        while termination.keep_running() {
            let mut best_mov = (0, 0);
            let mut best_delta = isize::MAX;
            let mut delta: isize = 0;

            for mov in problem.all_mov() {
                delta = problem.delta(mov);
            }
        }

        return data;
    }
}
