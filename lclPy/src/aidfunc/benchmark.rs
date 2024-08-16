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

    let mut res: Vec<Vec<Vec<(u128, f64, f64, u64)>>> = Vec::new();
    for algorithm in &algorithms {
        algorithm
            .lock()
            .unwrap()
            .set_termination(&termination_function);

        let mut algo_res: Vec<Vec<(u128, f64, f64, u64)>> = Vec::new();
        for problem in &problems {
            algorithm.lock().unwrap().set_problem(problem);
            let mut problem_res: Vec<(u128, f64, f64, u64)> = Vec::new();
            for i in &seed_list {
                problem.lock().unwrap().set_seed(*i);
                problem.lock().unwrap().reset();
                let mut res = algorithm.lock().unwrap().run(false);
                assert_eq!(problem.lock().unwrap().eval(), res.last().unwrap().2);
                problem_res.push(res.pop().unwrap());
            }
            algo_res.push(problem_res);
        }
        res.push(algo_res)
    }
    res
}
