use super::TerminationFunction;
pub struct MaxIterations {
    pub max_iterations: usize,
    current_iterations: usize,
}
impl MaxIterations {
    pub fn new(max_iterations: usize) -> Self {
        MaxIterations {
            max_iterations,
            current_iterations: 0,
        }
    }
}
impl TerminationFunction for MaxIterations {
    fn keep_running(&self) -> bool {
        self.current_iterations < self.max_iterations
    }
    fn init(&mut self) {
        self.current_iterations = 0;
    }

    fn check_variable(&mut self, _var: isize) -> bool {
        true
    }

    fn check_new_variable(&mut self, _var: isize) {}

    fn iteration_done(&mut self) {
        self.current_iterations += 1;
    }
}
