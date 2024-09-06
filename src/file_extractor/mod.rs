use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_file(file_location: std::string::String) -> Result<std::vec::Vec<f64>,Box<dyn std::error::Error>> {
    let file = File::open(file_location)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines().map(
        |line| line.unwrap().parse::<f64>().unwrap()
    ).collect())
}
