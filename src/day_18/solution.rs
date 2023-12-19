use crate::utils;
use std::time::Instant;
use itertools::Itertools;

struct Vertex {
    x: isize,
    y: isize,
}

pub fn solve() {
    let lines = utils::file_to_lines("18");
    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&lines), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&lines), time.elapsed());
}

fn part_one(lines: &Vec<String>) -> isize {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut cur = (0, 0);
    let mut border_len = 0;
    lines.iter().for_each(|line| {
        let split = line.split(' ').collect::<Vec<&str>>();
        let steps = split[1].parse::<isize>().unwrap();
        border_len += steps;
        match split[0] {
            "U" => cur.1 -= steps,
            "L" => cur.0 -= steps,
            "D" => cur.1 += steps,
            "R" => cur.0 += steps,
            _ => {}
        }
        vertices.push(Vertex { x: cur.0, y: cur.1 });
    });
    shoelace(vertices, border_len)
}

fn part_two(lines: &Vec<String>) -> isize {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut cur = (0, 0);
    let mut border_len = 0;
    lines.iter().for_each(|line| {
        let hex = line.split_once('(').unwrap().1.strip_prefix('#').unwrap().strip_suffix(')').unwrap();
        let steps = isize::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();
        let dir = hex.chars().last().unwrap();
        border_len += steps;
        match dir {
            '3' => cur.1 -= steps,
            '2' => cur.0 -= steps,
            '1' => cur.1 += steps,
            '0' => cur.0 += steps,
            _ => {}
        }
        vertices.push(Vertex { x: cur.0, y: cur.1 });
    });
    shoelace(vertices, border_len)
}

fn shoelace(vertices: Vec<Vertex>, border_len: isize) -> isize {
    let shoelace = vertices.iter().tuple_windows().fold(0, |acc, (a, b)| {
        acc + a.x * b.y - a.y * b.x
    });
    (border_len + shoelace) / 2 + 1
}