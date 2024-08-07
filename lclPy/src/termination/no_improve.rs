use super::TerminationFunction;
pub struct NoImprove {
    pub best: isize,
    pub max_iterations_without_improve: usize,
    curr_without_improve: usize,
    flipflop: bool,
    minimize: bool,
}

impl NoImprove {
    pub fn new(minimize: bool, max_iterations_without_improve: usize) -> Self {
        if minimize {
            return NoImprove {
                best: isize::MAX,
                max_iterations_without_improve,
                curr_without_improve: 0,
                flipflop: true,
                minimize,
            };
        } else {
            return NoImprove {
                best: isize::MIN,
                max_iterations_without_improve,
                curr_without_improve: 0,
                flipflop: true,
                minimize,
            };
        }
    }
}
impl TerminationFunction for NoImprove {
    fn keep_running(&mut self) -> bool {
        self.flipflop
    }
    fn init(&mut self) {
        self.best = if self.minimize {
            isize::MAX
        } else {
            isize::MIN
        };
        self.curr_without_improve = 0;
        self.flipflop = true;
    }

    fn check_variable(&mut self, _var: isize) -> bool {
        true
    }

    fn iteration_done(&mut self) {}

    fn check_new_variable(&mut self, var: isize) {
        if (self.best < var) == self.minimize {
            self.curr_without_improve += 1;
            if self.curr_without_improve > self.max_iterations_without_improve {
                self.flipflop = false;
            }
        } else {
            self.best = var;
        }
    }
}
