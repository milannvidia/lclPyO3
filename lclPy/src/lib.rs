mod lcl_rust;
use lcl_rust::io::*;
use lcl_rust::local_search::simulated_annealing::*;
use lcl_rust::local_search::steepest_descent::*;
use lcl_rust::problem::*;
use lcl_rust::termination::*;
use must_improve::MustImprove;
use pyo3::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use std::{
    io::{self},
    sync::{Arc, Mutex},
};
#[pyclass(frozen, name = "MoveType")]
struct DynMoveType {
    mov: MoveType,
}

#[pyclass(frozen, name = "Termination")]
struct DynTermination {
    termination: Arc<Mutex<dyn TerminationFunction>>,
}
#[pyclass(frozen, name = "Problem")]
struct DynProblem {
    problem: Arc<Mutex<dyn Problem>>,
}
#[pyclass(frozen, name = "LocalSearch")]
struct DynLocalSearch {
    local_search: Arc<Mutex<dyn LocalSearch>>,
}
#[pyclass(frozen, name = "Cooling")]
struct DynCooling {
    cooling: Arc<dyn CoolingFunction>,
}
#[pyclass(frozen, name = "IterationsPerTemp")]
struct DynIterTemp {
    iter_temp: Arc<dyn IterationsTemperature>,
}

#[pyclass(frozen, name = "FileReader")]
struct DynIO {
    file: String,
    delimiter: char,
    read_matrix: Option<fn(&str, char) -> Result<Vec<Vec<usize>>, io::Error>>,
    read_row: Option<fn(&str, char) -> Vec<usize>>,
}

#[pymethods]
impl DynMoveType {
    #[staticmethod]
    fn swap(seed: Option<u64>) -> Self {
        let rng;
        if seed.is_some() {
            rng = SmallRng::seed_from_u64(seed.unwrap());
        } else {
            rng = SmallRng::from_entropy();
        }
        DynMoveType {
            mov: MoveType::Swap { rng, size: 50 },
        }
    }
    #[staticmethod]
    fn reverse(seed: Option<u64>) -> Self {
        let rng;
        if seed.is_some() {
            rng = SmallRng::seed_from_u64(seed.unwrap());
        } else {
            rng = SmallRng::from_entropy();
        }
        DynMoveType {
            mov: MoveType::Reverse { rng, size: 50 },
        }
    }
    #[staticmethod]
    fn swap_tsp(seed: Option<u64>) -> Self {
        let rng;
        if seed.is_some() {
            rng = SmallRng::seed_from_u64(seed.unwrap());
        } else {
            rng = SmallRng::from_entropy();
        }
        DynMoveType {
            mov: MoveType::Tsp { rng, size: 50 },
        }
    }
}

#[pymethods]
impl DynIO {
    #[staticmethod]
    fn csv(file: &str, delimiter: char) -> Self {
        DynIO {
            file: file.to_owned(),
            delimiter,
            read_matrix: Some(lcl_rust::io::read_csv),
            read_row: None,
        }
    }
}

#[pymethods]
impl DynLocalSearch {
    #[staticmethod]
    fn simulated_annealing(
        start_temp: usize,
        minimize: bool,
        problem: Py<DynProblem>,
        termination_function: Py<DynTermination>,
        cooling_function: Py<DynCooling>,
        iterations_temperature: Py<DynIterTemp>,
    ) -> PyResult<Self> {
        let problem = Arc::clone(&problem.get().problem);
        let termination = Arc::clone(&termination_function.get().termination);
        let cooling_func = Arc::clone(&cooling_function.get().cooling);
        let iter = Arc::clone(&iterations_temperature.get().iter_temp);
        let sim = SimulatedAnnealing::new(
            start_temp,
            minimize,
            problem,
            termination,
            cooling_func,
            iter,
        );
        Ok(DynLocalSearch {
            local_search: Arc::new(Mutex::new(sim)),
        })
    }
    #[staticmethod]
    fn steepest_descent(
        minimize: bool,
        problem: Py<DynProblem>,
        termination_function: Py<DynTermination>,
    ) -> PyResult<Self> {
        let problem = Arc::clone(&problem.get().problem);
        let termination = Arc::clone(&termination_function.get().termination);
        let sim = SteepestDescent::new(minimize, problem, termination);
        Ok(DynLocalSearch {
            local_search: Arc::new(Mutex::new(sim)),
        })
    }

