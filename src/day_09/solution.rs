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
    histories.iter().cloned().map(|h| extrapolate(h)).sum()
}

fn part_two(histories: &Vec<Vec<isize>>) -> isize {
    histories
        .iter()
        .cloned()
        .map(|mut h| {
            h.reverse();
            extrapolate(h)
        })
        .sum()
}

fn extrapolate(mut current_history: Vec<isize>) -> isize {
    let mut edge_values = vec![*current_history.last().unwrap()];
    while current_history.iter().any(|v| *v != 0) {
        current_history = current_history
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        edge_values.push(*current_history.last().unwrap());
    }
    edge_values.iter().sum()
}
