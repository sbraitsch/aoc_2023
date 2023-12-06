use std::time::Instant;
use crate::utils;

struct CubeData(u32, u32, u32);

impl CubeData {
    fn from_string(string: String) -> Self {
        let mut cube_data = CubeData(0, 0, 0);
        string
            .split(", ")
            .map(|cubes| cubes.split_once(' ').unwrap())
            .for_each(|(num, color)| {
                let count = num.parse::<u32>().unwrap();
                match color.trim() {
                    "red" => {
                        cube_data.0 = cube_data.0.max(count);
                    }
                    "green" => {
                        cube_data.1 = cube_data.1.max(count);
                    }
                    "blue" => {
                        cube_data.2 = cube_data.2.max(count);
                    }
                    _ => {}
                }
            });
        cube_data
    }

    fn impossibility_value(&self, game_num: usize) -> usize {
        if self.0 <= 12 && self.1 <= 13 && self.2 <= 14 {
            game_num
        } else {
            0
        }
    }

    fn pow(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

pub fn solve() {
    let parsed_cube_data = utils::file_to_lines("02")
        .iter()
        .map(|line| CubeData::from_string(line.split_once(": ").unwrap().1.replace(';', ",")))
        .collect();
    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&parsed_cube_data), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&parsed_cube_data), time.elapsed());
}

fn part_one(cube_data: &Vec<CubeData>) -> usize {
    cube_data
        .iter()
        .enumerate()
        .map(|(idx, data)| data.impossibility_value(idx + 1))
        .sum()
}

fn part_two(cube_data: &Vec<CubeData>) -> u32 {
    cube_data.iter().map(|data| data.pow()).sum()
}
