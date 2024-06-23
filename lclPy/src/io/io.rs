use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

pub fn read_csv(file_location: &str, delimiter: char) -> Result<Vec<Vec<usize>>, io::Error> {
    let f = File::open(file_location)?;
    let br = BufReader::new(f);

    let matrix: Vec<Vec<usize>> = br
        .lines()
        .map(|l| {
            if delimiter == ' ' {
                l.unwrap()
                    .split_whitespace()
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect()
            } else {
                l.unwrap()
                    .split(delimiter)
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect()
            }
        })
        .collect();
    if matrix.len() != matrix[0].len() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "distanceMatrix is not a square",
        ));
    }
    Ok(matrix)
}
