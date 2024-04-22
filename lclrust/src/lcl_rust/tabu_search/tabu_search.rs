use crate::lcl_rust::problems::Problem;
use crate::lcl_rust::terminationfunc::TerminationFunction;
use std::collections::HashSet;
use std::time::Instant;

use super::tabu_list::{self, TabuList};

pub struct TabuSearch<'a> {
    pub(crate) problem: &'a mut dyn Problem,
    pub(crate) termination: &'a mut dyn TerminationFunction,
    pub(crate) tabu_list: TabuList<T>,
}

impl TabuSearch<'_> {
    pub fn reset(&mut self) {
        self.problem.reset()
    }
    pub fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let mut current: isize = self.problem.eval() as isize;
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

        while self.termination.keep_running() {
            let mut best_mov = (0, 0);
            let mut best_delta = isize::MAX;
            let mut delta: isize = 0;

            for mov in self.problem.all_mov() {
                delta = self.problem.delta(mov);
            }
        }

        return data;
    }
}
