use std::io::Error;
use std::ops::Div;
use std::{f64, io};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const RRR: f64 = 6371.0;
/// Reads the given file, skips over all lines with a "#" (used for comments) and returns a vector with all values;
///
/// # Arguments
/// file_location,delimiter
///
/// returns: Result<Vec<f64>, Error>
///
pub fn read_csv(file_location: &str, delimiter: Option<char>) -> Result<Vec<f64>, io::Error> {
    let f = File::open(file_location)?;
    let br = BufReader::new(f);
    let mut matrix: Vec<f64> = Vec::new();

    for line in br.lines() {
        if line.as_ref().unwrap().contains('#') {
            continue;
        }
        if delimiter.is_some() {
            matrix.append(
                &mut line
                    .unwrap()
                    .split(delimiter.unwrap())
                    .map(|f| f.parse().unwrap())
                    .collect(),
            );
        } else {
            matrix.append(
                &mut line
                    .unwrap()
                    .split_whitespace()
                    .map(|f| f.parse().unwrap())
                    .collect(),
            );
        }
    }
    Ok(matrix)
}

/// Uses read_csv to read a file and restructure it in a matrix
///
/// # Arguments
///
/// * `file`: file location
///
/// returns: Result<Vec<Vec<f64>>, Error>
///
pub fn read_distance_matrix(file: &str) -> Result<Vec<Vec<f64>>, io::Error> {
    let res: Vec<f64> = read_csv(file, None)?;

    let dimensions: f64 = (res.len() as f64).sqrt();
    if dimensions % 1f64 != 0f64 {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "distanceMatrix is not a square",
        ));
    }

    let matrix: Vec<Vec<f64>> = res
        .chunks(dimensions as usize)
        .map(|chunk| chunk.to_vec())
        .collect();
    for i in 0..dimensions as usize {
        if matrix[i][i] != 0f64 {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                format!("distance to location {} itself is not zero", i),
            ));
        }
    }
    Ok(matrix)
}

/// Uses read_csv to read a file, calculates the distance and restructures it in a matrix
///
/// # Arguments
///
/// * `file`: file location
///
/// returns: Result<Vec<Vec<f64>>, Error>
pub fn read_coord2d_to_distance_matrix(file: &str) -> Result<Vec<Vec<f64>>, io::Error> {
    let res: Vec<f64> = read_csv(file, None)?;
    let n: usize = res.len() / 2;
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let dist: f64 = f64::sqrt(
                (res[i * 2] - res[j * 2]).powf(2f64) + (res[i * 2 + 1] - res[j * 2 + 1]).powf(2f64),
            );
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }
    Ok(matrix)
}
/// Uses read_csv to read a file, calculates the distance and restructures it in a matrix
///
/// # Arguments
///
/// * `file`: file location
///
/// returns: Result<Vec<Vec<f64>>, Error>
///

pub fn read_dms_to_distance_matrix(file: &str) -> Result<Vec<Vec<f64>>, io::Error> {
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

    let matrix: Vec<Vec<f64>> = long_lat_to_dist_matrix(&cities);
    Ok(matrix)
}

fn long_lat_to_dist_matrix(cities: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
    let n: usize = cities.len();
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in i + 1..n {
            let dist: f64 = dist_globe(cities[i], cities[j]);
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }
    return matrix;
}

fn dist_globe(a: (f64, f64), b: (f64, f64)) -> f64 {
    let lat_a = a.0.to_radians();
    let lat_b = b.0.to_radians();

    let d_lat = lat_b - lat_a;
    let d_long = (b.1 - a.1).to_radians();

    let a = d_lat.div(2.0).sin().powf(2.0)
        + d_long.div(2.0).sin().powf(2.0) * lat_a.cos() * lat_b.cos();
    RRR * 2.0 * a.sqrt().asin()
}

/// Simple function to test if a distance matrix is symmetric or not
///
/// # Arguments
///
/// * `dist_matrix`:
///
/// returns: bool
///
/// # Examples
///
/// ```
/// use lclPyO3::aidfunc::io::check_if_distance_matrix_symmetric;
/// let distance_matrix: Vec<Vec<f64>> = vec![
///     vec![0.0, 2.0, 5.0, 8.0],
///     vec![2.0, 0.0, 4.0, 1.0],
///     vec![5.0, 4.0, 0.0, 7.0],
///     vec![8.0, 1.0, 7.0, 0.0],
/// ];
///
/// assert!(check_if_distance_matrix_symmetric(&distance_matrix))
/// ```
pub fn check_if_distance_matrix_symmetric(dist_matrix: &Vec<Vec<f64>>) -> bool {
    for i in 0..dist_matrix.len() {
        for j in 0..i {
            if dist_matrix[i][j] != dist_matrix[j][i] {
                return false;
            }
        }
    }
    true
}