    fn run(&self) -> Vec<(u128, isize, isize, usize)> {
        let mut x = self.local_search.lock().unwrap();
        return x.run(true);
    }
    fn reset(&self) {
        let mut x = self.local_search.lock().unwrap();
        x.reset();
    }
}

#[pymethods]
impl DynProblem {
    #[staticmethod]
    fn array_problem(move_type: Py<DynMoveType>) -> Self {}
    // #[staticmethod]
    // fn TSP(
    //     distance_matrix: Option<Vec<Vec<usize>>>,
    //     file_reader: Option<Py<DynIO>>,
    //     seed: Option<u64>,
    // ) -> Result<Self, io::Error> {
    //     let problem;
    //     if distance_matrix.is_some() {
    //         problem = TSP::new(false, distance_matrix.unwrap(), seed.or(None));
    //     } else if file_reader.is_some() {
    //         let dyn_io = file_reader.unwrap();
    //         let io_function = dyn_io.get().read_matrix;
    //         if io_function.is_none() {
    //             return Err(io::Error::other("This filereader is not meant for TSP"));
    //         }
    //         let matrix = io_function.unwrap()(dyn_io.get().file.as_str(), dyn_io.get().delimiter)?;
    //         problem = TSP::new(false, matrix, seed.or(None));
    //     } else {
    //         return Err(io::Error::other("matrix or file reader is needed"));
    //     }

    //     Ok(DynProblem {
    //         problem: Arc::new(Mutex::new(problem)),
    //     })
    // }
    // #[staticmethod]
    // fn bin_problem(weights: Vec<usize>, max_per_bin: usize) -> Result<Self, io::Error> {
    //     let problem = BinProblem::new(DeltaRating::ExponentialEmpty, max_per_bin, weights, None);
    //     Ok(DynProblem {
    //         problem: Arc::new(Mutex::new(problem)),
    //     })
    // }
}

#[pymethods]
impl DynCooling {
    #[staticmethod]
    fn geometric_cooling(alpha: f64) -> Self {
        DynCooling {
            cooling: Arc::new(GeometricCooling { alpha }),
        }
    }
}

#[pymethods]
impl DynIterTemp {
    #[staticmethod]
    fn cnst_iter_temp(iterations: usize) -> Self {
        DynIterTemp {
            iter_temp: Arc::new(CnstIterTemp { iterations }),
        }
    }
}

#[pymethods]
impl DynTermination {
    #[staticmethod]
    fn max_sec(max_sec: u64) -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(MaxSec::new(max_sec))),
        }
    }
    #[staticmethod]
    fn always_true_criterion() -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(AlwaysTrue::new())),
        }
    }
    #[staticmethod]
    fn max_iterations(max_iterations: usize) -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(MaxIterations::new(max_iterations))),
        }
    }
    #[staticmethod]
    fn min_temp(min_temp: isize) -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(MinTemp::new(min_temp))),
        }
    }
    #[staticmethod]
    fn MultiCritAnd(vec: Vec<Py<DynTermination>>) -> Self {
        let terminations = vec
            .iter()
            .map(|f| Arc::clone(&f.get().termination))
            .collect();
        DynTermination {
            termination: Arc::new(Mutex::new(MultiCritAnd::new(terminations))),
        }
    }
    #[staticmethod]
    fn MultiCritOr(vec: Vec<Py<DynTermination>>) -> Self {
        let terminations = vec
            .iter()
            .map(|f| Arc::clone(&f.get().termination))
            .collect();
        DynTermination {
            termination: Arc::new(Mutex::new(MultiCritOr::new(terminations))),
        }
    }
    #[staticmethod]
    fn must_improve() -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(MustImprove::new(true))),
        }
    }
    #[staticmethod]
    fn no_improve(iter_without_imp: usize) -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(NoImprove::new(true, iter_without_imp))),
        }
    }
}

#[pymodule]
fn lclRust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DynLocalSearch>()?;
    m.add_class::<DynProblem>()?;
    m.add_class::<DynTermination>()?;
    m.add_class::<DynIterTemp>()?;
    m.add_class::<DynCooling>()?;
    Ok(())
}
