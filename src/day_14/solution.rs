use crate::utils;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use itertools::Itertools;
use crate::day_14::solution::Direction::{EAST, NORTH, SOUTH, WEST};

#[derive(PartialEq, Debug, PartialOrd, Ord, Eq)]
enum Element {
    ROCK,
    BOULDER,
    SPACE,
}


#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

type RockMap = HashMap<(usize, usize), Element>;

pub fn solve() {
    let lines = utils::file_to_lines("14");
    let mut rocks = RockMap::new();
    let dim_x = lines[0].len();
    let dim_y = lines.len();

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            rocks.insert(
                (x, y),
                match c {
                    '#' => Element::BOULDER,
                    'O' => Element::ROCK,
                    _ => Element::SPACE,
                },
            );
        })
    });

    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&mut rocks, dim_x, dim_y), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&mut rocks, dim_x, dim_y), time.elapsed());
}

fn part_one(map: &mut RockMap, dim_x: usize, dim_y: usize) -> usize {
    let range = 0..(dim_x * dim_y);
    let res = roll(map, &dim_x, dim_y, NORTH, range);
    res
}


fn part_two(map: &mut RockMap, dim_x: usize, dim_y: usize) -> usize {
    let mut res_map: HashMap<i32, (i32, i32)> = HashMap::new();
    let range = 0..(dim_x * dim_y);
    let rev_range = range.clone().rev();
    let mut counter = 0;
    let cycles = 1_000_000_000;
    let mut set = HashSet::new();
    loop {
        roll(map, &dim_x, dim_y, NORTH, range.clone());
        roll(map, &dim_x, dim_y, WEST, range.clone());
        roll(map, &dim_x, dim_y, SOUTH, rev_range.clone());
        let res = roll(map, &dim_x, dim_y, EAST, rev_range.clone());
        counter += 1;
        let mut term = false;
        res_map.entry(res as i32).and_modify(|v| {
            if counter - v.0 == v.1 && counter % v.1 == cycles % v.1 {
                set.insert(res);
            } else if set.contains(&res) {
                set.remove(&res);
                if set.len() == 1 {
                    term = true;
                }
            }
            *v = (counter, counter - v.0)
        }).or_insert((counter, 0));
        if term { break set.iter().sum() }
    }
}

fn roll(map: &mut RockMap, dim_x: &usize, dim_y: usize, dir: Direction, range: impl Iterator<Item= usize>) -> usize {
    let res = range.map(|idx| {
        let (x, y) = to_coords(idx, dim_x);
        let elem = map.get(&(x, y)).unwrap();
        let mut new_idx = idx.clone();
        if *elem == Element::ROCK {
            match dir {
                NORTH => {
                    while new_idx >= *dim_x && map[&to_coords(new_idx - dim_x, dim_x)] == Element::SPACE {
                        new_idx -= dim_x;
                    }
                }
                EAST => {
                    while (new_idx % dim_x < *dim_x - 1) && map[&to_coords(new_idx + 1, dim_x)] == Element::SPACE {
                        new_idx += 1;
                    }
                }
                SOUTH => {
                    while new_idx < (dim_y - 1) * dim_x && map[&to_coords(new_idx + dim_x, dim_x)] == Element::SPACE {
                        new_idx += dim_x;
                    }
                }
                WEST => {
                    while (new_idx % dim_x > 0) && map[&to_coords(new_idx - 1, dim_x)] == Element::SPACE {
                        new_idx -= 1;
                    }
                }
            }

            if new_idx != idx {
                map.entry(to_coords(new_idx, dim_x)).and_modify(|v| *v = Element::ROCK);
                map.entry((x, y)).and_modify(|v| *v = Element::SPACE);
            }
            dim_y - (new_idx / dim_x)
        } else {
            0
        }
    }).sum();
    res
}

fn print_map(map: &mut RockMap, dim_x: &usize) {
    map.iter().sorted_by(|((a_x, a_y), _), ((b_x, b_y), _)| {
        (a_y * dim_x + a_x).cmp(&(b_y * dim_x + b_x))
    }).for_each(|((x, _), e)| {
        if x % dim_x == 0 { println!() }
        match e {
            Element::ROCK => print!("O"),
            Element::BOULDER => print!("#"),
            Element::SPACE => print!(".")
        }
    });
    println!();
}

fn to_coords(idx: usize, dim_x: &usize) -> (usize, usize) {
    (idx % dim_x, idx / dim_x)
}
