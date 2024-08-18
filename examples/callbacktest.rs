use std::sync::{Arc, Mutex};

use lclPyO3::{
    aidfunc::read_distance_matrix,
    local_search::{
        simulated_annealing::{CoolingFunction, IterationsTemperature},
        SimulatedAnnealing,
    },
    problem::{ArrayProblem, Evaluation, MoveType, Problem},
    termination::TerminationFunction,
};

fn main() {
    let cooling = CoolingFunction::geometric_cooling(0.95);
    let iteration_calc = IterationsTemperature::const_iter_temp(1000);
    let termination = TerminationFunction::max_sec(5);
    let move_type = MoveType::tsp(None);
    let distance_matrix = read_distance_matrix("data/distanceMatrix");
    let evaluation = Evaluation::tsp(distance_matrix.unwrap());
    let problem: Arc<Mutex<dyn Problem>> =
        Arc::new(Mutex::new(ArrayProblem::new(&move_type, &evaluation)));
    let sim = Arc::new(Mutex::new(SimulatedAnnealing::new(
        2000,
        true,
        &problem,
        &termination,
        &cooling,
        &iteration_calc,
    )));
    let res =
        lclPyO3::aidfunc::benchmark(vec![sim], vec![problem], Some(termination), Some(1), None);
    println!("{:?}", res);
}
