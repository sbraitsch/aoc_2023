use crate::utils;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    all: String,
    value: usize,
    bid: usize,
    part: usize,
}

static PART: usize = 2;

fn char_to_num(c: char, part: usize) -> u32 {
    match c {
        'T' => 10,
        'J' => match part {
            2 => 1,
            _ => 11,
        },
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
                    let a = char_to_num(self.all.chars().nth(i).unwrap(), self.part);
                    let b = char_to_num(other.all.chars().nth(i).unwrap(), self.part);
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
    fn from_input(line: String, part: usize) -> Self {
        let (all, bid) = line.split_once(' ').unwrap();
        Hand {
            all: String::from(all),
            value: Hand::get_hand_value(all, part),
            bid: bid.parse::<usize>().unwrap(),
            part,
        }
    }

    fn parse_hand(cards: &str) -> (HashMap<char, usize>, usize) {
        let mut card_map: HashMap<char, usize> = HashMap::new();
        cards.chars().for_each(|c| {
            card_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        });
        let jokers = *card_map.get(&'J').unwrap_or(&0);
        (card_map, jokers)
    }

    fn get_hand_value(cards: &str, part: usize) -> usize {
        let (card_map, jokers) = Hand::parse_hand(cards);
        let values;
        let max: usize;
        if part == 1 {
            values = card_map.into_values().collect::<Vec<usize>>();
            max = *values.iter().max().unwrap();
        } else {
            values = card_map
                .into_iter()
                .filter(|(k, _)| *k != 'J')
                .map(|(_, v)| v)
                .collect();
            max = values.iter().max().unwrap_or(&0) + jokers;
        };

        match max {
            x if x > 3 => x + 1,
            3 => {
                return match part {
                    1 => if values.contains(&2) { 4 } else { 3 },
                    _ => if (values.iter().filter(|v| **v == 2).count() == 2)
                        || (jokers == 0 && values.contains(&2)) {
                        4
                    } else { 3 }
                }
            }
            2 => values.iter().filter(|v| **v == 2usize).count().max(1),
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
    sum_solution(1)
}

fn part_two() -> usize {
    sum_solution(2)
}

fn sum_solution(part: usize) -> usize {
    let mut hands = utils::file_to_lines("07")
        .into_iter()
        .map(|line| Hand::from_input(line, part))
        .collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            (i + 1) * h.bid
        })
        .sum()
}
