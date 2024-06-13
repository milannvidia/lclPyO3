use super::TerminationFunction;
pub struct MustImprove {
    pub best: isize,
}
impl MustImprove {
    pub fn new(minimize: bool) -> Self {
        if minimize {
            return MustImprove { best: isize::MAX };
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
        self.best = isize::MAX;
    }

    fn check_variable(&mut self, var: isize) -> bool {
        if self.best < var {
            false
        } else {
            self.best = var;
            true
        }
    }

    fn iteration_done(&mut self) {}
}
