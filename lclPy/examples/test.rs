use std::sync::{Arc, Mutex};

use lcl_rust::*;
use local_search::simulated_annealing::*;
use rand::{rngs::SmallRng, SeedableRng};
fn main() {
    let matrix = io::TspReader::DistanceMatrix {
        file: "./data/usca312/usca312_dist.txt".to_owned(),
    }
    .get_distance_matrix()
    .unwrap();
    let prob = problem::Evaluation::Tsp {
        distance_matrix: matrix,
        symmetric: true,
    };
    let move_type = problem::MoveType::Swap {
        rng: SmallRng::from_entropy(),
        size: 48,
    };
    let problem: Arc<Mutex<dyn problem::Problem>> =
        Arc::new(Mutex::new(problem::ArrayProblem::new(&move_type, &prob)));
    let termination: Arc<Mutex<dyn termination::TerminationFunction>> =
        Arc::new(Mutex::new(termination::MaxSec::new(600)));
    let cooling: Arc<dyn local_search::simulated_annealing::CoolingFunction> =
        Arc::new(local_search::simulated_annealing::GeometricCooling { alpha: 0.95 });
    let iteration_calc: Arc<dyn local_search::simulated_annealing::IterationsTemperature> =
        Arc::new(local_search::simulated_annealing::iter_temp::CnstIterTemp::new(1000000));

    let mut lcl = local_search::simulated_annealing::SimulatedAnnealing::new(
        2000,
        true,
        &problem,
        &termination,
        &cooling,
        &iteration_calc,
    );
    println!("{}", problem.lock().unwrap().eval());
    let res = lcl.run(true);
    println!(
        "{}, {}, {}, {}",
        res[res.len() - 1].0,
        res[res.len() - 1].1,
        res[res.len() - 1].2,
        res[res.len() - 1].3
    )
}
