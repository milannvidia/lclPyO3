use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use rand::Rng;


fn main() {
    let reader = BufReader::new(File::open("src/distanceMatrix").unwrap());

    let matrix: Vec<Vec<usize>> = reader.lines()
        .map(|l| l.unwrap().split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();

    simulated_annealing(&matrix);
}

fn simulated_annealing(distance_matrix: &Vec<Vec<usize>>) {
    let e=std::f64::consts::E;
    let mut rng = rand::thread_rng();
    let mut data:Vec<(u128,isize)>=vec![];
    let mut temperature:usize = 2000;
    let mut iterations = 0;
    let now = Instant::now();

    let mut current_solution: Vec<usize> = (0..distance_matrix.len()).collect();
    let mut base_value:isize = tsp_eval(&distance_matrix, &current_solution) as isize;
    let mut best = base_value;
    let mut i;
    let mut j;


    while now.elapsed().as_secs() < 5 {
    // while iterations < 5000000 {
        for _ in 0..1500 {
            if now.elapsed().as_secs() > 5 {
                break;
            }
            //two indexes
            i = rng.gen_range(1..distance_matrix.len());
            j = rng.gen_range(1..distance_matrix.len());
            while i == j {
                j = rng.gen_range(1..distance_matrix.len());
            }
            if j < i {
                std::mem::swap(&mut i, &mut j)
            }
            let delta = delta_tsp(&distance_matrix, i, j, &mut current_solution);

            if delta <= 0 {
                current_solution.swap(i, j);
                base_value += delta;

                if base_value < best {
                    best = base_value;
                    println!("{} {}",best,now.elapsed().as_millis())
                }
                let mut x =vec![(now.elapsed().as_nanos(), base_value)];
                data.append(&mut x);
            } else {
                let exp:f64= -(delta  as f64)/ temperature as f64;
                let propability: f64 = e.powf(exp);
                let randonm: f64 = rng.gen();
                if propability > randonm {
                    current_solution.swap(i, j);
                    base_value += delta;
                }
            }
            iterations += 1;
        }
        temperature = (temperature as f64 * 0.95) as usize;
    }
    let score=tsp_eval(distance_matrix, &current_solution);
    println!("{}",score)
}

fn delta_tsp(distance_matrix: &Vec<Vec<usize>>, index0: usize, index1: usize, current_solution: &mut Vec<usize>) -> isize {
    let mut initialscore:isize = 0;
    let mut nextscore:isize = 0;


    let indexsafe = (index1 + 1) % distance_matrix.len();

    initialscore += distance_matrix[current_solution[index0 - 1]][current_solution[index0]] as isize;
    initialscore += distance_matrix[current_solution[index0]][current_solution[index0 + 1]] as isize;
    initialscore += distance_matrix[current_solution[index1 - 1]][current_solution[index1]] as isize;
    initialscore += distance_matrix[current_solution[index1]][current_solution[indexsafe]] as isize;

    current_solution.swap(index0, index1);

    nextscore += distance_matrix[current_solution[index0 - 1]][current_solution[index0]] as isize;
    nextscore += distance_matrix[current_solution[index0]][current_solution[index0 + 1]] as isize;
    nextscore += distance_matrix[current_solution[index1 - 1]][current_solution[index1]] as isize;
    nextscore += distance_matrix[current_solution[index1]][current_solution[indexsafe]] as isize;

    current_solution.swap(index0, index1);


    return (nextscore - initialscore) as isize;
}


fn tsp_eval(distance_matrix: &Vec<Vec<usize>>, current_solution: &Vec<usize>) -> usize {
    let mut score:usize = 0;
    for i in 1..current_solution.len() {
        score += distance_matrix[current_solution[i - 1]][current_solution[i]];
    }
    return score ;
}
