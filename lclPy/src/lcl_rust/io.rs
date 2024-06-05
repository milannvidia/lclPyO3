use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

use csv::ReaderBuilder;

pub(crate) fn read_csv(
    fileLocation: String,
    delimiter: char,
) -> Result<Vec<Vec<usize>>, io::Error> {
    let f = File::open(fileLocation)?;
    let br = BufReader::new(f);

    let matrix: Vec<Vec<usize>> = br
        .lines()
        .map(|l| {
            l.unwrap()
                .split(delimiter)
                .map(|number| number.parse().unwrap())
                .collect()
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
