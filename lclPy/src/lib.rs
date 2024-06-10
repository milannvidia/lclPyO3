use lcl_rust::{
    io::read_csv,
    problems::{BinProblem, DeltaRating, Problem, TSP},
    simulated_annealing::{
        cooling_func::{CoolingFunction, GeometricCooling},
        iter_temp::{CnstIterTemp, IterationsTemperature},
    },
    terminationfunc::{
        AlwaysTrueCriterion, MaxIterations, MaxSec, MinTemp, MustImprove, NoImprove,
        TerminationFunction,
    },
    LocalSearch, SimulatedAnnealing, SteepestDescent,
};
use std::{
    io::{self},
    sync::{Arc, Mutex},
};

use pyo3::prelude::*;
mod lcl_rust;

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

#[pymethods]
impl DynLocalSearch {
    #[staticmethod]
    fn simulated_annealing(
        problem: Py<DynProblem>,
        termination_function: Py<DynTermination>,
        cooling_function: Py<DynCooling>,
        iterations_temperature: Py<DynIterTemp>,
    ) -> PyResult<Self> {
        let problem = Arc::clone(&problem.get().problem);
        let termination = Arc::clone(&termination_function.get().termination);
        let cooling_func = Arc::clone(&cooling_function.get().cooling);
        let iter = Arc::clone(&iterations_temperature.get().iter_temp);

        let sim = SimulatedAnnealing::new(2000, problem, termination, cooling_func, iter);
        Ok(DynLocalSearch {
            local_search: Arc::new(Mutex::new(sim)),
        })
    }
    #[staticmethod]
    fn steepest_descent(
        problem: Py<DynProblem>,
        termination_function: Py<DynTermination>,
    ) -> PyResult<Self> {
        let problem = Arc::clone(&problem.get().problem);
        let termination = Arc::clone(&termination_function.get().termination);
        let sim = SteepestDescent::new(problem, termination);
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
    fn TSP(distance_matrix: Vec<Vec<usize>>) -> Result<Self, io::Error> {
        let distance_matrix = read_csv("src/distanceMatrix", ' ')?;
        let problem = TSP::new(false, distance_matrix, None);
        Ok(DynProblem {
            problem: Arc::new(Mutex::new(problem)),
        })
    }
    #[staticmethod]
    fn bin_problem(weights: Vec<usize>, max_per_bin: usize) -> Result<Self, io::Error> {
        let problem = BinProblem::new(DeltaRating::ExponentialEmpty, max_per_bin, weights, None);
        Ok(DynProblem {
            problem: Arc::new(Mutex::new(problem)),
        })
    }
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
            termination: Arc::new(Mutex::new(AlwaysTrueCriterion {})),
        }
    }
    #[staticmethod]
    fn max_iterations(max_iterations: usize) -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(MaxIterations::new(max_iterations))),
        }
    }
    #[staticmethod]
    fn min_temp(min_temp: usize) -> Self {
        DynTermination {
            termination: Arc::new(Mutex::new(MinTemp::new(min_temp))),
        }
    }
    // #[staticmethod]
    // fn MultiCritAnd(a: Py<DynTermination>, b: Py<DynTermination>) -> Self {
    //     let first = Arc::clone(&a.get().termination);
    //     let sec = Arc::clone(&b.get().termination);
    //     todo!();
    // }
    // #[staticmethod]
    // fn MultiCritOr(a: Py<DynTermination>, b: Py<DynTermination>) -> Self {
    //     let first = Arc::clone(&a.get().termination);
    //     let sec = Arc::clone(&b.get().termination);
    //     todo!();
    // }
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
