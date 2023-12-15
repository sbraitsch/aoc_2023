use std::fs::{read_to_string, File};
use std::io::{self, BufRead};

pub fn file_to_string(day: &str) -> String {
    read_to_string(format!("src/day_{day}/input.txt")).unwrap()
}

pub fn file_to_lines(day: &str) -> Vec<String> {
    let file = File::open(format!("src/day_{day}/input.txt")).expect("No such file.");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Error parsing line"))
        .collect()
}