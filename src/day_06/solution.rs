use crate::utils;
use std::time::Instant;

type Race = (f64, f64);

pub fn solve() {
    // I could just hardcode the input and save time parsing, but that's lame
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
        .map(|n| n.parse::<f64>().unwrap())
        .zip(
            input[1]
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse::<f64>().unwrap()),
        )
        .collect::<Vec<Race>>()
        .into_iter()
        .map(|race| solve_quadratic(race))
        .product()
}

fn part_two(input: &Vec<String>) -> usize {
    let race = (
        input[0]
            .split_once(':')
            .unwrap()
            .1
            .replace(char::is_whitespace, "")
            .parse::<f64>()
            .unwrap(),
        input[1]
            .split_once(':')
            .unwrap()
            .1
            .replace(char::is_whitespace, "")
            .parse::<f64>()
            .unwrap(),
    );
    solve_quadratic(race)
}

fn solve_quadratic((available_time, dist_to_beat): Race) -> usize {
    let discriminant = available_time.powi(2) - 4.0 * dist_to_beat;

    if discriminant >= 0.0 {
        let upper_bound = ((available_time + discriminant.sqrt()) / 2.0).floor() as usize;
        let lower_bound = ((available_time - discriminant.sqrt()) / 2.0).ceil() as usize;
        return (lower_bound..=upper_bound).count();
    } else {
        panic!("No solutions!");
    }

}
