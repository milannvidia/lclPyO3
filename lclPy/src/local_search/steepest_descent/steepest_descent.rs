use super::LocalSearch;
use crate::problem::Problem;
use crate::termination::TerminationFunction;
use crate::MoveType;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct SteepestDescent {
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: Arc<Mutex<dyn TerminationFunction>>,
    minimize: bool,
}
impl SteepestDescent {
    pub fn new(
        minimize: bool,
        problem: &Arc<Mutex<dyn Problem>>,
        termination: &Arc<Mutex<dyn TerminationFunction>>,
    ) -> Self {
        SteepestDescent {
            problem: problem.clone(),
            termination: termination.clone(),
            minimize,
        }
    }
}
impl LocalSearch for SteepestDescent {
    fn reset(&mut self) {
        self.problem.lock().unwrap().reset();
    }
    /// Runs the meta heuristic steepest descent.
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
    ///# use std::sync::{Arc, Mutex, MutexGuard};
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::local_search::{LocalSearch, SteepestDescent};
    ///# use lclpy::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    ///# use lclpy::termination::{AlwaysTrue, TerminationFunction};
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    ///# let rng=SmallRng::seed_from_u64(0);
    ///# let move_type=MoveType::Tsp {rng,size:4};
    ///# let eval=Evaluation::Tsp {distance_matrix,symmetric:true};
    ///# let problem:Arc<Mutex<dyn Problem>>=Arc::new(Mutex::new(ArrayProblem::new(&move_type,&eval)));
    ///# let termination:Arc<Mutex<dyn TerminationFunction>>=Arc::new(Mutex::new(AlwaysTrue::new()));
    ///
    /// let mut sim=SteepestDescent::new(true,&problem,&termination);
    /// let data=sim.run(false).last().unwrap().1;
    /// assert_eq!(data,15isize);
    /// ```
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
                let delta = problem.delta_eval(mov, None);
                if (delta < best_delta) == self.minimize {
                    best_delta = delta;
                    best_mov = mov;
                }
            }
            current = current + best_delta;
            termination.check_variable(current);
            if (current < best) == self.minimize {
                problem.do_mov(best_mov, None);
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

        data
    }

    fn set_problem(&mut self, problem: &Arc<Mutex<dyn Problem>>) {
        if let MoveType::MultiNeighbor { .. } = problem.lock().unwrap().get_move_type() {
            panic!("Can't use multiNeighbor in Steepest Descent")
        } else {
            self.problem = problem.clone();
        }
    }

    fn set_termination(&mut self, termination: &Arc<Mutex<dyn TerminationFunction>>) {
        self.termination = termination.clone();
    }
}
#[cfg(test)]
mod tests {
    use crate::local_search::{LocalSearch, SteepestDescent};
    use crate::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    use crate::termination::{AlwaysTrue, TerminationFunction};
    use rand::prelude::SmallRng;
    use rand::SeedableRng;
    use std::sync::{Arc, Mutex};

    #[test]
    fn steepest_descent_test() {
        let distance_matrix = vec![
            vec![0, 2, 5, 8],
            vec![2, 0, 4, 1],
            vec![5, 4, 0, 7],
            vec![8, 1, 7, 0],
        ];
        let rng = SmallRng::seed_from_u64(0);
        let move_type = MoveType::Tsp { rng, size: 4 };
        let eval = Evaluation::Tsp {
            distance_matrix,
            symmetric: true,
        };
        let problem: Arc<Mutex<dyn Problem>> =
            Arc::new(Mutex::new(ArrayProblem::new(&move_type, &eval)));
        let termination: Arc<Mutex<dyn TerminationFunction>> =
            Arc::new(Mutex::new(AlwaysTrue::new()));

        let mut sim = SteepestDescent::new(true, &problem, &termination);
        let data = sim.run(false).last().unwrap().1;
        assert_eq!(data, 15isize);
    }
}
