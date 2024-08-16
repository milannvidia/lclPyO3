use super::LocalSearch;
use crate::problem::Problem;
use crate::termination::TerminationFunction;
use crate::MoveType;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::vec;

pub struct VariableNeighborhood {
    problem: Arc<Mutex<dyn Problem>>,
    termination: TerminationFunction,
    minimize: bool,
    neighborhood: usize,
}

impl VariableNeighborhood {
    pub fn new(
        problem: &Arc<Mutex<dyn Problem>>,
        termination: &TerminationFunction,
        minimize: bool,
    ) -> Self {
        let mut term = termination.clone();
        term.set_goal(minimize);
        VariableNeighborhood {
            problem: problem.clone(),
            termination: term,
            minimize,
            neighborhood: 0,
        }
    }

    fn get_all_mov_select(&self, move_type: &MoveType) -> Vec<(usize, usize)> {
        match move_type {
            MoveType::Reverse { rng: _, size: _ } => move_type.get_all_mov(),
            MoveType::Swap { rng: _, size: _ } => move_type.get_all_mov(),
            MoveType::Tsp { rng: _, size: _ } => move_type.get_all_mov(),
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => move_types[self.neighborhood].get_all_mov(),
        }
    }

    fn delta_eval(&self, problem: &mut dyn Problem, mov: (usize, usize)) -> f64 {
        match problem.get_move_type().to_owned() {
            MoveType::Reverse { .. } | MoveType::Swap { .. } | MoveType::Tsp { .. } => {
                problem.delta_eval(mov, None)
            }
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => {
                let move_type = Some(&move_types[self.neighborhood]);
                problem.delta_eval(mov, move_type)
            }
        }
    }

    fn do_move(&self, problem: &mut dyn Problem, best_move: (usize, usize)) {
        match problem.get_move_type().to_owned() {
            MoveType::Reverse { .. } | MoveType::Swap { .. } | MoveType::Tsp { .. } => {
                problem.do_mov(best_move, None)
            }
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => {
                let move_type = Some(&move_types[self.neighborhood]);
                problem.do_mov(best_move, move_type)
            }
        }
    }
}
impl LocalSearch for VariableNeighborhood {
    fn reset(&mut self) {
        self.problem.lock().unwrap().reset()
    }

    /// Runs the meta heuristic variable neighborhood search.
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
    ///# use lclPyO3::local_search::LocalSearch;
    ///# use lclPyO3::local_search::vns::VariableNeighborhood;
    ///# use lclPyO3::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    ///# use lclPyO3::termination::TerminationFunction;
    ///    let distance_matrix: Vec<Vec<f64>> = vec![
    ///        vec![0.0, 2.0, 5.0, 8.0],
    ///        vec![2.0, 0.0, 4.0, 1.0],
    ///        vec![5.0, 4.0, 0.0, 7.0],
    ///        vec![8.0, 1.0, 7.0, 0.0],
    ///    ];
    ///# let move_type_0=MoveType::tsp(Some(0));
    ///# let move_type_1=MoveType::reverse(Some(0));
    ///# let move_type_2=MoveType::swap(Some(0));
    ///# let move_type=MoveType::multi_neighbor(vec![move_type_0,move_type_1,move_type_2],None);
    ///# let eval=Evaluation::Tsp {distance_matrix,symmetric:true};
    ///# let problem:Arc<Mutex<dyn Problem>>=Arc::new(Mutex::new(ArrayProblem::new(&move_type,&eval)));
    ///# let termination=TerminationFunction::max_sec(1);
    ///
    /// let mut sim=VariableNeighborhood::new(&problem,&termination,true);
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
        if log {
            data.push((now.elapsed().as_nanos(), best, current, iterations));
        }
        self.termination.init();
        while self.termination.keep_running() {
            let mut best_delta = if self.minimize { f64::MAX } else { f64::MIN };
            let mut best_move: Option<(usize, usize)> = None;

            for mov in self.get_all_mov_select(problem.get_move_type()) {
                let delta = self.delta_eval(&mut *(problem), mov);
                if (delta < best_delta) == self.minimize {
                    best_delta = delta;
                    best_move = Some(mov);
                    continue;
                }
            }
            current += best_delta;

            self.termination.check_new_variable(current);

            if (current < best) == self.minimize {
                self.do_move(&mut *problem, best_move.unwrap());
                problem.set_best();
                best = current;
                if log {
                    data.push((now.elapsed().as_nanos(), best, current, iterations))
                }
            } else {
                current -= best_delta;
                match problem.get_move_type() {
                    MoveType::Reverse { rng: _, size: _ }
                    | MoveType::Swap { rng: _, size: _ }
                    | MoveType::Tsp { rng: _, size: _ } => break,
                    MoveType::MultiNeighbor {
                        move_types,
                        weights: _,
                    } => {
                        if self.neighborhood + 1 >= move_types.len() {
                            break;
                        } else {
                            self.neighborhood += 1;
                        }
                    }
                }
            }
            iterations += 1;
            self.termination.iteration_done();
        }
        data.push((now.elapsed().as_nanos(), best, current, iterations));
        data
    }

    fn set_problem(&mut self, problem: &Arc<Mutex<dyn Problem>>) {
        self.problem = problem.clone();
    }

    fn set_termination(&mut self, termination: &TerminationFunction) {
        self.termination = termination.clone();
    }
}
#[cfg(test)]
mod tests {
    use crate::local_search::vns::VariableNeighborhood;
    use crate::local_search::LocalSearch;
    use crate::problem::{ArrayProblem, Evaluation, MoveType, Problem};
    use crate::termination::TerminationFunction;
    use std::sync::{Arc, Mutex};

    #[test]
    fn vns_test() {
        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 2.0, 5.0, 8.0],
            vec![2.0, 0.0, 4.0, 1.0],
            vec![5.0, 4.0, 0.0, 7.0],
            vec![8.0, 1.0, 7.0, 0.0],
        ];
        let move_type_0 = MoveType::tsp(Some(0));
        let move_type_1 = MoveType::swap(Some(0));
        let move_type_2 = MoveType::reverse(Some(0));
        let move_type = MoveType::multi_neighbor(vec![move_type_0, move_type_1, move_type_2], None);
        let eval = Evaluation::tsp(distance_matrix);
        let problem: Arc<Mutex<dyn Problem>> =
            Arc::new(Mutex::new(ArrayProblem::new(&move_type, &eval)));
        let termination = TerminationFunction::max_sec(1);

        let mut sim = VariableNeighborhood::new(&problem, &termination, true);
        let data = sim.run(false).last().unwrap().1;
        assert_eq!(data, 15.0);
    }
}
