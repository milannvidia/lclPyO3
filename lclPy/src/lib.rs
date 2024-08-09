use pyo3::{exceptions::PyValueError, ffi::PyErr_BadArgument, prelude::*};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use simulated_annealing::{CoolingFunction, IterationsTemperature, SimulatedAnnealing};
use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
};
use steepest_descent::SteepestDescent;
use tabu_search::TabuSearch;
pub mod io;
pub mod local_search;
pub mod problem;
pub mod termination;
use local_search::*;
use problem::*;
use termination::*;
// #[pyclass(frozen, name = "TspReader")]
// struct DynTspReader {
//     read: TspReader,
// }

#[pyclass(frozen, name = "RustMoveType")]
struct DynMoveType {
    mov: MoveType,
}

#[pyclass(frozen, name = "RustEvaluation")]
struct DynEvaluation {
    eva: Evaluation,
}

#[pyclass(frozen, name = "RustTermination")]
struct DynTermination {
    termination: Arc<Mutex<dyn TerminationFunction>>,
}

#[pyclass(frozen, name = "RustProblem")]
struct DynProblem {
    problem: Arc<Mutex<dyn Problem>>,
}

#[pyclass(frozen, name = "RustLocalSearch")]
struct DynLocalSearch {
    local_search: Arc<Mutex<dyn LocalSearch>>,
}

#[pyclass(frozen, name = "RustCooling")]
struct DynCooling {
    cooling: CoolingFunction,
}

#[pyclass(frozen, name = "RustIterationsPerTemp")]
struct DynIterTemp {
    iter_temp: IterationsTemperature,
}
#[pymethods]
impl DynEvaluation {
    #[staticmethod]
    fn empty_bins(weights: Vec<usize>, max_fill: usize) -> Self {
        DynEvaluation {
            eva: Evaluation::EmptyBins { weights, max_fill },
        }
    }
    #[staticmethod]
    fn empty_space(weights: Vec<usize>, max_fill: usize) -> Self {
        DynEvaluation {
            eva: Evaluation::EmptySpace { weights, max_fill },
        }
    }
    #[staticmethod]
    fn empty_space_exp(weights: Vec<usize>, max_fill: usize) -> Self {
        DynEvaluation {
            eva: Evaluation::EmptySpaceExp { weights, max_fill },
        }
    }
    #[staticmethod]
    fn TSP(distance_matrix: Vec<Vec<usize>>) -> Self {
        DynEvaluation {
            eva: Evaluation::Tsp {
                distance_matrix,
                symmetric: true,
            },
        }
    }
    #[staticmethod]
    fn QAP(distance_matrix: Vec<Vec<usize>>, flow_matrix: Vec<Vec<usize>>) -> Self {
        DynEvaluation {
            eva: Evaluation::QAP {
                distance_matrix,
                flow_matrix,
            },
        }
    }
    #[staticmethod]
    fn tsp_from_dist_matrix(file: &str) -> PyResult<Self> {
        let distance_matrix = io::TspReader::DistanceMatrix {
            file: file.to_owned(),
        }
        .get_distance_matrix()?;
        let mut symmetric = true;
        'outer: for i in 0..distance_matrix.len() {
            for j in 0..i {
                if distance_matrix[i][j] != distance_matrix[j][i] {
                    symmetric = false;
                    break 'outer;
                }
            }
        }
        Ok(DynEvaluation {
            eva: Evaluation::Tsp {
                distance_matrix,
                symmetric,
            },
        })
    }
}

