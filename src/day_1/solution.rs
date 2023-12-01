use crate::utils;

pub fn solve() {
    let lines = utils::file_to_lines(1);
    println!("Part 1: {:?}", part_one(&lines));
    println!("Part 2: {:?}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> u32 {
    lines.into_iter()
        .map(|line| line
            .chars()
            .into_iter()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()
        )
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

fn part_two(lines: &Vec<String>) -> u32 {
    let mut parsed_lines = vec![];
    for s in lines {
        let replaced_string = s
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        parsed_lines.push(replaced_string);
    }
    part_one(&parsed_lines)
}
