use crate::lcl_rust::problems::Problem;
use crate::lcl_rust::simulated_annealing::cooling_func::CoolingFunction;
use crate::lcl_rust::simulated_annealing::iter_temp::IterationsTemperature;
use crate::lcl_rust::terminationfunc::TerminationFunction;
use rand::Rng;
use std::time::Instant;

pub struct SimulatedAnnealing<'a> {
    pub(crate) problem: &'a mut dyn Problem,
    pub(crate) temp: usize,
    pub(crate) termination: &'a mut dyn TerminationFunction,
    pub(crate) cool_func: &'a mut dyn CoolingFunction,
    pub(crate) iter_temp: &'a dyn IterationsTemperature,
}
impl SimulatedAnnealing<'_> {
    pub fn reset(&mut self) {
        self.problem.reset()
    }
    pub fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
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
            // while true {
            // while iterations < 5000000 {
            //FIXME :: mogelijkse fout dqt self temp als referentie wordt gestuurd
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
