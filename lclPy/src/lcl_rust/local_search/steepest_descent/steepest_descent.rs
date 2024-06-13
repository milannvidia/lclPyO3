use super::LocalSearch;
use crate::lcl_rust::problem::Problem;
use crate::lcl_rust::termination::TerminationFunction;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct SteepestDescent {
    // pub(crate) problem:&'a mut dyn Problem,
    // pub(crate) termination:&'a mut dyn TerminationFunction,
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: Arc<Mutex<dyn TerminationFunction>>,
    minimize: bool,
}
impl SteepestDescent {
    pub fn new(
        minimize: bool,
        problem: Arc<Mutex<dyn Problem>>,
        termination: Arc<Mutex<dyn TerminationFunction>>,
    ) -> Self {
        SteepestDescent {
            problem,
            termination,
            minimize,
        }
    }
}
impl LocalSearch for SteepestDescent {
    fn reset(&mut self) {
        self.problem.lock().unwrap().reset();
    }
    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let mut problem = self.problem.lock().unwrap();
        let mut termination = self.termination.lock().unwrap();
        let mut current: isize = problem.eval() as isize;
        let mut best: isize = current;
        let now = Instant::now();
        let mut iterations = 0;
        let mut data: Vec<(u128, isize, isize, usize)> = vec![];

        termination.init();
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }
        while termination.keep_running() {
            // while iterations<100{
            let mut best_mov = (0, 0);
            let mut best_delta = if self.minimize {
                isize::MAX
            } else {
                isize::MIN
            };
            for mov in problem.get_all_mov() {
                let delta = problem.delta_eval(mov);
                if (delta < best_delta) == self.minimize {
                    best_delta = delta;
                    best_mov = mov;
                }
            }
            current = current + best_delta;

            if (current < best) == self.minimize {
                problem.do_mov(best_mov);
                problem.set_best();
                best = current;
                if log {
                    data.push((now.elapsed().as_nanos(), best, current, iterations));
                }
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