#[pymethods]
impl DynMoveType {
    #[staticmethod]
    #[pyo3(signature = (size, seed=None))]
    fn swap(size: usize, seed: Option<u64>) -> Self {
        let rng;
        if seed.is_some() {
            rng = SmallRng::seed_from_u64(seed.unwrap());
        } else {
            rng = SmallRng::from_entropy();
        }
        DynMoveType {
            mov: MoveType::Swap { rng, size },
        }
    }
    #[staticmethod]
    #[pyo3(signature = (size, seed=None))]
    fn reverse(size: usize, seed: Option<u64>) -> Self {
        let rng;
        if seed.is_some() {
            rng = SmallRng::seed_from_u64(seed.unwrap());
        } else {
            rng = SmallRng::from_entropy();
        }
        DynMoveType {
            mov: MoveType::Reverse { rng, size },
        }
    }
    #[staticmethod]
    #[pyo3(signature = (size, seed=None))]
    fn swap_tsp(size: usize, seed: Option<u64>) -> Self {
        let rng;
        if seed.is_some() {
            rng = SmallRng::seed_from_u64(seed.unwrap());
        } else {
            rng = SmallRng::from_entropy();
        }
        DynMoveType {
            mov: MoveType::Tsp { rng, size },
        }
    }
    #[staticmethod]
    #[pyo3(signature = (move_array, weights=None))]
    fn multi_neighbor(
        move_array: Vec<Py<DynMoveType>>,
        weights: Option<Vec<f64>>,
    ) -> Result<Self, PyErr> {
        let mut move_types: Vec<MoveType> = vec![];
        for mov in move_array {
            let cloned_mov = mov.get().mov.clone();
            if let MoveType::MultiNeighbor { .. } = cloned_mov {
                return Err(PyErr::new::<PyValueError, _>(
                    "Can't have multi neighbor in multineighbor",
                ));
            }
            move_types.push(cloned_mov);
        }
        let res_weights: Vec<f64>;
        if weights.is_some() {
            let sum: f64 = weights.as_ref().unwrap().iter().sum();
            if (sum - 1.0).abs() > 1e-9 {
                return Err(PyErr::new::<PyValueError, _>("weights don't add to 1"));
            }
            res_weights = weights.unwrap();
        } else {
            res_weights = vec![1.0f64 / (move_types.len() as f64); move_types.len()];
        }
        Ok(DynMoveType {
            mov: MoveType::MultiNeighbor {
                move_types,
                weights: res_weights,
            },
        })
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
        let sim = SimulatedAnnealing::new(
            start_temp,
            minimize,
            &problem.get().problem,
            &termination_function.get().termination,
            &cooling_function.get().cooling,
            &iterations_temperature.get().iter_temp,
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
        let sim = SteepestDescent::new(
            minimize,
            &problem.get().problem,
            &termination_function.get().termination,
        );
        Ok(DynLocalSearch {
            local_search: Arc::new(Mutex::new(sim)),
        })
    }
    #[staticmethod]
    fn tabu_search(
        minimize: bool,
        problem: Py<DynProblem>,
        termination_function: Py<DynTermination>,
    ) -> PyResult<Self> {
        let sim = TabuSearch::new(
            &problem.get().problem,
            &termination_function.get().termination,
            minimize,
        );
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
    fn array_problem(move_type: Py<DynMoveType>, evaluation: Py<DynEvaluation>) -> Self {
        let move_enum = &move_type.get().mov;
        let eva = &evaluation.get().eva;
        DynProblem {
            problem: Arc::new(Mutex::new(ArrayProblem::new(move_enum, eva))),
        }
    }
}

#[pymethods]
impl DynCooling {
    #[staticmethod]
    fn geometric_cooling(alpha: f64) -> Self {
        DynCooling {
            cooling: CoolingFunction::GeometricCooling { alpha },
        }
    }
}

#[pymethods]
impl DynIterTemp {
    #[staticmethod]
    fn cnst_iter_temp(iterations: usize) -> Self {
        DynIterTemp {
            iter_temp: IterationsTemperature::ConstIterTemp { iterations },
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
    fn multi_crit_and(vec: Vec<Py<DynTermination>>) -> Self {
        let terminations = vec
            .iter()
            .map(|f| Arc::clone(&f.get().termination))
            .collect();
        DynTermination {
            termination: Arc::new(Mutex::new(MultiCritAnd::new(terminations))),
        }
    }
    #[staticmethod]
    fn multi_crit_or(vec: Vec<Py<DynTermination>>) -> Self {
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
fn lclpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DynLocalSearch>()?;
    m.add_class::<DynProblem>()?;
    m.add_class::<DynTermination>()?;
    m.add_class::<DynIterTemp>()?;
    m.add_class::<DynCooling>()?;
    m.add_class::<DynEvaluation>()?;
    m.add_class::<DynMoveType>()?;
    Ok(())
}
