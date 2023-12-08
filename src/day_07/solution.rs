use crate::utils;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    all: String,
    value: usize,
    bid: usize,
    allow_jokers: bool,
}

fn char_to_num(c: char, allow_jokers: bool) -> u32 {
    match c {
        'T' => 10,
        'J' => {
            if allow_jokers {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        any => any.to_digit(10).unwrap(),
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => {
                for i in 0..5 {
                    let a = char_to_num(self.all.chars().nth(i).unwrap(), self.allow_jokers);
                    let b = char_to_num(other.all.chars().nth(i).unwrap(), self.allow_jokers);
                    return match a.cmp(&b) {
                        Ordering::Equal => continue,
                        order => order,
                    };
                }
                Ordering::Equal
            }
            order => order,
        }
    }
}

impl Hand {
    fn from_input(line: String, allow_jokers: bool) -> Self {
        let (all, bid) = line.split_once(' ').unwrap();
        Hand {
            all: String::from(all),
            value: Hand::get_hand_value(all, allow_jokers),
            bid: bid.parse::<usize>().unwrap(),
            allow_jokers,
        }
    }

    fn parse_hand(cards: &str, allow_jokers: bool) -> (Vec<usize>, usize) {
        let mut card_map: HashMap<char, usize> = HashMap::new();
        cards.chars().for_each(|c| {
            card_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        });
        let jokers = *card_map.get(&'J').unwrap_or(&0);
        if allow_jokers && jokers > 0 {
            let max_char = card_map
                .iter()
                .filter(|(k, _)| **k != 'J')
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap_or((&'A', &0))
                .0;
            card_map
                .entry(*max_char)
                .and_modify(|v| *v += jokers)
                .or_insert(5);
            card_map.remove(&'J');
        }
        let values = card_map.into_values().collect::<Vec<usize>>();
        let max = *values.iter().max().unwrap();
        (values, max)
    }

    fn get_hand_value(cards: &str, allow_jokers: bool) -> usize {
        let (values, max) = Hand::parse_hand(cards, allow_jokers);

        match max {
            x if x > 3 => x + 1,
            3 => {
                if values.contains(&2) {
                    4
                } else {
                    3
                }
            }
            2 => values.iter().filter(|v| **v == 2usize).count(),
            _ => 0,
        }
    }
}

pub fn solve() {
    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(), time.elapsed());
}

fn part_one() -> usize {
    sum_solution(false)
}

fn part_two() -> usize {
    sum_solution(true)
}

fn sum_solution(allow_jokers: bool) -> usize {
    let mut hands = utils::file_to_lines("07")
        .into_iter()
        .map(|line| Hand::from_input(line, allow_jokers))
        .collect::<Vec<Hand>>();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}
