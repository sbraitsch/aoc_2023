use crate::utils;
use std::time::Instant;

pub fn solve() {
    let lines = utils::file_to_lines("13");
    let blocks = lines.split(|s| s.is_empty()).collect::<Vec<&[String]>>();
    let mut time = Instant::now();
    println!(
        "Part 1: {:?} in {:?}",
        part_one(&blocks),
        time.elapsed()
    );
    time = Instant::now();
    println!(
        "Part 2: {:?} in {:?}",
        part_two(),
        time.elapsed()
    );
}

fn part_one(blocks: &Vec<&[String]>) -> usize {
    blocks.iter().map(|block| {
        let (rows, columns) = parse_block(block);
        let row_val = find_mirror(rows) * 100;
        return if row_val == 0 {
            find_mirror(columns)
        } else { row_val };
    }).sum()
}



fn part_two() -> usize {
    0
}

fn parse_block(lines: &&[String]) -> (Vec<usize>, Vec<usize>) {
    let mut col_vec = vec![];
    let rows = lines.iter().enumerate().map(|(row, line)|{
        let binary_string = line.chars().enumerate().map(|(col, c)| {
            let bin= match c {
                '.' => '0',
                _ => '1'
            };
            if row == 0 {
                col_vec.push(String::from(bin))
            } else {
                col_vec.get_mut(col).unwrap().push(bin)
            }
            bin
        }).collect::<String>();
        usize::from_str_radix(&binary_string, 2).unwrap()
    }).collect();

    let columns = col_vec.iter().map(|s| usize::from_str_radix(&s, 2).unwrap()).collect();
    (rows, columns)
}

fn find_mirror(vec: Vec<usize>) -> usize {
    let mut tail: Vec<usize> = vec![];
    let mut best_mirror =(0, 0);
    for (i, row) in vec.iter().enumerate() {
        let last = tail.last();
        if let Some(n) = last {
            if n == row {
                let ml = (vec.len() - i).min(i);
                let mut t = tail.clone();
                t.reverse();
                let mut valid = true;
                for x in 1..ml {
                    if t[x] != vec[i + x] {
                        valid = false
                    }
                }
                if valid && ml > best_mirror.1 {
                    best_mirror = (i, ml);
                }
            }
        }
        tail.push(*row);
    }
    best_mirror.0
}

