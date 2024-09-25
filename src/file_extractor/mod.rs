use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;
use std::vec::Vec;

pub fn read_file(file_location: String) -> Result<Vec<f64>, Box<dyn Error>> {
    let file = File::open(file_location)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .collect())
}
