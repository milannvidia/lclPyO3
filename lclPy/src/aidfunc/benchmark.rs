use std::sync::{Arc, Mutex};

use crate::{LocalSearch, Problem, TerminationFunction};

///
///
/// # Arguments
///
/// * `problems`: A vector of Arc<Mutex<dyn Problem>>
/// * `algorithms`: A vector of Arc<Mutex<dyn LocalSearch>>
/// * `termination_functions`: A Arc<Mutex<dyn TerminationFunction>>
/// * `runs`: #runs if seeds isn't specified runs dictates the amount of runs
/// * `seeds`:  If a vector of seeds is passed this dictates the amount of runs
///
/// returns: ()
///

pub fn benchmark(
    problems: Vec<Arc<Mutex<dyn Problem>>>,
    algorithms: Vec<Arc<Mutex<dyn LocalSearch>>>,
    termination_function: &TerminationFunction,
    runs: Option<u64>,
    seeds: Option<Vec<u64>>,
) -> Vec<Vec<Vec<(u128, f64, f64, usize)>>> {
    let seed_list: Vec<u64> = seeds
        .or(Some((0..(runs.or(Some(10)).unwrap())).collect()))
        .unwrap();

    let res: Vec<Vec<Vec<(u128, f64, f64, usize)>>> = Vec::new();
    for algorithm in &algorithms {
        let mut algo_res: Vec<Vec<(u128, f64, f64, usize)>> = Vec::new();
        let mut current_alg = algorithm.lock().unwrap();
        current_alg.set_termination(&termination_function);

        for problem in &problems {
            let mut problem_res: Vec<(u128, f64, f64, usize)> = Vec::new();

            current_alg.set_problem(problem);

            for i in &seed_list {
                problem.lock().unwrap().set_seed(*i);
                problem_res.push(current_alg.run(false).pop().unwrap());
            }
            algo_res.push(problem_res);
        }
    }
    res
}
