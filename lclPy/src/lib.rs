use pyo3::prelude::*;
mod lcl_rust;
use lcl_rust::problems::*;
use lcl_rust::terminationfunc::*;
use lcl_rust::LocalSearch;

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn lclRust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}

const local_search: Option<Box<dyn LocalSearch>> = None;
const problem: Option<Box<dyn Problem>> = None;
const termination: Option<Box<dyn TerminationFunction>> = None;

#[pyfunction]
fn setAlgorithm(a: String) {
    if(problem ==None ){
        
    }
    match a {
        "SimulatedAnnealing" => ,
        "SteepestDescent" =>,
        "TabuSearch" =>,
        "VNS"=>,
        _=>
    }
}
