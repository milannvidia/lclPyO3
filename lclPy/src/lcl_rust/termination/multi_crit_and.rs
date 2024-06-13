use super::TerminationFunction;
use std::sync::{Arc, Mutex};
pub struct MultiCritAnd {
    pub critirions: Vec<Arc<Mutex<dyn TerminationFunction>>>,
}
impl MultiCritAnd {
    pub fn new(and: Vec<Arc<Mutex<dyn TerminationFunction>>>) -> Self {
        let cloned = and.iter().map(|f| Arc::clone(f)).collect();
        MultiCritAnd { critirions: cloned }
    }
    pub fn add(&mut self, crit: &Arc<Mutex<dyn TerminationFunction>>) {
        self.critirions.push(Arc::clone(crit));
    }
}

impl TerminationFunction for MultiCritAnd {
    fn keep_running(&mut self) -> bool {
        for crit in &mut self.critirions {
            if !crit.lock().unwrap().keep_running() {
                return false;
            }
        }
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, var: isize) -> bool {
        for crit in &mut self.critirions {
            if !crit.lock().unwrap().check_variable(var) {
                return false;
            }
        }
        true
    }

    fn iteration_done(&mut self) {
        for crit in &mut self.critirions {
            crit.lock().unwrap().iteration_done();
        }
    }
}
