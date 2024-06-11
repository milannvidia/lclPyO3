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
    fn keep_running(&mut self) -> bool {
        if self.current_iterations < self.max_iterations {
            self.current_iterations += 1;
            true
        } else {
            false
        }
    }
    fn init(&mut self) {
        self.current_iterations = 0;
    }

    fn check_variable(&mut self, _var: usize) -> bool {
        true
    }
}
