#![allow(non_snake_case)]
use local_search::*;
use problem::*;
use pyo3::{exceptions::PyValueError, prelude::*};

use simulated_annealing::{CoolingFunction, IterationsTemperature, SimulatedAnnealing};
use std::sync::{Arc, Mutex};
use steepest_descent::SteepestDescent;
use tabu_search::TabuSearch;
use termination::*;
use vns::VariableNeighborhood;
pub mod aidfunc;
pub mod local_search;
pub mod problem;
pub mod termination;
// #[pyclass(frozen, name = "TspReader")]
// struct DynTspReader {
//     read: TspReader,
// }

// ====================================================================================================================================================================
// Functions
// ====================================================================================================================================================================

#[pyfunction]
#[pyo3(signature=(algorithms,problems,termination_function=None,runs=None,seeds=None))]
fn benchmark(
    algorithms: Vec<Py<DynLocalSearch>>,
    problems: Vec<Py<DynProblem>>,
    termination_function: Option<Py<DynTermination>>,
    runs: Option<u64>,
    seeds: Option<Vec<u64>>,
) -> Vec<Vec<Vec<Vec<(u128, f64, f64, u64)>>>> {
    println!("hii");
    let r_problems: Vec<Arc<Mutex<dyn Problem>>> =
        problems.iter().map(|f| f.get().problem.clone()).collect();
    let r_algorithms: Vec<Arc<Mutex<dyn LocalSearch>>> = algorithms
        .iter()
        .map(|f| f.get().local_search.clone())
        .collect();

    let r_term: Option<TerminationFunction> = if termination_function.is_some() {
        Some(termination_function.unwrap().get().termination.clone())
    } else {
        None
    };
    aidfunc::benchmark(r_algorithms, r_problems, r_term, runs, seeds)
}

// ====================================================================================================================================================================
// Classes
// ====================================================================================================================================================================
#[pyclass(frozen, name = "MoveType")]
struct DynMoveType {
    mov: MoveType,
}
#[pyclass(frozen, name = "Evaluation")]
struct DynEvaluation {
    eva: Evaluation,
}
#[pyclass(frozen, name = "Termination")]
struct DynTermination {
    termination: TerminationFunction,
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
    cooling: CoolingFunction,
}

#[pyclass(frozen, name = "IterationsPerTemp")]
struct DynIterTemp {
    iter_temp: IterationsTemperature,
}

// ====================================================================================================================================================================
// Methods
// ====================================================================================================================================================================

#[pymethods]
impl DynEvaluation {
    #[staticmethod]
    fn empty_bins(weights: Vec<f64>, max_fill: f64) -> Self {
        DynEvaluation {
            eva: Evaluation::bins(weights, max_fill),
        }
    }
    #[staticmethod]
    fn empty_space(weights: Vec<f64>, max_fill: f64) -> Self {
        DynEvaluation {
            eva: Evaluation::empty_space(weights, max_fill),
        }
    }
    #[staticmethod]
    fn empty_space_exp(weights: Vec<f64>, max_fill: f64) -> Self {
        DynEvaluation {
            eva: Evaluation::empty_space_exp(weights, max_fill),
        }
    }
    #[staticmethod]
    fn tsp(distance_matrix: Vec<Vec<f64>>) -> Self {
        DynEvaluation {
            eva: Evaluation::tsp(distance_matrix),
        }
    }
    #[staticmethod]
    fn qap(distance_matrix: Vec<Vec<f64>>, flow_matrix: Vec<Vec<f64>>) -> Self {
        DynEvaluation {
            eva: Evaluation::qap(distance_matrix, flow_matrix),
        }
    }
    #[staticmethod]
    fn tsp_from_dist_matrix(file: &str) -> PyResult<Self> {
        let distance_matrix = aidfunc::io::read_distance_matrix(file)?;
        Ok(DynEvaluation {
            eva: Evaluation::tsp(distance_matrix),
        })
    }
    #[staticmethod]
    fn tsp_from_coord2d(file: &str) -> PyResult<Self> {
        let distance_matrix = aidfunc::io::read_coord2d_to_distance_matrix(file)?;
        Ok(DynEvaluation {
            eva: Evaluation::tsp(distance_matrix),
        })
    }
    #[staticmethod]
    fn tsp_from_dms(file: &str) -> PyResult<Self> {
        let distance_matrix = aidfunc::io::read_dms_to_distance_matrix(file)?;
        Ok(DynEvaluation {
            eva: Evaluation::tsp(distance_matrix),
        })
    }
}

