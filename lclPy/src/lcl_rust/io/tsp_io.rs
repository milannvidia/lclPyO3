use std::{
    f64::consts::PI,
    fs::File,
    io::{self, BufRead, BufReader},
    usize, vec,
};
static RRR: f64 = 6378.388;

pub enum TspReader {
    DistanceMatrix { file: String, size: usize },
    Coord2d { file: String, size: usize },
    Dms { file: String, size: usize },
}
impl TspReader {
    pub fn get_distance_matrix(&self) -> Result<Vec<Vec<usize>>, io::Error> {
        match self {
            TspReader::DistanceMatrix { file, size } => {
                let f = File::open(file)?;
                let br = BufReader::new(f);
                let mut current_line_length = 0;
                let mut matrix: Vec<Vec<usize>> = vec![vec![]];
                for x in br.lines() {
                    let line = x.unwrap();
                    if line.contains('#') {
                        continue;
                    }
                    let mut res: Vec<usize> = line
                        .split_whitespace()
                        .map(|number| number.parse::<usize>().unwrap())
                        .collect();
                    if res.len() + current_line_length <= *size {
                        current_line_length += res.len();
                        let index = matrix.len() - 1;
                        matrix[index].append(&mut res);
                    } else {
                        current_line_length = res.len();
                        matrix.push(res);
                    }
                }
                return Ok(matrix);
            }
            TspReader::Dms { file, size } => {
                let f = File::open(file)?;
                let br = BufReader::new(f);
                let mut cities: Vec<(f64, f64)> = vec![];

                for x in br.lines() {
                    let line = x.unwrap();
                    if line.contains('#') {
                        continue;
                    }

                    let res: Vec<&str> = line.split_whitespace().collect();
                    let mut lat = res[0].parse::<f64>().unwrap()
                        + res[1].parse::<f64>().unwrap() / 60f64
                        + res[2].parse::<f64>().unwrap() / 3600f64;
                    if res[3] == "S" {
                        lat *= -1f64;
                    }
                    let mut long = res[4].parse::<f64>().unwrap()
                        + res[5].parse::<f64>().unwrap() / 60f64
                        + res[6].parse::<f64>().unwrap() / 3600f64;
                    if res[7] == "W" {
                        long *= -1f64;
                    }
                    cities.push((lat, long));
                }
                let mut matrix: Vec<Vec<usize>> = long_lat_to_distmatrix(&cities);
                return Ok(matrix);
            }
            TspReader::Coord2d { file, size } => {
                let matrix: Vec<Vec<usize>> = vec![vec![]];
                return Ok(matrix);
            }
        }
    }
}

fn long_lat_to_distmatrix(cities: &Vec<(f64, f64)>) -> Vec<Vec<usize>> {}

fn dist_globe(a: (f64, f64), b: (f64, f64)) -> usize {
    let dlat = degree_to_rad(b.0 - a.0);
    let dlong = degree_to_rad(b.1 - a.1);
    let a=(dlat/2).sin()*
}

fn degree_to_rad(degree: f64) -> f64 {
    PI * (degree / 180f64)
}
