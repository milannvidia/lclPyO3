use std::sync::{Arc, Mutex};

use crate::{LocalSearch, Problem, TerminationFunction};


/// Benchmark function
///
/// # Arguments
///
/// * `problems`: All problems to be tested
/// * `algorithms`: All algorithms to be tested
/// * `termination_function`: termination function to be used
/// * `runs`: how many runs, is used when seeds is None
/// * `seeds`: seeds to be used, also dictates amount of runs
///
/// returns: Vec<Vec<Vec<(u128, f64, f64, u64)>>>
///

pub fn benchmark(
    problems: Vec<Arc<Mutex<dyn Problem>>>,
    algorithms: Vec<Arc<Mutex<dyn LocalSearch>>>,
    termination_function: &TerminationFunction,
    runs: Option<u64>,
    seeds: Option<Vec<u64>>,
) -> Vec<Vec<Vec<(u128, f64, f64, u64)>>> {
    let seed_list: Vec<u64> = seeds.unwrap_or((0..runs.unwrap_or(10)).collect());

    let res: Vec<Vec<Vec<(u128, f64, f64, u64)>>> = Vec::new();
    for algorithm in &algorithms {
        let mut algo_res: Vec<Vec<(u128, f64, f64, u64)>> = Vec::new();
        let mut current_alg = algorithm.lock().unwrap();
        current_alg.set_termination(&termination_function);

        for problem in &problems {
            let mut problem_res: Vec<(u128, f64, f64, u64)> = Vec::new();

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
