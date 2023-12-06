use crate::utils;
use rayon::prelude::*;
use std::time::Instant;

type Race = (usize, usize);

pub fn solve() {
    let input = utils::file_to_lines("06");
    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&input), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&input), time.elapsed());
}

fn part_one(input: &Vec<String>) -> usize {
    input[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .zip(
            input[1]
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap()),
        )
        .collect::<Vec<Race>>()
        .into_par_iter()
        .map(|race| get_win_possibility_count(race))
        .product()
}

fn part_two(input: &Vec<String>) -> usize {
    let race = (
        input[0]
            .split_once(':')
            .unwrap()
            .1
            .replace(char::is_whitespace, "")
            .parse::<usize>()
            .unwrap(),
        input[1]
            .split_once(':')
            .unwrap()
            .1
            .replace(char::is_whitespace, "")
            .parse::<usize>()
            .unwrap(),
    );
    get_win_possibility_count(race)
}

fn get_win_possibility_count(race: Race) -> usize {
    let half_count = (1..=race.0 / 2)
        .into_par_iter()
        .filter(|hold_time| (hold_time * (race.0 - hold_time)) > race.1)
        .count();
    half_count + half_count - ((race.0 + 1) % 2)
}
