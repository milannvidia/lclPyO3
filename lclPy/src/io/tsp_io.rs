use std::{
    f64::consts::PI,
    fs::File,
    io::{self, BufRead, BufReader, Error},
    usize, vec,
};
static RRR: f64 = 6378.388;

pub enum TspReader {
    DistanceMatrix { file: String },
    Coord2d { file: String },
    Dms { file: String },
}
impl TspReader {
    /// Returns the get distance matrix of this [`TspReader`].
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn get_distance_matrix(&self) -> Result<Vec<Vec<usize>>, io::Error> {
        match self {
            TspReader::DistanceMatrix { file } => {
                let f = File::open(file)?;
                let br = BufReader::new(f);
                let mut res: Vec<usize> = vec![];

                for x in br.lines() {
                    let line = x.unwrap();
                    if !line.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
                        continue;
                    }
                    let mut row: Vec<usize> = line
                        .split_whitespace()
                        .map(|number| number.parse::<usize>().unwrap())
                        .collect();
                    res.append(&mut row);
                }
                let dimensions: f64 = (res.len() as f64).sqrt();
                if dimensions % 1f64 != 0f64 {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        "distanceMatrix is not a square",
                    ));
                }
                let matrix: Vec<Vec<usize>> = res
                    .chunks(dimensions as usize)
                    .map(|chunk| chunk.to_vec())
                    .collect();
                for i in 0..dimensions as usize {
                    if matrix[i][i] != 0 {
                        return Err(Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("distance to location {} itself is not zero", i),
                        ));
                    }
                }
                return Ok(matrix);
            }
            TspReader::Dms { file } => {
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
                let matrix: Vec<Vec<usize>> = long_lat_to_dist_matrix(&cities);
                return Ok(matrix);
            }
            TspReader::Coord2d { file } => {
                let matrix: Vec<Vec<usize>> = vec![vec![]];
                return Ok(matrix);
            }
        }
    }
}

fn long_lat_to_dist_matrix(cities: &Vec<(f64, f64)>) -> Vec<Vec<usize>> {
    return vec![vec![0]];
}

fn dist_globe(a: (f64, f64), b: (f64, f64)) -> usize {
    let dLat = degree_to_rad(b.0 - a.0);
    let dLong = degree_to_rad(b.1 - a.1);

    return 0;
}

fn degree_to_rad(degree: f64) -> f64 {
    PI * (degree / 180f64)
}