#[pymethods]
impl DynMoveType {
    #[staticmethod]
    #[pyo3(signature = (seed=None))]
    fn swap(seed: Option<u64>) -> Self {
        DynMoveType {
            mov: MoveType::swap(seed),
        }
    }
    #[staticmethod]
    #[pyo3(signature = (seed=None))]
    fn reverse(seed: Option<u64>) -> Self {
        DynMoveType {
            mov: MoveType::reverse(seed),
        }
    }
    #[staticmethod]
    #[pyo3(signature = ( seed=None))]
    fn swap_tsp(seed: Option<u64>) -> Self {
        DynMoveType {
            mov: MoveType::tsp(seed),
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
                    "Can't have multi neighbor in multiNeighbor",
                ));
            }
            move_types.push(cloned_mov);
        }
        Ok(DynMoveType {
            mov: MoveType::multi_neighbor(move_types, weights),
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
    #[pyo3(signature = (minimize, problem,termination_function,tabu_list_size=None))]
    fn tabu_search(
        minimize: bool,
        problem: Py<DynProblem>,
        termination_function: Py<DynTermination>,
        tabu_list_size: Option<usize>,
    ) -> PyResult<Self> {
        let sim = TabuSearch::new(
            &problem.get().problem,
            &termination_function.get().termination,
            minimize,
            tabu_list_size,
        );
        Ok(DynLocalSearch {
            local_search: Arc::new(Mutex::new(sim)),
        })
    }

    #[staticmethod]
    fn vns(
        minimize: bool,
        problem: Py<DynProblem>,
        termination_function: Py<DynTermination>,
    ) -> PyResult<Self> {
        let sim = VariableNeighborhood::new(
            &problem.get().problem,
            &termination_function.get().termination,
            minimize,
        );
        Ok(DynLocalSearch {
            local_search: Arc::new(Mutex::new(sim)),
        })
    }

    fn run(&self) -> Vec<(u128, f64, f64, u64)> {
        let mut x = self.local_search.lock().unwrap();
        x.run(true)
    }

    fn reset(&self) {
        let mut x = self.local_search.lock().unwrap();
        x.reset();
    }

    fn set_problem(&self, problem: Py<DynProblem>) {
        self.local_search
            .lock()
            .unwrap()
            .set_problem(&problem.get().problem.clone())
    }

    fn set_termination(&self, termination_function: Py<DynTermination>) {
        self.local_search
            .lock()
            .unwrap()
            .set_termination(&termination_function.get().termination.clone())
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

    fn set_eval_type(&self, eval_type: Py<DynEvaluation>) {
        self.problem
            .lock()
            .unwrap()
            .set_eval_type(eval_type.get().eva.clone());
    }

    fn set_move_type(&self, move_type: Py<DynMoveType>) {
        self.problem
            .lock()
            .unwrap()
            .set_move_type(move_type.get().mov.clone());
    }

    fn reset(&self) {
        self.problem.lock().unwrap().reset();
    }

    fn eval(&self) -> f64 {
        self.problem.lock().unwrap().eval()
    }

    fn set_seed(&self, seed: u64) {
        self.problem.lock().unwrap().set_seed(seed);
    }
}

#[pymethods]
impl DynCooling {
    #[staticmethod]
    fn geometric_cooling(alpha: f64) -> Self {
        DynCooling {
            cooling: CoolingFunction::geometric_cooling(alpha),
        }
    }
}

#[pymethods]
impl DynIterTemp {
    #[staticmethod]
    fn cnst_iter_temp(iterations: usize) -> Self {
        DynIterTemp {
            iter_temp: IterationsTemperature::const_iter_temp(iterations),
        }
    }
}

#[pymethods]
impl DynTermination {
    #[staticmethod]
    fn max_sec(max_sec: u64) -> Self {
        DynTermination {
            termination: TerminationFunction::max_sec(max_sec),
        }
    }
    #[staticmethod]
    fn always_true() -> Self {
        DynTermination {
            termination: TerminationFunction::always_true(),
        }
    }
    #[staticmethod]
    fn max_iterations(max_iterations: usize) -> Self {
        DynTermination {
            termination: TerminationFunction::max_iterations(max_iterations),
        }
    }
    #[staticmethod]
    fn min_temp(min_temp: isize) -> Self {
        DynTermination {
            termination: TerminationFunction::min_temp(min_temp),
        }
    }
    #[staticmethod]
    fn multi_crit_and(vec: Vec<Py<DynTermination>>) -> Self {
        let terminations = vec.iter().map(|f| f.get().termination.clone()).collect();
        DynTermination {
            termination: TerminationFunction::multi_crit_and(terminations),
        }
    }
    #[staticmethod]
    fn multi_crit_or(vec: Vec<Py<DynTermination>>) -> Self {
        let terminations = vec.iter().map(|f| f.get().termination.clone()).collect();
        DynTermination {
            termination: TerminationFunction::multi_crit_or(terminations),
        }
    }
    #[staticmethod]
    fn must_improve() -> Self {
        DynTermination {
            termination: TerminationFunction::must_improve(),
        }
    }
    #[staticmethod]
    fn no_improve(iter_without_imp: usize) -> Self {
        DynTermination {
            termination: TerminationFunction::no_improve(iter_without_imp),
        }
    }
}

#[pymodule]
fn lclPyO3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DynLocalSearch>()?;
    m.add_class::<DynProblem>()?;
    m.add_class::<DynTermination>()?;
    m.add_class::<DynIterTemp>()?;
    m.add_class::<DynCooling>()?;
    m.add_class::<DynEvaluation>()?;
    m.add_class::<DynMoveType>()?;
    m.add_function(wrap_pyfunction!(benchmark, m)?)?;
    Ok(())
}
