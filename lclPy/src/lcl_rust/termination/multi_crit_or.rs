use super::TerminationFunction;
use std::sync::{Arc, Mutex};
pub struct MultiCritOr {
    pub critirions: Vec<Arc<Mutex<dyn TerminationFunction>>>,
}
impl MultiCritOr {
    pub fn new(and: Vec<Arc<Mutex<dyn TerminationFunction>>>) -> Self {
        let cloned = and.iter().map(|f| Arc::clone(f)).collect();
        MultiCritOr { critirions: cloned }
    }
    pub fn add(&mut self, crit: &Arc<Mutex<dyn TerminationFunction>>) {
        self.critirions.push(Arc::clone(crit));
    }
}

impl TerminationFunction for MultiCritOr {
    fn keep_running(&mut self) -> bool {
        for crit in &mut self.critirions {
            if crit.lock().unwrap().keep_running() {
                return true;
            }
        }
        false
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, var: usize) -> bool {
        for crit in &mut self.critirions {
            if crit.lock().unwrap().check_variable(var) {
                return true;
            }
        }
        false
    }
}
