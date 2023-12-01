use crate::utils;

pub fn solve_day_1() {
    let lines = utils::file_to_lines(1);
    println!("Part 1: {:?}", part_one(&lines));
    println!("Part 2: {:?}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for s in lines {
        let numericals: Vec<u32> = s.chars().into_iter().filter_map(|c| c.to_digit(10)).collect();
        let (Some(x), Some(y)) = (numericals.first(), numericals.last()) else { panic!("Invalid Line in Input") };
        sum += x * 10 + y;
    }
    sum
}


fn part_two(lines: &Vec<String>) -> u32 {
    let mut parsed_lines = vec![];
    for s in lines {
        let replaced_string = s.replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        parsed_lines.push(replaced_string); 
    }
    part_one(&parsed_lines)
}


