use super::TerminationFunction;
pub struct NoImprove {
    pub best: usize,
    pub max_iterations_without_improve: usize,
    curr_without_improve: usize,
}

impl NoImprove {
    pub fn new(minimize: bool, max_iterations_without_improve: usize) -> Self {
        if minimize {
            return NoImprove {
                best: usize::MAX,
                max_iterations_without_improve,
                curr_without_improve: 0,
            };
        } else {
            return NoImprove {
                best: usize::MIN,
                max_iterations_without_improve,
                curr_without_improve: 0,
            };
        }
    }
}
impl TerminationFunction for NoImprove {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {
        self.best = usize::MAX;
        self.curr_without_improve = 0;
    }

    fn check_variable(&mut self, var: usize) -> bool {
        if self.best < var {
            self.curr_without_improve += 1;
            if self.curr_without_improve > self.max_iterations_without_improve {
                false
            } else {
                true
            }
        } else {
            self.best = var;
            true
        }
    }
}
