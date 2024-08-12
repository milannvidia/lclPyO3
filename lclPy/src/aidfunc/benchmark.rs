pub(crate) fn benchmark(
    problems: Vec<Arc<Mutex<dyn Problem>>>,
    algorithms: Vec<Arc<Mutex<dyn LocalSearch>>>,
    termination_functions: &Arc<Mutex<dyn TerminationFunction>>,
    runs: Option<usize>,
    seeds: Option<Vec<u64>>,
) {
    for algorithm in algorithm {
        for problem in problems {
            for i in seeds {}
        }
    }
}
