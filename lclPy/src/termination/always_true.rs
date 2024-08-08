use super::TerminationFunction;
pub struct AlwaysTrue {}
impl AlwaysTrue {
    pub fn new() -> Self {
        AlwaysTrue {}
    }
}
impl TerminationFunction for AlwaysTrue {
    fn keep_running(&self) -> bool {
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, _var: isize) -> bool {
        true
    }

    fn check_new_variable(&mut self, _var: isize) {}

    fn iteration_done(&mut self) {}
}
