use crate::utils;
use std::collections::HashMap;
use std::time::Instant;

pub fn solve() {
    let input = utils::file_to_string("15");
    let escaped_input = input.trim_end_matches('\n');
    let segments: Vec<&str> = escaped_input.split(',').collect();

    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&segments), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&segments), time.elapsed());
}

fn part_one(segments: &Vec<&str>) -> usize {
    segments.iter().map(|seg| { hash(seg) }).sum()
}


fn part_two(segments: &Vec<&str>) -> usize {
    let mut box_map: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    segments.iter().for_each(|seg| {
        match seg.ends_with('-') {
            true => {
                let label = &seg[0..seg.len() - 1];
                let box_num = hash(label);
                box_map.entry(box_num).and_modify(|vec| {
                    if let Some(idx) = vec.iter().position(|e| e.0 == label) {
                        vec.remove(idx);
                    }
                }).or_insert(vec![(String::from(label), 0)]);
            },
            false => {
                let (label, focal_str) = seg.split_once('=').unwrap();
                let focal_size= focal_str.parse::<usize>().unwrap();
                let box_num = hash(label);
                box_map.entry(box_num).and_modify(|vec| {
                    if let Some(idx) = vec.iter().position(|e| e.0 == label) {
                        vec.get_mut(idx).unwrap().1 = focal_size;
                    } else {
                        vec.push((String::from(label), focal_size));
                    }
                }).or_insert(vec![(String::from(label), focal_size)]);
            }
        }
    });
    box_map.iter().map(|(num, lenses)| {
        lenses.iter().enumerate().map(|(idx, (_, f))| {
            (1 + num) * (1 + idx) * f
        }).sum::<usize>()
    }).sum()
}

fn hash(segment: &str) -> usize {
    segment.chars().fold(0, |acc, c| { ((acc + usize::from(c as u8)) * 17) % 256 })
}
