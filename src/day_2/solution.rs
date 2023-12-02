use crate::utils;

#[derive(Debug)]
struct CubeData {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeData {
    fn from_string(string: String) -> Self {
        let mut cube_data = CubeData {
            red: 0,
            green: 0,
            blue: 0,
        };
        string
            .trim()
            .split(',')
            .map(|dies| dies.trim().split_once(' ').unwrap())
            .for_each(|(num, color)| {
                let count = num.parse::<u32>().unwrap();
                match color.trim() {
                    "red" => {
                        if cube_data.red < count {
                            cube_data.red = count
                        }
                    }
                    "green" => {
                        if cube_data.green < count {
                            cube_data.green = count
                        }
                    }
                    "blue" => {
                        if cube_data.blue < count {
                            cube_data.blue = count
                        }
                    }
                    _ => {}
                }
            });
        cube_data
    }

    fn is_possible(&self, game_num: usize) -> usize {
        if self.red <= 12 && self.green <= 13 && self.blue <= 14 {
            game_num
        } else {
            0
        }
    }

    fn pow(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn solve() {
    let lines = utils::file_to_lines(2);
    println!("Part 1: {:?}", part_one(&lines));
    println!("Part 2: {:?}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .enumerate()
        .map(|(game_idx, line)| (game_idx, line.split_once(": ").unwrap().1))
        .map(|(game_idx, configs)| {
            CubeData::from_string(configs.replace(';', ",")).is_possible(game_idx + 1)
        })
        .sum()
}

fn part_two(lines: &Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|configs| CubeData::from_string(configs.replace(';', ",")).pow())
        .sum()
}
