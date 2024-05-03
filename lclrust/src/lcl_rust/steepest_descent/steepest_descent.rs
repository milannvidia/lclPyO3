use crate::lcl_rust::problems::Problem;
use crate::lcl_rust::terminationfunc::TerminationFunction;
use std::time::Instant;

pub struct SteepestDescent {
    // pub(crate) problem:&'a mut dyn Problem,
    // pub(crate) termination:&'a mut dyn TerminationFunction,
    pub(crate) problem: Box<dyn Problem>,
    pub(crate) termination: Box<dyn TerminationFunction>,
}

impl SteepestDescent {
    pub fn reset(&mut self) {
        self.problem.reset()
    }
    pub fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let mut current: isize = self.problem.eval() as isize;
        let mut best: isize = current;
        let now = Instant::now();
        let mut iterations = 0;
        let mut data: Vec<(u128, isize, isize, usize)> = vec![];

        self.termination.init();
        if log {
            data.append(&mut vec![(
                now.elapsed().as_nanos(),
                best,
                current,
                iterations,
            )]);
        }
        while self.termination.keep_running() {
            // while iterations<100{
            let mut best_mov = (0, 0);
            let mut best_delta = isize::MAX;
            for mov in self.problem.all_mov() {
                let delta = self.problem.delta(mov);
                if delta < best_delta {
                    best_delta = delta;
                    best_mov = mov;
                }
            }
            current = current + best_delta;

            if current < best {
                let test1 = self.problem.eval();
                self.problem.domov(best_mov);
                self.problem.set_best();
                let test2 = self.problem.eval();
                best = current;
                if log {
                    data.append(&mut vec![(
                        now.elapsed().as_nanos(),
                        best,
                        current,
                        iterations,
                    )]);
                }
            } else {
                break;
            }
            iterations += 1;
        }
        data.append(&mut vec![(
            now.elapsed().as_nanos(),
            best,
            current,
            iterations,
        )]);

        return data;
    }
}
