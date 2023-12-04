mod lcl_rust;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;
use lcl_rust::problems;
use lcl_rust::terminationfunc;
use crate::lcl_rust::problems::Problem;
use crate::lcl_rust::simulated_annealing::SimulatedAnnealing;
use crate::lcl_rust::steepest_descent::SteepestDescent;


fn main() {
    let reader = BufReader::new(File::open("src/distanceMatrix").unwrap());

    let matrix: Vec<Vec<usize>> = reader.lines()
        .map(|l| l.unwrap().split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();
    let size=matrix.len();
    let mut problem =problems::TSP{
        swap: false,
        distance_matrix: matrix,
        solution: (0..size).collect(),
        // solution:vec![0,7,37,30,43,17,6,27,5,36,18,26,16,42,29,35,45,32,19,46,20,31,38,47,4,41,23,9,44,34,3,25,1,28,33,40,15,21,2,22,13,24,12,10,11,14,39,8],
        size,
        rng: rand::thread_rng(),
        best_solution: (0..size).collect(),
    };
    let mut termination=terminationfunc::MaxSec{
        time: Instant::now(), max_sec: 5
    };
    // let mut x =SimulatedAnnealing{
    //     problem: &mut problem,
    //     it_per_temp: 1500,
    //     temp: 2000,
    //     termination: &mut termination,
    // };
    let mut x=SteepestDescent{ problem: &mut problem, termination: &mut termination };


    let result=x.run(false);
    for values in result {
        println!("{}", values.0)
    }
    problem.solution=problem.best_solution.to_vec();


}