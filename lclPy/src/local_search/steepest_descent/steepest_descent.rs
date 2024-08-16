use super::LocalSearch;
use crate::problem::Problem;
use crate::termination::TerminationFunction;
use crate::MoveType;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct SteepestDescent {
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: TerminationFunction,
    minimize: bool,
}
impl SteepestDescent {
    pub fn new(
        minimize: bool,
        problem: &Arc<Mutex<dyn Problem>>,
        termination: &TerminationFunction,
    ) -> Self {
        let mut term = termination.clone();
        term.set_goal(minimize);
        SteepestDescent {
            problem: problem.clone(),
            termination: term,
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
    ///# use lclPyO3::local_search::{LocalSearch, SteepestDescent};
    ///# use lclPyO3::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    ///# use lclPyO3::termination::TerminationFunction;
    ///    let distance_matrix: Vec<Vec<f64>> = vec![
    ///        vec![0.0, 2.0, 5.0, 8.0],
    ///        vec![2.0, 0.0, 4.0, 1.0],
    ///        vec![5.0, 4.0, 0.0, 7.0],
    ///        vec![8.0, 1.0, 7.0, 0.0],
    ///    ];
    ///# let move_type=MoveType::tsp(Some(0)) ;
    ///# let eval=Evaluation::tsp(distance_matrix);
    ///# let problem:Arc<Mutex<dyn Problem>>=Arc::new(Mutex::new(ArrayProblem::new(&move_type,&eval)));
    ///# let termination=TerminationFunction::always_true();
    ///
    /// let mut sim=SteepestDescent::new(true,&problem,&termination);
    /// let data=sim.run(false).last().unwrap().1;
    /// assert_eq!(data,15.0);
    /// ```
    fn run(&mut self, log: bool) -> Vec<(u128, f64, f64, u64)> {
        let mut problem = self.problem.lock().unwrap();
        let mut current = problem.eval();
        let mut best = current;
        let now = Instant::now();
        let mut iterations = 0;
        let mut data: Vec<(u128, f64, f64, u64)> = vec![];

        self.termination.init();
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }
        while self.termination.keep_running() {
            // while iterations<100{
            let mut best_mov = (0, 0);
            let mut best_delta = if self.minimize { f64::MAX } else { f64::MIN };
            for mov in problem.get_all_mov() {
                let delta = problem.delta_eval(mov, None);
                if (delta <= best_delta) == self.minimize || (delta >= best_delta) != self.minimize
                {
                    best_delta = delta;
                    best_mov = mov;
                }
            }
            current = current + best_delta;

            self.termination.check_new_variable(current);
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
            self.termination.iteration_done();
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

    fn set_termination(&mut self, termination: &TerminationFunction) {
        self.termination = termination.clone();
    }
}
#[cfg(test)]
mod tests {
    use crate::local_search::{LocalSearch, SteepestDescent};
    use crate::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    use crate::termination::TerminationFunction;
    use std::sync::{Arc, Mutex};

    #[test]
    fn steepest_descent_test() {
        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 2.0, 5.0, 8.0],
            vec![2.0, 0.0, 4.0, 1.0],
            vec![5.0, 4.0, 0.0, 7.0],
            vec![8.0, 1.0, 7.0, 0.0],
        ];
        let move_type = MoveType::tsp(Some(0));
        let eval = Evaluation::tsp(distance_matrix);
        let problem: Arc<Mutex<dyn Problem>> =
            Arc::new(Mutex::new(ArrayProblem::new(&move_type, &eval)));
        let termination = TerminationFunction::always_true();

        let mut sim = SteepestDescent::new(true, &problem, &termination);
        let data = sim.run(false).last().unwrap().1;
        assert_eq!(data, 15.0);
    }
}
