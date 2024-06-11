pub trait TerminationFunction: Send {
    fn keep_running(&mut self) -> bool;
    fn init(&mut self);
    fn check_variable(&mut self, var: usize) -> bool;
}
