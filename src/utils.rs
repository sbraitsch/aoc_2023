use std::fs::{read_to_string, File};
use std::io::{self, BufRead};

pub fn file_to_string(day: u32) -> String {
    read_to_string(format!("src/day_{day}/input.txt")).unwrap()
}

pub fn file_to_lines(day: &str) -> Vec<String> {
    let file = File::open(format!("src/day_{day}/input.txt")).expect("No such file.");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Error parsing line"))
        .collect()
}

pub mod custom_macros {
    #[macro_export()]
    macro_rules! map {
        // Base case: an empty map
        () => {
            std::collections::HashMap::new()
        };

        // Recursive case: create a map with key-value pairs
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut temp_map = std::collections::HashMap::new();
                $(
                    temp_map.insert($key, $value);
                )*
                temp_map
            }
        };
    }
}
