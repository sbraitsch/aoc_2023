use crate::utils;
use itertools::Itertools;
use rayon::prelude::*;
use std::time::Instant;

type TFMap = Vec<Vec<(usize, usize, usize)>>;

pub fn solve() {
    let input = utils::file_to_string("05");

    let blocks: Vec<_> = input.split("\n\n").collect();
    let seeds: Vec<usize> = blocks[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mappings: TFMap = blocks[1..]
        .par_iter()
        .map(|b| {
            b.split_once('\n')
                .unwrap()
                .1
                .lines()
                .map(|line| {
                    let nums: Vec<usize> = line
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect();
                    (nums[0], nums[1], nums[2])
                })
                .collect()
        })
        .collect();

    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(seeds.clone(), &mappings), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(seeds, &mappings), time.elapsed());
}

fn part_one(seeds: impl IntoParallelIterator<Item = usize>, mappings: &TFMap) -> usize {
    seeds
        .into_par_iter()
        .map(|seed| {
            let mut carry = seed.clone();
            for i in 0..7 {
                carry = mappings[i]
                    .iter()
                    .find_map(|(dest, src, range)| {
                        if (src..&(src + range)).contains(&&carry) {
                            let diff = carry - src;
                            Some(dest + diff)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(carry);
            }
            carry
        })
        .min()
        .unwrap()
}

fn part_two(seeds: Vec<usize>, mappings: &TFMap) -> usize {
    seeds
        .into_iter()
        .tuples()
        .map(|(a, b)| part_one(a..a + b, mappings))
        .min()
        .unwrap()
}
