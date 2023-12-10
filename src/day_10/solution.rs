use crate::utils;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub fn solve() {
    let lines = utils::file_to_lines("10");
    let mut start_idx = 0;

    let mut pipes: HashMap<usize, Vec<usize>> = HashMap::new();
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let idx: isize = (y * line.len() + x) as isize;
            let links = match c {
                '|' => vec![idx - line.len() as isize, idx + line.len() as isize],
                '-' => vec![idx - 1, idx + 1],
                'L' => vec![idx - line.len() as isize, idx + 1],
                'J' => vec![idx - line.len() as isize, idx - 1],
                'F' => vec![idx + line.len() as isize, idx + 1],
                '7' => vec![idx + line.len() as isize, idx - 1],
                'S' => {
                    start_idx = usize::try_from(idx).unwrap();
                    let starts = vec![
                        idx - line.len() as isize,
                        idx + line.len() as isize,
                        idx - 1,
                        idx + 1,
                    ];
                    starts
                }
                _ => vec![],
            };
            pipes.insert(
                y * line.len() + x,
                links
                    .iter()
                    .filter(|i| {
                        let range = 0..(lines.len() * line.len()) as isize;
                        range.contains(i)
                    })
                    .map(|i| usize::try_from(*i).unwrap())
                    .collect(),
            );
        })
    });

    let mut time = Instant::now();
    let start_points = &pipes[&start_idx];
    let max_loop = start_points
        .iter()
        .map(|first| get_loop_indices(first, start_idx, &pipes))
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

fn part_one(lp: &Vec<usize>) -> usize {
    lp.len() / 2 + 1
}

fn part_two(lp: &Vec<usize>, lines: Vec<String>) -> usize {
    let offset = lines.len() / 4;
    let range = offset..lines.len() - offset;
    lines[range.clone()].iter().enumerate().map(|(y, line)| {
        line[range.clone()].chars().enumerate().filter(|(x,char)| {
            !lp.contains(&((y + offset) * line.len() + x + offset))
        }).count()
    }).sum()
}


fn get_loop_indices(first: &usize, start: usize, pipes: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut indices = vec![];
    let mut cur = first;
    let mut prev = start;
    let mut next = &pipes[&cur]
        .iter()
        .find(|idx| **idx != prev)
        .unwrap_or(&start)
        .clone();
    while *next != start {
        indices.push(*cur);
        prev = *cur;
        cur = next;
        next = &pipes[cur]
            .iter()
            .find(|idx| **idx != prev)
            .unwrap_or(&start);
    }
    indices
}

fn winding_num(indices: &Vec<usize>, cols: usize, point: (usize, usize)) -> bool {
    let mut winding_number = 0;
    let (x, y) = point;

    for i in 0..indices.len() {
        let current_index = indices[i];
        let next_index = indices[(i + 1) % indices.len()];

        let current_row = current_index / cols;
        let current_col = current_index % cols;

        let next_row = next_index / cols;
        let next_col = next_index % cols;

        if (current_row <= y && y < next_row) || (next_row <= y && y < current_row) {
            let cross_product = (next_col as isize - current_col as isize)
                * (y as isize - current_row as isize)
                - (next_row as isize - current_row as isize) * (x as isize - current_col as isize);

            if cross_product > 0 {
                winding_number += 1;
            } else if cross_product < 0 {
                winding_number -= 1;
            }
        }
    }

    winding_number != 0
}
