use std::time::Instant;

use super::TerminationFunction;
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
    fn keep_running(&mut self) -> bool {
        return if self.time.elapsed().as_secs() < self.max_sec {
            true
        } else {
            false
        };
    }

    fn init(&mut self) {
        self.time = Instant::now();
    }

    fn check_variable(&mut self, _var: usize) -> bool {
        true
    }
}
