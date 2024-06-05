use crate::lcl_rust::problems::Problem;
use crate::lcl_rust::simulated_annealing::cooling_func::CoolingFunction;
use crate::lcl_rust::simulated_annealing::iter_temp::IterationsTemperature;
use crate::lcl_rust::terminationfunc::TerminationFunction;
use crate::lcl_rust::LocalSearch;
use pyo3::prelude::*;
use rand::Rng;
use std::time::Instant;

#[pyclass(unsendable)]
pub struct SimulatedAnnealing {
    temp: usize,
    start_temp: usize,
    pub(crate) problem: Box<dyn Problem>,
    pub(crate) termination: Box<dyn TerminationFunction>,
    pub(crate) cool_func: Box<dyn CoolingFunction>,
    pub(crate) iter_temp: Box<dyn IterationsTemperature>,
    // pub(crate) termination: &'a mut dyn TerminationFunction,
    // pub(crate) cool_func: &'a mut dyn CoolingFunction,
    // pub(crate) iter_temp: &'a dyn IterationsTemperature,
    // pub(crate) problem: &'a mut dyn Problem,
}
impl SimulatedAnnealing {
    pub fn new(
        temp: usize,
        problem: Box<dyn Problem>,
        termination: Box<dyn TerminationFunction>,
        cooling: Box<dyn CoolingFunction>,
        iteration_calc: Box<dyn IterationsTemperature>,
    ) -> Self {
        SimulatedAnnealing {
            temp,
            start_temp: temp,
            termination: termination,
            problem: problem,
            cool_func: cooling,
            iter_temp: iteration_calc,
        }
    }
}
impl LocalSearch for SimulatedAnnealing {
    fn reset(&mut self) {
        self.problem.reset();
        self.temp = self.start_temp;
    }
    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let e = std::f64::consts::E;
        let mut iterations: usize = 0;
        let now = Instant::now();
        let mut current: isize = self.problem.eval() as isize;
        let mut best = current;
        let mut data: Vec<(u128, isize, isize, usize)> = vec![];
        let mut rng = rand::thread_rng();

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
            for _ in 0..self.iter_temp.get_iterations(self.temp) {
                if !self.termination.keep_running() {
                    break;
                }

                let mov = self.problem.mov();
                let delta = self.problem.delta(mov);

                if delta <= 0 {
                    self.problem.domov(mov);
                    current += delta;

                    if current < best {
                        self.problem.set_best();
                        best = current;
                        if log {
                            data.append(&mut vec![(
                                now.elapsed().as_nanos(),
                                best,
                                current,
                                iterations,
                            )]);
                        }
                    }
                } else {
                    let exp: f64 = -(delta as f64) / self.temp as f64;
                    let probability: f64 = e.powf(exp);
                    let random: f64 = rng.gen();
                    if probability > random {
                        self.problem.domov(mov);
                        current += delta;
                    }
                }
                iterations += 1;
            }
            self.temp = self.cool_func.get_next_temp(self.temp);
            if !self.termination.check_variable(self.temp) {
                break;
            }
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
