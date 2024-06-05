use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use lcl_rust::{
    problems::{self, TSP},
    simulated_annealing, terminationfunc, LocalSearch, SimulatedAnnealing,
};
use pyo3::prelude::*;
mod lcl_rust;

#[pymethods]
impl SimulatedAnnealing {
    #[new]
    fn new_py() -> Self {
        let reader = BufReader::new(File::open("src/distanceMatrix").unwrap());

        let matrix: Vec<Vec<usize>> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect()
            })
            .collect();
        let size = matrix.len();
        let problem = problems::TSP {
            swap: false,
            distance_matrix: matrix,
            solution: (0..size).collect(),
            // solution:vec![0,7,37,30,43,17,6,27,5,36,18,26,16,42,29,35,45,32,19,46,20,31,38,47,4,41,23,9,44,34,3,25,1,28,33,40,15,21,2,22,13,24,12,10,11,14,39,8],
            size,
            rng: rand::thread_rng(),
            best_solution: (0..size).collect(),
        };
        let termination = terminationfunc::MaxSec {
            time: Instant::now(),
            max_sec: 5,
        };
        SimulatedAnnealing::new(
            2000,
            Box::new(problem),
            Box::new(termination),
            Box::new(simulated_annealing::cooling_func::GeometricCooling { alpha: 0.95 }),
            Box::new(simulated_annealing::iter_temp::CnstIterTemp { iterations: 1000 }),
        )
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
    fn new_py() -> Self {}
}

/// A Python module implemented in Rust.
#[pymodule]
fn lclRust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SimulatedAnnealing>()?;
    Ok(())
}
