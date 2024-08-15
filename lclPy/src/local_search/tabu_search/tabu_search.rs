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
        let mut res = TabuSearch {
            problem: problem.clone(),
            termination: termination.clone(),
            minimize,
        };
        res.set_goal(minimize);
        res
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
    ///# use rand::rngs::SmallRng;
    /// use rand::SeedableRng;
    ///# use lclpy::local_search::{LocalSearch, TabuSearch};
    ///# use lclpy::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    ///# use lclpy::termination::{MaxSec, TerminationFunction};
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
    ///# let termination:Arc<Mutex<dyn TerminationFunction>>=Arc::new(Mutex::new(MaxSec::new(1)));
    ///
    /// let mut sim=TabuSearch::new(&problem,&termination,true);
    /// let data=sim.run(false).last().unwrap().1;
    /// assert_eq!(data,15);
    /// ```
    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let mut problem = self.problem.lock().unwrap();
        let mut current: isize = problem.eval() as isize;
        let mut best: isize = current;
        let now = Instant::now();
        let mut iterations = 0;
        let mut data: Vec<(u128, isize, isize, usize)> = vec![];
        let mut tabu_list: Vec<u64> = vec![];
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }

        self.termination.init();
        while self.termination.keep_running() {
            let mut best_mov: Option<(usize, usize)> = None;
            let mut best_delta = isize::MAX;
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

    fn set_goal(&mut self, minimize: bool) {
        self.termination.set_goal(minimize);
    }
}
#[cfg(test)]
mod tests {
    use crate::local_search::{LocalSearch, TabuSearch};
    use crate::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    use crate::termination::TerminationFunction;
    use rand::prelude::SmallRng;
    use rand::SeedableRng;
    use std::sync::{Arc, Mutex};

    #[test]
    fn tabu_search_test() {
        let distance_matrix = vec![
            vec![0, 2, 5, 8],
            vec![2, 0, 4, 1],
            vec![5, 4, 0, 7],
            vec![8, 1, 7, 0],
        ];
        let rng = Box::new(SmallRng::seed_from_u64(0));
        let move_type = MoveType::Tsp { rng, size: 4 };
        let eval = Evaluation::Tsp {
            distance_matrix,
            symmetric: true,
        };
        let problem: Arc<Mutex<dyn Problem>> =
            Arc::new(Mutex::new(ArrayProblem::new(&move_type, &eval)));
        let termination = TerminationFunction::MaxIterations {
            max_iterations: 1000,
            current_iterations: 0,
        };

        let mut sim = TabuSearch::new(&problem, &termination, true);
        let data = sim.run(false).last().unwrap().1;
        assert_eq!(data, 15);
    }
}
