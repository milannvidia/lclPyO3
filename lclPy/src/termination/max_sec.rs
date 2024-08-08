use super::TerminationFunction;
use std::time::Instant;
pub struct MaxSec {
    pub time: Instant,
    pub max_sec: u64,
}
impl MaxSec {
    pub fn new(max_sec: u64) -> Self {
        MaxSec {
            time: Instant::now(),
            max_sec,
        }
    }
}
impl TerminationFunction for MaxSec {
    fn keep_running(&self) -> bool {
        self.time.elapsed().as_secs() < self.max_sec
    }

    fn init(&mut self) {
        self.time = Instant::now();
    }

    fn check_variable(&mut self, _var: isize) -> bool {
        true
    }

    fn check_new_variable(&mut self, _var: isize) {}

    fn iteration_done(&mut self) {}
}
