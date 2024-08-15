use std::time::Instant;

#[derive(Clone)]
pub enum TerminationFunction {
    AlwaysTrue {},
    MaxIterations {
        max_iterations: usize,
        current_iterations: usize,
    },
    MaxSec {
        time: Instant,
        max_sec: u64,
    },
    MinTemp {
        min_temp: isize,
    },
    MultiCritAnd {
        criterion: Vec<TerminationFunction>,
    },
    MultiCritOr {
        criterion: Vec<TerminationFunction>,
    },
    MustImprove {
        best: f64,
        flipflop: bool,
        minimize: bool,
    },
    NoImprove {
        best: f64,
        max_iterations_without_improve: usize,
        curr_without_improve: usize,
        flipflop: bool,
        minimize: bool,
    },
}

impl TerminationFunction {
    pub fn keep_running(&self) -> bool {
        match self {
            TerminationFunction::AlwaysTrue {} | TerminationFunction::MinTemp { .. } => true,

            TerminationFunction::MaxIterations {
                max_iterations,
                current_iterations,
            } => current_iterations < max_iterations,

            TerminationFunction::MaxSec { time, max_sec } => time.elapsed().as_secs() < *max_sec,

            TerminationFunction::MultiCritAnd { criterion } => {
                for crit in criterion {
                    if !crit.keep_running() {
                        return false;
                    }
                }
                true
            }

            TerminationFunction::MultiCritOr { criterion } => {
                for crit in criterion {
                    if crit.keep_running() {
                        return true;
                    }
                }
                false
            }
            TerminationFunction::MustImprove { flipflop, .. } => *flipflop,

            TerminationFunction::NoImprove { flipflop, .. } => *flipflop,
        }
    }

    pub fn init(&mut self) {
        match self {
            TerminationFunction::AlwaysTrue {} | TerminationFunction::MinTemp { .. } => (),

            TerminationFunction::MaxIterations {
                current_iterations, ..
            } => *current_iterations = 0,

            TerminationFunction::MaxSec { time, .. } => *time = Instant::now(),

            TerminationFunction::MultiCritAnd { criterion }
            | TerminationFunction::MultiCritOr { criterion } => {
                for crit in criterion {
                    crit.init();
                }
            }

            TerminationFunction::MustImprove {
                best,
                flipflop,
                minimize,
            } => {
                if *minimize {
                    *best = f64::MAX
                } else {
                    *best = f64::MIN
                }
                *flipflop = true;
            }

            TerminationFunction::NoImprove {
                best,
                curr_without_improve,
                flipflop,
                minimize,
                ..
            } => {
                if *minimize {
                    *best = f64::MAX
                } else {
                    *best = f64::MIN
                }
                *flipflop = true;
                *curr_without_improve = 0;
            }
        }
    }

    pub fn check_variable(&self, var: isize) -> bool {
        match self {
            TerminationFunction::AlwaysTrue {}
            | TerminationFunction::MaxIterations { .. }
            | TerminationFunction::MaxSec { .. }
            | TerminationFunction::MustImprove { .. }
            | TerminationFunction::NoImprove { .. } => true,

            TerminationFunction::MinTemp { min_temp } => var > *min_temp,

            TerminationFunction::MultiCritAnd { criterion } => {
                for crit in criterion {
                    if !crit.check_variable(var) {
                        return false;
                    }
                }
                true
            }
            TerminationFunction::MultiCritOr { criterion } => {
                for crit in criterion {
                    if crit.check_variable(var) {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn check_new_variable(&mut self, var: f64) {
        match self {
            TerminationFunction::AlwaysTrue {}
            | TerminationFunction::MaxIterations { .. }
            | TerminationFunction::MaxSec { .. }
            | TerminationFunction::MinTemp { .. } => (),

            TerminationFunction::MultiCritAnd { criterion }
            | TerminationFunction::MultiCritOr { criterion } => {
                for crit in criterion {
                    crit.check_new_variable(var);
                }
            }

            TerminationFunction::MustImprove {
                best,
                flipflop,
                minimize,
            } => {
                if (*best <= var) == *minimize || (*best >= var) != *minimize {
                    *flipflop = false;
                } else {
                    *best = var;
                }
            }

            TerminationFunction::NoImprove {
                best,
                max_iterations_without_improve,
                curr_without_improve,
                flipflop,
                minimize,
            } => {
                if (*best < var) == *minimize || (*best >= var) != *minimize {
                    *curr_without_improve += 1;
                    if *curr_without_improve > *max_iterations_without_improve {
                        *flipflop = false;
                    }
                } else {
                    *best = var;
                }
            }
        }
    }

    pub fn iteration_done(&mut self) {
        match self {
            TerminationFunction::AlwaysTrue {}
            | TerminationFunction::MustImprove { .. }
            | TerminationFunction::NoImprove { .. }
            | TerminationFunction::MaxSec { .. }
            | TerminationFunction::MinTemp { .. } => (),

            TerminationFunction::MaxIterations {
                current_iterations, ..
            } => *current_iterations += 1,

            TerminationFunction::MultiCritAnd { criterion }
            | TerminationFunction::MultiCritOr { criterion } => {
                for crit in criterion {
                    crit.iteration_done();
                }
            }
        }
    }

    pub fn set_goal(&mut self, goal_minimize: bool) {
        match self {
            TerminationFunction::AlwaysTrue {} => todo!(),
            TerminationFunction::MaxIterations { .. } => todo!(),
            TerminationFunction::MaxSec { .. } => todo!(),
            TerminationFunction::MinTemp { .. } => todo!(),
            TerminationFunction::MultiCritAnd { criterion }
            | TerminationFunction::MultiCritOr { criterion } => {
                for crit in criterion {
                    crit.set_goal(goal_minimize)
                }
            }
            TerminationFunction::NoImprove { minimize, .. }
            | TerminationFunction::MustImprove { minimize, .. } => {
                *minimize = goal_minimize;
                self.init()
            }
        }
    }
}
