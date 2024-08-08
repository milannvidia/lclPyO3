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
    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)>;
}
