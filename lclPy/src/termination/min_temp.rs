use super::TerminationFunction;
pub struct MinTemp {
    pub min_temp: isize,
}
impl MinTemp {
    pub fn new(min_temp: isize) -> Self {
        MinTemp { min_temp }
    }
}
impl TerminationFunction for MinTemp {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, var: isize) -> bool {
        var > self.min_temp
    }

    fn iteration_done(&mut self) {}

    fn check_new_variable(&mut self, _var: isize) {}
}
