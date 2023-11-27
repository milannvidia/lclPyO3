mod lcl_rust;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::lcl_rust::localsearch::mov::ArraySwap::ArraySwap;
use crate::lcl_rust::eval::tspeval::Tspeval;


fn main() {
    let reader =BufReader::new(File::open("src/distanceMatrix").unwrap());

    let matrix: Vec<Vec<u64>> = reader.lines()
        .map(|l| l.unwrap().split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();

    let swapfunc=ArraySwap{size:48};
    let eval=Tspeval{distance_matrix:matrix,size:48};




    println!("Distance matrix:\n{:?}", matrix);
}
