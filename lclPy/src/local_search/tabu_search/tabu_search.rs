use super::LocalSearch;
use crate::problem::Problem;
use crate::termination::TerminationFunction;
use crate::MoveType;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::vec;

pub struct TabuSearch {
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: TerminationFunction,
    minimize: bool,
}
impl TabuSearch {
    pub fn new(
        problem: &Arc<Mutex<dyn Problem>>,
        termination: &TerminationFunction,
        minimize: bool,
    ) -> Self {
        let mut term = termination.clone();
        term.set_goal(minimize);
        TabuSearch {
            problem: problem.clone(),
            termination: term,
            minimize,
        }
    }
}

impl LocalSearch for TabuSearch {
    fn reset(&mut self) {
        self.problem.lock().unwrap().reset()
    }
    /// Runs the meta heuristic tabu search.
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
    /// use std::time::Instant;
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::local_search::{LocalSearch, TabuSearch};
    ///# use lclpy::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    ///# use lclpy::termination::TerminationFunction;
    ///
    ///# let distance_matrix: Vec<Vec<f64>> = vec![vec![0.0, 2.0, 5.0, 8.0],vec![2.0, 0.0, 4.0, 1.0],vec![5.0, 4.0, 0.0, 7.0],vec![8.0, 1.0, 7.0, 0.0]];
    ///# let move_type=MoveType::tsp(Some(0)) ;
    ///# let eval=Evaluation::tsp (distance_matrix);
    ///# let problem:Arc<Mutex<dyn Problem>>=Arc::new(Mutex::new(ArrayProblem::new(&move_type,&eval)));
    /// let termination=TerminationFunction::max_sec(1);
    ///
    /// let mut sim=TabuSearch::new(&problem,&termination,true);
    /// let data=sim.run(false).last().unwrap().1;
    ///
    /// assert_eq!(data,15.0);
    /// ```
    fn run(&mut self, log: bool) -> Vec<(u128, f64, f64, u64)> {
        let mut problem = self.problem.lock().unwrap();
        let mut current = problem.eval();
        let mut best = current;
        let now = Instant::now();
        let mut iterations = 0;
        let mut data: Vec<(u128, f64, f64, u64)> = vec![];
        let mut tabu_list: Vec<u64> = vec![];
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }

        self.termination.init();
        while self.termination.keep_running() {
            let mut best_mov: Option<(usize, usize)> = None;
            let mut best_delta = if self.minimize { f64::MAX } else { f64::MIN };
            let mut best_hash: u64 = 0;

            for mov in problem.get_all_mov() {
                let delta = problem.delta_eval(mov, None);
                let hash = problem.hash();
                if !tabu_list.contains(&hash)
                    && ((delta < best_delta) == self.minimize
                        || (delta > best_delta) != self.minimize)
                {
                    best_delta = delta;
                    best_mov = Some(mov);
                    best_hash = hash;
                }
            }
            if best_mov.is_some() {
                current = current + best_delta;
                problem.do_mov(best_mov.unwrap(), None);
                if (current < best) == self.minimize || (best < current) != self.minimize {
                    best = current;
                }
                tabu_list.push(best_hash);
                if log {
                    data.push((now.elapsed().as_nanos(), best, current, iterations));
                }
                self.termination.check_new_variable(current);
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
            panic!("Can't use multiNeighbor in Tabu Search")
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
    use crate::local_search::{LocalSearch, TabuSearch};
    use crate::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    use crate::termination::TerminationFunction;
    use std::sync::{Arc, Mutex};

    #[test]
    fn tabu_search_test() {
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
        let termination = TerminationFunction::max_iterations(1000);

        let mut sim = TabuSearch::new(&problem, &termination, true);
        let data = sim.run(false).last().unwrap().1;
        assert_eq!(data, 15.0);
    }
}
