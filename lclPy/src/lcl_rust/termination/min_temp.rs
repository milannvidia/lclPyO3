use super::TerminationFunction;
pub struct MinTemp {
    pub min_temp: usize,
}
impl MinTemp {
    pub fn new(min_temp: usize) -> Self {
        MinTemp { min_temp }
    }
}
impl TerminationFunction for MinTemp {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, var: usize) -> bool {
        var > self.min_temp
    }
}
