use crate::utils;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::pin::Pin;

pub fn solve() {
    let lines = utils::file_to_lines("04")
        .iter()
        .map(|l| {
            let (winning, drawn) = l.split_once(": ").unwrap().1.split_once('|').unwrap();
            (
                winning
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<HashSet<usize>>(),
                drawn
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<HashSet<usize>>(),
            )
        })
        .collect::<Vec<(HashSet<usize>, HashSet<usize>)>>();
    println!("Part 1: {:?}", part_one(&lines));
    println!("Part 2: {:?}", part_two(&lines));
}

fn part_one(cards: &Vec<(HashSet<usize>, HashSet<usize>)>) -> u32 {
    cards
        .iter()
        .map(|(winning, drawn)| {
            let intersection = winning.intersection(drawn).count();
            match intersection {
                0 => 0,
                x => 2u32.pow((x - 1) as u32),
            }
        })
        .sum()
}

fn part_two(cards: &Vec<(HashSet<usize>, HashSet<usize>)>) -> usize {
    let mut card_copy_map: HashMap<usize, usize> = HashMap::new();
    for i in 0..cards.len() { card_copy_map.insert(i, 1); }
    cards
        .iter()
        .enumerate()
        .for_each(|(idx, (winning, drawn))| {
            let win_count = winning.intersection(drawn).count();
            let copy_count = card_copy_map.get(&idx).unwrap().clone();
            for i in idx + 1..=(idx + win_count).clamp(0, cards.len() - 1) {
                match card_copy_map.entry(i) {
                    Entry::Occupied(ref mut entry) => {
                        *entry.get_mut() += copy_count;
                    }
                    _ => {}
                }
            }

        });
    card_copy_map.values().sum()
}
