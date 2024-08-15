use std::sync::{Arc, Mutex};

use crate::{Problem, TerminationFunction};

pub trait LocalSearch: Send {
    /// Resets the state of the problem to ascending indices.
    fn reset(&mut self);

    /// Runs the given meta heuristic.
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
    fn run(&mut self, log: bool) -> Vec<(u128, f64, f64, u64)>;

    ///Setter for internal problem
    fn set_problem(&mut self, problem: &Arc<Mutex<dyn Problem>>);

    ///Setter for termination function
    fn set_termination(&mut self, termination: &TerminationFunction);
}
