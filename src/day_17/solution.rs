use std::collections::HashMap;
use std::hash::Hash;
use crate::utils;
use std::time::Instant;
use itertools::Itertools;

#[derive(Debug)]
struct Node {
    idx: usize,
    cost: usize,
    g_cost: usize,
    h_cost: usize,
    prev: Option<(usize, usize)>,
    is_target: bool,
}
pub fn solve() {
    let lines = utils::file_to_lines("17");
    let max_dim = lines.len() - 1;

    fn calc_h_cost(start: (usize, usize), end: (usize, usize)) -> usize {
        end.0.abs_diff(start.0) + end.1.abs_diff(start.1)
    }
    let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();
    lines.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, v)| {
            nodes.insert((x, y), Node {
                idx: y * lines.len() + x,
                cost: v.to_digit(10).unwrap() as usize,
                g_cost: 0,
                h_cost: calc_h_cost((x, y), (max_dim, max_dim)),
                prev: None,
                is_target: x == max_dim && y == max_dim
            });
        })
    });

    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&mut nodes, max_dim), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(), time.elapsed());
}

fn part_one(maze: &mut HashMap<(usize, usize), Node>, dim: usize) -> usize {
    let (path, heat_loss) = find_path(maze, dim);
    for y in 0..=dim {
        for x in 0..=dim {
            if path.contains(&(x, y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
    heat_loss
}

fn part_two() -> usize {
    1
}

fn find_path(maze: &mut HashMap<(usize, usize), Node>, dim: usize) -> (Vec<(usize, usize)>, usize) {
    let mut found: Vec<(usize, usize)> = Vec::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    found.push((0, 0));

    while !found.is_empty() {
        //found = sort_by_f_cost(maze, &found);
        let current_idx = found.remove(0);

        if maze[&current_idx].is_target {
            let mut i = current_idx;
            let mut path = Vec::new();
            while let Some(p) = maze.get(&i) {
                path.push(i);
                if let Some(pr) = p.prev {
                    i = pr;
                } else { break; }
            }
            path.reverse();
            return (path, maze[&current_idx].g_cost);
        } else {
            visited.push(current_idx);
            get_adjacent(current_idx, dim, &maze).iter().for_each(|adj| {
                if !visited.contains(&adj){
                    let cost_through_current = maze[&current_idx].g_cost + maze[adj].cost;
                    maze.entry(*adj).and_modify(|n| {
                        if !found.contains(&adj) {
                            n.g_cost = cost_through_current;
                            n.prev = Some(current_idx);
                            found.push(*adj);
                        } else {
                            if cost_through_current < n.g_cost {
                                n.g_cost = cost_through_current;
                                n.prev = Some(current_idx);
                            }
                        }
                    });
                }
            })
        }
    }
    (vec![], 0)
}

fn get_adjacent<'a>((x, y): (usize, usize), dim: usize, maze: &HashMap<(usize, usize), Node>) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    let cur = &maze[&(x, y)];
    let mut movable_x = true;
    let mut movable_y = true;
    if let Some(p) = cur.prev {
        let prev = &maze[&p];
        if let Some(pp) = prev.prev {
            let prevprev = &maze[&pp];
            if let Some(ppp) = prevprev.prev {
                if x == p.0 && p.0 == pp.0 && pp.0 == ppp.0 { movable_y = false}
                if y == p.1 && p.1 == pp.1 && pp.1 == ppp.1 { movable_x = false}
            }
        }
    }
    if y > 0 && movable_y { adjacent.push((x, y - 1))}
    if y < dim && movable_y{ adjacent.push((x, y + 1))}
    if x > 0 && movable_x { adjacent.push((x - 1, y))}
    if x < dim && movable_x { adjacent.push((x + 1, y))}
    adjacent
}

fn sort_by_f_cost(maze: &HashMap<(usize, usize), Node>, found: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    found.iter().sorted_by(|a, b| {
        let a_n = &maze[a];
        let b_n = &maze[b];
        (a_n.g_cost + a_n.h_cost).cmp(&(b_n.g_cost + b_n.h_cost))
    }).cloned().collect()
}


