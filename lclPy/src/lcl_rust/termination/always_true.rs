use super::TerminationFunction;
pub struct AlwaysTrue {}

impl AlwaysTrue {
    pub fn new() -> Self {
        AlwaysTrue {}
    }
}
impl TerminationFunction for AlwaysTrue {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, _var: usize) -> bool {
        true
    }
}
