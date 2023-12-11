use crate::utils;
use itertools::Itertools;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    fn distance_to(&self, other: &Node) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub fn solve() {
    let lines = utils::file_to_lines("11");
    let galaxies: Vec<Node> = lines.iter().enumerate().map(|(y, line)| {
            line.chars().enumerate().filter_map(|(x, char)| {
                    if char == '#' { Some(Node { x, y }) } else { None }
            }).collect::<Vec<Node>>()
        }).flatten().collect();
    let used_rows: Vec<usize> = galaxies.iter().map(|n| n.y).unique().collect();
    let used_cols: Vec<usize> = galaxies.iter().map(|n| n.x).unique().collect();

    let mut time = Instant::now();

    println!(
        "Part 1: {:?} in {:?}",
        measure_galaxy_distances(&mut expand_universe(galaxies.clone(), 1, &used_rows, &used_cols)),
        time.elapsed()
    );
    time = Instant::now();
    println!(
        "Part 2: {:?} in {:?}",
        measure_galaxy_distances(&mut expand_universe(galaxies, 999_999, &used_rows, &used_cols)),
        time.elapsed()
    );
}

fn expand_universe(mut nodes: Vec<Node>, expansion: usize, used_rows: &Vec<usize>, used_cols: &Vec<usize>, ) -> Vec<Node> {
    nodes.iter_mut().for_each(|node| {
        node.x += (node.x - used_cols.iter().filter(|idx| **idx < node.x).count()) * expansion;
        node.y += (node.y - used_rows.iter().filter(|idx| **idx < node.y).count()) * expansion;
    });
    nodes
}

fn measure_galaxy_distances(galaxies: &mut Vec<Node>) -> usize {
    (0..galaxies.len()).map(|i| {
        (i + 1..galaxies.len()).map(|j| galaxies[i].distance_to(&galaxies[j])).sum::<usize>()
    }).sum::<usize>()
}
