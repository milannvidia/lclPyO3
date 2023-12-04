use std::time::{Instant};

pub trait TerminationFunction {
    fn keep_running(&self) ->bool;
    fn init(&mut self);
}
pub struct MaxSec {
    pub time:Instant,
    pub max_sec:u64,
}
impl TerminationFunction for MaxSec{
    fn keep_running(&self) -> bool {
        return if self.time.elapsed().as_secs() < self.max_sec {
            true
        } else {
            false
        }
    }

    fn init(&mut self) {
        self.time=Instant::now();
    }
}
