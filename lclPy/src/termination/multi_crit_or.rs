use super::TerminationFunction;
use std::sync::{Arc, Mutex};
pub struct MultiCritOr {
    pub criterion: Vec<Arc<Mutex<dyn TerminationFunction>>>,
}
impl MultiCritOr {
    pub fn new(and: Vec<Arc<Mutex<dyn TerminationFunction>>>) -> Self {
        let cloned = and.iter().map(|f| Arc::clone(f)).collect();
        MultiCritOr { criterion: cloned }
    }
    pub fn add(&mut self, crit: &Arc<Mutex<dyn TerminationFunction>>) {
        self.criterion.push(Arc::clone(crit));
    }
}

impl TerminationFunction for MultiCritOr {
    fn keep_running(&self) -> bool {
        for crit in &self.criterion {
            if crit.lock().unwrap().keep_running() {
                return true;
            }
        }
        false
    }
    fn init(&mut self) {
        for crit in &mut self.criterion {
            crit.lock().unwrap().init();
        }
    }

    fn check_variable(&mut self, var: isize) -> bool {
        for crit in &mut self.criterion {
            if crit.lock().unwrap().check_variable(var) {
                return true;
            }
        }
        false
    }

    fn check_new_variable(&mut self, var: isize) {
        for crit in &mut self.criterion {
            crit.lock().unwrap().check_new_variable(var);
        }
    }

    fn iteration_done(&mut self) {
        for crit in &mut self.criterion {
            crit.lock().unwrap().iteration_done();
        }
    }
}
