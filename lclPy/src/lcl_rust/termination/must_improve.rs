use super::TerminationFunction;
pub struct MustImprove {
    pub best: usize,
}
impl MustImprove {
    pub fn new(minimize: bool) -> Self {
        if minimize {
            return MustImprove { best: usize::MAX };
        } else {
            return MustImprove { best: 0 };
        }
    }
}

impl TerminationFunction for MustImprove {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {
        self.best = usize::MAX;
    }

    fn check_variable(&mut self, var: usize) -> bool {
        if self.best < var {
            false
        } else {
            self.best = var;
            true
        }
    }
}
