use super::*;
use crate::lcl_rust::problem::Problem;
use crate::lcl_rust::termination::TerminationFunction;
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct SimulatedAnnealing {
    temp: usize,
    start_temp: usize,
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: Arc<Mutex<dyn TerminationFunction>>,
    pub(crate) cool_func: Arc<dyn CoolingFunction>,
    pub(crate) iter_temp: Arc<dyn IterationsTemperature>,
}
impl SimulatedAnnealing {
    pub fn new(
        temp: usize,
        problem: Arc<Mutex<dyn Problem>>,
        termination: Arc<Mutex<dyn TerminationFunction>>,
        cooling: Arc<dyn CoolingFunction>,
        iteration_calc: Arc<dyn IterationsTemperature>,
    ) -> Self {
        SimulatedAnnealing {
            temp,
            start_temp: temp,
            termination,
            problem,
            cool_func: cooling,
            iter_temp: iteration_calc,
        }
    }
}
impl LocalSearch for SimulatedAnnealing {
    fn reset(&mut self) {
        self.problem.lock().unwrap().reset();
        self.temp = self.start_temp;
    }
    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let mut problem = self.problem.lock().unwrap();
        let mut termination = self.termination.lock().unwrap();

        let e = std::f64::consts::E;
        let mut iterations: usize = 0;
        let now = Instant::now();
        let mut current: isize = problem.eval() as isize;
        let mut best = current;
        let mut data: Vec<(u128, isize, isize, usize)> = vec![];
        let mut rng = rand::thread_rng();

        termination.init();
        if log {
            data.append(&mut vec![(
                now.elapsed().as_nanos(),
                best,
                current,
                iterations,
            )]);
        }
        while termination.keep_running() {
            for _ in 0..self.iter_temp.get_iterations(self.temp) {
                if !termination.keep_running() {
                    break;
                }

                let mov = problem.mov();
                let delta = problem.delta(mov);

                if delta <= 0 {
                    problem.domov(mov);
                    current += delta;

                    if current < best {
                        problem.set_best();
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
                        problem.domov(mov);
                        current += delta;
                    }
                }
                iterations += 1;
            }
            self.temp = self.cool_func.get_next_temp(self.temp);
            if !termination.check_variable(self.temp) {
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
