use super::LocalSearch;
use crate::problem::Problem;
use crate::termination::TerminationFunction;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::vec;

pub struct TabuSearch {
    // pub(crate) problem: &'a mut dyn Problem,
    // pub(crate) termination: &'a mut dyn TerminationFunction,
    pub(crate) problem: Arc<Mutex<dyn Problem>>,
    pub(crate) termination: Arc<Mutex<dyn TerminationFunction>>,
    minimize: bool,
    // pub(crate) tabu_list: TabuList<T>,
}
impl TabuSearch {
    pub fn new(
        problem: &Arc<Mutex<dyn Problem>>,
        termination: &Arc<Mutex<dyn TerminationFunction>>,
        minimize: bool,
    ) -> Self {
        TabuSearch {
            problem: problem.clone(),
            termination: termination.clone(),
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
    ///# use rand::rngs::SmallRng;
    ///# use lclpy::local_search::{LocalSearch, TabuSearch};
    ///# use lclpy::problem::{ArrayProblem, Evaluation, MoveType};
    ///# use lclpy::termination::MaxSec;
    ///
    ///# let distamce_matrix=vec![[0, 2, 5, 8],[2, 0, 4, 1],[5, 4, 0, 7],[8, 1, 7, 0]];
    ///# let rng=SmallRng::seed_from_u64(0);
    ///# let move_type=MoveType::Tsp {rng,size:4};
    ///# let eval=Evaluation::Tsp {distance_matrix,symmetric:true};
    ///# let problem=Arc::new(Mutex::new(ArrayProblem::new(&move_type,&eval)));
    ///# let termination=Arc::new(Mutex::new(MaxSec::new(1)));
    /// let mut sim=TabuSearch::new(&problem,&termination,true);
    /// let data=sim.run(false).last()?.1;
    /// let sol:Vec<usize>=vec![0,1,3,2];
    /// let res:Vec<usize>=problem.lock().unwrap().best_solution().clone();
    ///
    /// assert_eq!(data,15isize);
    /// assert_eq!(sol,res);
    /// ```
    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        let mut problem = self.problem.lock().unwrap();
        let mut termination = self.termination.lock().unwrap();
        let mut current: isize = problem.eval() as isize;
        let mut best: isize = current;
        let now = Instant::now();
        let mut iterations = 0;
        let mut data: Vec<(u128, isize, isize, usize)> = vec![];
        let mut tabu_list: Vec<u64> = vec![];
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }

        termination.init();
        while termination.keep_running() {
            let mut best_mov: Option<(usize, usize)> = None;
            let mut best_delta = isize::MAX;
            let mut best_hash: u64 = 0;

            for mov in problem.get_all_mov() {
                let delta = problem.delta_eval(mov, None);
                let hash = problem.hash();
                if !tabu_list.contains(&hash) && (best_delta < delta) == self.minimize {
                    best_delta = delta;
                    best_mov = Some(mov);
                    best_hash = hash;
                }
            }
            current = current + best_delta;
            if best_mov.is_some() {
                problem.do_mov(best_mov.unwrap(), None);
                if (best < current) == self.minimize {
                    best = current;
                }
                tabu_list.push(best_hash);
                if log {
                    data.push((now.elapsed().as_nanos(), best, current, iterations));
                }
                termination.check_variable(current);
            } else {
                break;
            }
            iterations += 1;
            termination.iteration_done();
        }
        data.push((now.elapsed().as_nanos(), best, current, iterations));
        data
    }
}
