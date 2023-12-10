use crate::day_10::solution::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::utils;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn solve() {
    let lines = utils::file_to_lines("10");
    let mut start_idx = 0;
    let dim = lines[0].len();
    let map: Vec<Vec<char>> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            if let Some(idx) = line.chars().position(|c| c == 'S') {
                start_idx = y * line.len() + idx;
            }
            line.chars().collect()
        })
        .collect();

    let mut time = Instant::now();
    let start_points = vec![
        start_idx - 1,
        start_idx + 1,
        start_idx - dim,
        start_idx + dim,
    ];
    let max_loop = start_points
        .iter()
        .map(|first| get_loop(&map, start_idx, *first, dim))
        .max_by(|a, b| a.len().cmp(&&b.len()))
        .unwrap();

    println!("Part 1: {:?} in {:?}", part_one(&max_loop), time.elapsed());
    time = Instant::now();
    println!(
        "Part 2: {:?} in {:?}",
        part_two(&max_loop, lines),
        time.elapsed()
    );
}

fn get_loop(map: &Vec<Vec<char>>, start: usize, first: usize, dim: usize) -> Vec<usize> {
    let mut indices = vec![];
    let mut cur = first;
    let mut prev = start;
    let mut next;
    while cur != start {
        let dir = get_dir(&cur, &prev);
        next = match map[cur / dim][cur % dim] {
            '|' => match dir {
                UP => cur - dim,
                DOWN => cur + dim,
                _ => start,
            },
            '-' => match dir {
                LEFT => cur - 1,
                RIGHT => cur + 1,
                _ => start,
            },
            'L' => match dir {
                LEFT => cur - dim,
                DOWN => cur + 1,
                _ => start,
            },
            'J' => match dir {
                RIGHT => cur - dim,
                DOWN => cur - 1,
                _ => start,
            },
            'F' => match dir {
                LEFT => cur + dim,
                UP => cur + 1,
                _ => start,
            },
            '7' => match dir {
                RIGHT => cur + dim,
                UP => cur - 1,
                _ => start,
            },
            'S' => start,
            _ => start,
        };
        indices.push(cur);
        prev = cur;
        cur = next;
    }
    indices
}

fn get_dir(cur: &usize, prev: &usize) -> Direction {
    match cur > prev {
        true => {
            if cur.abs_diff(*prev) == 1 {
                RIGHT
            } else {
                DOWN
            }
        }
        false => {
            if cur.abs_diff(*prev) == 1 {
                LEFT
            } else {
                UP
            }
        }
    }
}

fn part_one(lp: &Vec<usize>) -> usize {
    lp.len() / 2 + 1
}

fn part_two(lp: &Vec<usize>, lines: Vec<String>) -> usize {
    let offset = lines.len() / 4;
    let range = offset..lines.len() - offset;
    lines[range.clone()]
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line[range.clone()]
                .chars()
                .enumerate()
                .filter(|(x, char)| !lp.contains(&((y + offset) * line.len() + x + offset)))
                .count()
        })
        .sum()
}
