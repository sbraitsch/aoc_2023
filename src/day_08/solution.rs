use std::collections::HashMap;
use crate::utils;
use std::time::Instant;
use num_integer::lcm;

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

pub fn solve() {
    let lines = utils::file_to_lines("08");
    let directions: Vec<char> = lines[0].chars().collect();
    let mut node_map: NodeMap = HashMap::new();

    lines[2..].into_iter().for_each(|line| {
        let split =  line.split_once(" = ").unwrap();
        let key = split.0;
        let (left, right) = split.1.trim_matches(|c| c == '(' || c == ')').split_once(", ").unwrap();
        node_map.insert(key, (left, &right));
    });

    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&node_map, &directions, "AAA", false), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&node_map, &directions), time.elapsed());
}

fn part_one(map: &NodeMap, directions: &Vec<char>, start: &str, part_2: bool) -> usize {
    let mut cur = start;
    let mut steps = 0;
    while if !part_2 { cur != "ZZZ"} else { !cur.ends_with('Z')} {
        directions.iter().for_each(|dir| {
            let (left, right) = map[cur];
            match dir {
                'L' => cur = left,
                _ => cur = right
            }
            steps += 1;
        })
    }
    steps
}

fn part_two(map: &NodeMap, directions: &Vec<char>) -> usize {
    map.iter().filter(|(k, _)| k.ends_with('A')).map(|(k, _)| {
        let cycle = part_one(map, directions, k, true);
        println!("Cycle for {k} -> {cycle}");
        cycle
    }).fold(1, |acc, x| lcm(acc, x))
}
