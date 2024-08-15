use super::*;
use crate::termination::TerminationFunction;
use crate::{problem::Problem, MoveType};
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct SimulatedAnnealing {
    temp: usize,
    start_temp: usize,
    minimize: bool,
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: TerminationFunction,
    pub(crate) cool_func: CoolingFunction,
    pub(crate) iter_temp: IterationsTemperature,
}
impl SimulatedAnnealing {
    pub fn new(
        temp: usize,
        minimize: bool,
        problem: &Arc<Mutex<dyn Problem>>,
        termination: &TerminationFunction,
        cooling: &CoolingFunction,
        iteration_calc: &IterationsTemperature,
    ) -> Self {
        let mut res = SimulatedAnnealing {
            temp,
            minimize,
            start_temp: temp,
            termination: termination.clone(),
            problem: problem.clone(),
            cool_func: cooling.clone(),
            iter_temp: iteration_calc.clone(),
        };
        res.set_goal(minimize);
        res
    }
}
impl LocalSearch for SimulatedAnnealing {
    fn reset(&mut self) {
        self.problem.lock().unwrap().reset();
    }

    /// Runs the meta heuristic simulated annealing.
    ///
    /// # Arguments
    ///
    /// * `log`: Whether intermediate results are tracked or not.
    ///
    /// returns: a vector of tuples.
    /// tuple.0 = a timestamp
    /// tuple.1 = best score found
    /// tuple.2 = current score
    /// tuple.3 = #iterations
    ///
    /// # Examples
    ///
    /// ```
    ///# use std::sync::{Arc, Mutex};
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::local_search::simulated_annealing::CoolingFunction::GeometricCooling;
    ///# use lclpy::local_search::simulated_annealing::IterationsTemperature::ConstIterTemp;
    ///# use lclpy::local_search::{LocalSearch, SimulatedAnnealing};
    ///# use lclpy::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    ///# use lclpy::termination::{MinTemp, TerminationFunction};
    ///
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    ///# let rng=SmallRng::seed_from_u64(0);
    ///# let move_type=MoveType::Tsp {rng,size:4};
    ///# let eval=Evaluation::Tsp {distance_matrix,symmetric:true};
    ///# let problem:Arc<Mutex<dyn Problem>>=Arc::new(Mutex::new(ArrayProblem::new(&move_type,&eval)));
    ///# let cooling=GeometricCooling {alpha:0.75f64};
    ///# let termination:Arc<Mutex<dyn TerminationFunction>>=Arc::new(Mutex::new(MinTemp::new(10)));
    ///# let iter=ConstIterTemp {iterations:1000};
    ///
    /// let mut sim=SimulatedAnnealing::new(2000,true,&problem,&termination,&cooling,&iter);
    /// let data=sim.run(false).last().unwrap().1;
    /// assert_eq!(data,15);
    /// ```
    fn run(&mut self, log: bool) -> Vec<(u128, f64, f64, usize)> {
        let mut problem = self.problem.lock().unwrap();
        self.temp = self.start_temp;
        let e = std::f64::consts::E;
        let mut iterations: usize = 0;
        let now = Instant::now();
        let mut current = problem.eval();
        let mut best = current;
        let mut data: Vec<(u128, f64, f64, usize)> = vec![];
        let mut rng = rand::thread_rng();

        problem.set_best();
        self.termination.init();

        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }

        while self.termination.keep_running() {
            for _ in 0..self.iter_temp.get_iterations(self.temp) {
                if !self.termination.keep_running() {
                    break;
                }

                let mov = problem.get_mov();
                let delta = problem.delta_eval(mov, None);

                if (delta <= 0.0) == self.minimize || (delta >= 0.0) != self.minimize {
                    problem.do_mov(mov, None);
                    current += delta;
                    if (current < best) == self.minimize {
                        problem.set_best();
                        best = current;
                    }

                    if log {
                        data.push((now.elapsed().as_nanos(), best, current, iterations));
                    }
                } else {
                    let exp: f64 = -(delta as f64) / (self.temp as f64);
                    let probability: f64 = e.powf(exp);
                    let random: f64 = rng.gen();
                    if probability > random {
                        problem.do_mov(mov, None);
                        current += delta;
                        if log {
                            data.push((now.elapsed().as_nanos(), best, current, iterations));
                        }
                    }
                }
                iterations += 1;
                self.termination.iteration_done();
            }
            self.temp = self.cool_func.get_next_temp(self.temp);
            if !self.termination.check_variable(self.temp as isize) {
                break;
            }
        }

        data.push((now.elapsed().as_nanos(), best, current, iterations));
        data
    }

    fn set_problem(&mut self, problem: &Arc<Mutex<dyn Problem>>) {
        if let MoveType::MultiNeighbor { .. } = problem.lock().unwrap().get_move_type() {
            panic!("Can't use multiNeighbor in Simulated Annealing")
        } else {
            self.problem = problem.clone();
        }
    }

    fn set_termination(&mut self, termination: &TerminationFunction) {
        self.termination = termination.clone();
    }

    fn set_goal(&mut self, minimize: bool) {
        self.termination.set_goal(minimize)
    }
}
#[cfg(test)]
mod tests {
    use crate::local_search::simulated_annealing::CoolingFunction::GeometricCooling;
    use crate::local_search::simulated_annealing::IterationsTemperature::ConstIterTemp;
    use crate::local_search::{LocalSearch, SimulatedAnnealing};
    use crate::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    use crate::termination::TerminationFunction;
    use rand::prelude::SmallRng;
    use rand::SeedableRng;
    use std::sync::{Arc, Mutex};

    #[test]
    fn simulated_annealing_test() {
        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 2.0, 5.0, 8.0],
            vec![2.0, 0.0, 4.0, 1.0],
            vec![5.0, 4.0, 0.0, 7.0],
            vec![8.0, 1.0, 7.0, 0.0],
        ];
        let rng = Box::new(SmallRng::seed_from_u64(0));
        let move_type = MoveType::Tsp { rng, size: 4 };
        let eval = Evaluation::Tsp {
            distance_matrix,
            symmetric: true,
        };
        let problem: Arc<Mutex<dyn Problem>> =
            Arc::new(Mutex::new(ArrayProblem::new(&move_type, &eval)));
        let cooling = GeometricCooling { alpha: 0.75f64 };
        let termination = TerminationFunction::MinTemp { min_temp: 10 };
        let iter = ConstIterTemp { iterations: 1000 };

        let mut sim = SimulatedAnnealing::new(2000, true, &problem, &termination, &cooling, &iter);
        let data = sim.run(false).last().unwrap().1;
        assert_eq!(data, 15.0);
    }
}
