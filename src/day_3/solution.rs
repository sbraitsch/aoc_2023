use crate::utils;

#[derive(Debug)]
struct Element {
    row: usize,
    col: usize,
    length: usize,
    value: usize,
    could_be_gear: bool,
}

impl Element {
    fn from_input(input: &Vec<String>) -> Vec<Self> {
        let mut elements = vec![];
        input.into_iter().enumerate().for_each(|(row, line)| {
            let mut num_buffer = String::new();
            line.chars().enumerate().for_each(|(col, c)| match c {
                '.' => {
                    if !num_buffer.is_empty() {
                        elements.push(Element::clear_buffer_and_create(
                            &mut num_buffer,
                            (row, col),
                        ));
                    }
                }
                '0'..='9' => {
                    num_buffer.push(c);
                    if col == line.len() - 1 {
                        elements.push(Element::clear_buffer_and_create(
                            &mut num_buffer,
                            (row, col),
                        ));
                    }
                }
                _ => {
                    if !num_buffer.is_empty() {
                        elements.push(Element::clear_buffer_and_create(
                            &mut num_buffer,
                            (row, col),
                        ));
                    }
                    elements.push(Element {
                        row,
                        col,
                        length: 1,
                        value: 0,
                        could_be_gear: c == '*',
                    });
                }
            });
        });
        elements
    }

    fn clear_buffer_and_create(buffer: &mut String, (row, col): (usize, usize)) -> Element {
        let e = Element {
            row,
            col: col - buffer.len(),
            length: buffer.len(),
            value: buffer.parse::<usize>().unwrap(),
            could_be_gear: false,
        };
        buffer.clear();
        e
    }

    fn find_adjacent<'a>(&'a self, elements: &'a Vec<Element>, reverse: bool) -> Vec<&Element> {
        elements
            .iter()
            .filter(|e| match reverse {
                false => e.value == 0 && Element::adjacent_condition(self, e),
                true => e.value > 0 && Element::adjacent_condition(e, self)
            })
            .collect()
    }

    fn adjacent_condition(reference_point: &Element, element_to_check: &Element) -> bool {
        element_to_check.row.abs_diff(reference_point.row) < 2
            && (reference_point.col.checked_sub(1).unwrap_or(0)
            ..=reference_point.col + reference_point.length)
            .contains(&element_to_check.col)
    }
}
pub fn solve() {
    let elements = Element::from_input(&utils::file_to_lines(3));
    println!("Part 1: {:?}", part_one(&elements));
    println!("Part 2: {:?}", part_two(&elements));
}

fn part_one(elements: &Vec<Element>) -> usize {
    elements
        .iter()
        .filter(|e| e.value > 0)
        .map(|e| if e.find_adjacent(elements, false).len() > 0 { e.value } else { 0 })
        .sum()
}

fn part_two(elements: &Vec<Element>) -> usize {
    elements
        .iter()
        .filter(|e| e.could_be_gear)
        .map(|e| {
            let adj = e.find_adjacent(elements, true);
            if adj.len() == 2 {
                adj.iter().fold(0, |acc, v| v.value.max(acc * v.value)) } else { 0 }
        })
        .sum()
}
