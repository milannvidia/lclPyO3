use std::isize;

use super::TerminationFunction;
pub struct MustImprove {
    pub best: isize,
    flipflop: bool,
    minimize: bool,
}
impl MustImprove {
    pub fn new(minimize: bool) -> Self {
        if minimize {
            return MustImprove {
                best: isize::MAX,
                flipflop: true,
                minimize,
            };
        } else {
            return MustImprove {
                best: isize::MIN,
                flipflop: true,
                minimize,
            };
        }
    }
}

impl TerminationFunction for MustImprove {
    fn keep_running(&mut self) -> bool {
        self.flipflop
    }
    fn init(&mut self) {
        self.best = if self.minimize {
            isize::MAX
        } else {
            isize::MIN
        };
        self.flipflop = true;
    }

    fn check_variable(&mut self, _var: isize) -> bool {
        true
    }

    fn iteration_done(&mut self) {}

    fn check_new_variable(&mut self, var: isize) {
        if (self.best < var) == self.minimize {
            self.flipflop = false;
        } else {
            self.best = var;
        }
    }
}
