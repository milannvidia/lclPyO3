use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
    string,
    time::Instant,
};

use lcl_rust::{
    io::read_csv,
    problems::{self, Problem, TSP},
    simulated_annealing,
    terminationfunc::{self, MaxSec, TerminationFunction},
    LocalSearch, SimulatedAnnealing,
};
use pyo3::prelude::*;
mod lcl_rust;

#[pymethods]
impl SimulatedAnnealing {
    #[new]
    fn new_py(problem: impl Problem) -> Result<Self, io::Error> {
        let distance_matrix = read_csv("src/distanceMatrix".to_owned(), b' ')?;
        let problem = TSP::new(false, distance_matrix);
        let terminationfunc = MaxSec::new(5);
        Ok(SimulatedAnnealing::new(
            2000,
            Box::new(problem),
            Box::new(terminationfunc),
            Box::new(simulated_annealing::cooling_func::GeometricCooling { alpha: 0.95 }),
            Box::new(simulated_annealing::iter_temp::CnstIterTemp { iterations: 1000 }),
        ))
    }

    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)> {
        (self as &mut dyn LocalSearch).run(log)
    }
    fn reset(&mut self) {
        (self as &mut dyn LocalSearch).reset()
    }
}

#[pymethods]
impl TSP {
    #[new]
    fn new_from_file(swap: bool, fileLocation: String, delimiter: char) -> Result<TSP, io::Error> {
        let matrix: Vec<Vec<usize>> = read_csv(fileLocation, delimiter)?;
        let problem = TSP::new(swap, matrix);
        Ok(problem)
    }
}
#[pymethods]
impl MaxSec {
    #[new]
    fn new_py(max_sec: u64) -> MaxSec {
        MaxSec::new(max_sec)
    }
}
/// A Python module implemented in Rust.
#[pymodule]
fn lclRust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SimulatedAnnealing>()?;
    m.add_class::<MaxSec>()?;
    m.add_class::<TSP>()?;
    Ok(())
}
