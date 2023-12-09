use crate::utils;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let lines = utils::file_to_lines("09");
    let histories: Vec<Vec<isize>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&histories), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&histories), time.elapsed());
}

fn part_one(histories: &Vec<Vec<isize>>) -> isize {
    histories.iter().map(|h| extrapolate_val(h, false)).sum()
}

fn part_two(histories: &Vec<Vec<isize>>) -> isize {
    histories.iter().map(|h| extrapolate_val(h, true)).sum()
}

fn extrapolate_val(history: &Vec<isize>, prev: bool) -> isize {
    let mut current_history = history.clone();
    let mut edge_values = Vec::new();
    match prev {
        true => edge_values.push(*history.first().unwrap()),
        false => edge_values.push(*history.last().unwrap()),
    }
    while current_history.iter().any(|v| *v != 0) {
        current_history = current_history
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        match prev {
            true => edge_values.push(*current_history.first().unwrap()),
            false => edge_values.push(*current_history.last().unwrap()),
        }
    }
    return match prev {
        true => edge_values[..edge_values.len() - 1]
            .iter()
            .rev()
            .fold(0, |acc, x| x - acc),
        false => edge_values.iter().sum(),
    };
}
