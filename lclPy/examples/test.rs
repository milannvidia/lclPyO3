// use lclpy::io::*;
// use lclpy::local_search::*;
// use lclpy::problem::*;
// use lclpy::termination::*;
// use rand::rngs::SmallRng;
// use rand::SeedableRng;

// use std::sync::{Arc, Mutex};
// fn main() {
//     let matrix = TspReader::DistanceMatrix {
//         file: "./data/usca312/usca312_dist.txt".to_owned(),
//     }
//     .get_distance_matrix()
//     .unwrap();
//     let prob = Evaluation::Tsp {
//         distance_matrix: matrix,
//         symmetric: true,
//     };
//     let move_type = MoveType::Swap {
//         rng: SmallRng::from_entropy(),
//         size: 48,
//     };
//     let problem: Arc<Mutex<dyn problem::Problem>> =
//         Arc::new(Mutex::new(ArrayProblem::new(&move_type, &prob)));
//     let termination: Arc<Mutex<dyn TerminationFunction>> = Arc::new(Mutex::new(MaxSec::new(600)));
//     let cooling: Arc<dyn simulated_annealing::CoolingFunction> =
//         Arc::new(simulated_annealing::GeometricCooling { alpha: 0.95 });
//     let iteration_calc: Arc<dyn simulated_annealing::IterationsTemperature> =
//         Arc::new(simulated_annealing::iter_temp::CnstIterTemp::new(1000000));

//     let mut lcl = simulated_annealing::SimulatedAnnealing::new(
//         2000,
//         true,
//         &problem,
//         &termination,
//         &cooling,
//         &iteration_calc,
//     );
//     println!("{}", problem.lock().unwrap().eval());
//     let res = lcl.run(true);
//     println!(
//         "{}, {}, {}, {}",
//         res[res.len() - 1].0,
//         res[res.len() - 1].1,
//         res[res.len() - 1].2,
//         res[res.len() - 1].3
//     )
// }
