use pyo3::prelude::*;
use std::{time::Instant, usize::MAX};
pub trait TerminationFunction {
    fn keep_running(&mut self) -> bool;
    fn init(&mut self);
    fn check_variable(&mut self, var: usize) -> bool;
}
#[pyclass(unsendable)]
pub struct MaxSec {
    pub time: Instant,
    pub max_sec: u64,
}
impl MaxSec {
    pub fn new(max_sec: u64) -> Self {
        MaxSec {
            time: Instant::now(),
            max_sec,
        }
    }
}
impl TerminationFunction for MaxSec {
    fn keep_running(&mut self) -> bool {
        return if self.time.elapsed().as_secs() < self.max_sec {
            true
        } else {
            false
        };
    }

    fn init(&mut self) {
        self.time = Instant::now();
    }

    fn check_variable(&mut self, _var: usize) -> bool {
        true
    }
}

pub struct AlwaysTrueCriterion {}

impl TerminationFunction for AlwaysTrueCriterion {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, _var: usize) -> bool {
        true
    }
}

pub struct MaxIterations {
    pub max_iterations: u64,
    current_iterations: u64,
}

impl TerminationFunction for MaxIterations {
    fn keep_running(&mut self) -> bool {
        if self.current_iterations < self.max_iterations {
            self.current_iterations += 1;
            true
        } else {
            false
        }
    }
    fn init(&mut self) {
        self.current_iterations = 0;
    }

    fn check_variable(&mut self, _var: usize) -> bool {
        true
    }
}

// only to be used in simmulated annealing
pub struct MinTemp {
    pub min_temp: usize,
}

impl TerminationFunction for MinTemp {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, var: usize) -> bool {
        var > self.min_temp
    }
}

pub struct MultiCritAnd {
    pub critirions: Vec<Box<dyn TerminationFunction>>,
}

impl TerminationFunction for MultiCritAnd {
    fn keep_running(&mut self) -> bool {
        for crit in &mut self.critirions {
            if !crit.keep_running() {
                return false;
            }
        }
        true
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, var: usize) -> bool {
        for crit in &mut self.critirions {
            if !crit.check_variable(var) {
                return false;
            }
        }
        true
    }
}

pub struct MultiCritOr {
    pub critirions: Vec<Box<dyn TerminationFunction>>,
}

impl TerminationFunction for MultiCritOr {
    fn keep_running(&mut self) -> bool {
        for crit in &mut self.critirions {
            if crit.keep_running() {
                return true;
            }
        }
        false
    }
    fn init(&mut self) {}

    fn check_variable(&mut self, var: usize) -> bool {
        for crit in &mut self.critirions {
            if crit.check_variable(var) {
                return true;
            }
        }
        false
    }
}

pub struct MustImprove {
    pub best: usize,
}

impl TerminationFunction for MustImprove {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {
        self.best = MAX;
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

pub struct NoImprove {
    pub best: usize,
    pub max_iterations_without_improve: usize,
    curr_without_improve: usize,
}

impl TerminationFunction for NoImprove {
    fn keep_running(&mut self) -> bool {
        true
    }
    fn init(&mut self) {
        self.best = MAX;
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
