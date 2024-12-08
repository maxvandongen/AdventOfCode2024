use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    ops::{Add, Sub},
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: isize,
    y: isize,
    char: Option<char>,
    has_antinode: bool,
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            char: self.char,
            has_antinode: false,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            char: self.char,
            has_antinode: false,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

struct PuzzleMap {
    map: Vec<Position>,
    max_x: isize,
    max_y: isize,
}

fn get_index(x: isize, y: isize, max_y: isize) -> isize {
    return (max_y + 1) * y + x;
}

impl PuzzleMap {
    fn mark_antinode(&mut self, x: isize, y: isize) {
        self.map
            .get_mut(get_index(x, y, self.max_y) as usize)
            .expect("Position should exist")
            .has_antinode = true;
    }

    fn is_pos_in_bounds(&self, pos: Position) -> bool {
        return pos.x >= 0 && pos.x <= self.max_x && pos.y >= 0 && pos.y <= self.max_y;
    }
}

fn gen_map(file_name: &str) -> PuzzleMap {
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");

    let line_length = content.clone().lines().next().expect("Has line").len();

    let mut map: Vec<Position> = Vec::new();
    let mut chars: HashSet<char> = HashSet::new();

    content.split("\n").enumerate().for_each(|(y_idx, line)| {
        line.chars().enumerate().for_each(|(x_idx, c)| {
            let pos = Position {
                x: x_idx as isize,
                y: y_idx as isize,
                char: match c {
                    '.' => None,
                    c => {
                        chars.insert(c);
                        Some(c)
                    }
                },
                has_antinode: false,
            };
            map.push(pos);
        });
    });

    PuzzleMap {
        map,
        max_x: (line_length - 1) as isize,
        max_y: (line_length - 1) as isize,
    }
}

fn main() {
    let file_name = "input";

    let mut possible_antinodes: Vec<Position> = Vec::new();

    let mut puzzle_map = gen_map(&file_name);

    let chars: Vec<char> = puzzle_map
        .map
        .iter()
        .filter(|node| node.char.is_some())
        .map(|node| node.char.expect("has char"))
        .collect();

    for char in chars {
        let char_positions: Vec<_> = puzzle_map
            .map
            .iter()
            .filter(|pos| pos.char == Some(char))
            .collect();
        let char_permutations: Vec<_> = char_positions.iter().permutations(2).collect();

        char_permutations.iter().for_each(|val| {
            let left = *val[0];
            let right = *val[1];
            calculate_antinodes(left, right)
                .iter()
                .for_each(|node| possible_antinodes.push(*node));
        });
    }

    let in_bound_antinodes: Vec<_> = possible_antinodes
        .iter()
        .filter(|node| puzzle_map.is_pos_in_bounds(**node))
        .collect();

    in_bound_antinodes
        .iter()
        .for_each(|node| puzzle_map.mark_antinode(node.x, node.y));

    let marked_nodes: Vec<_> = puzzle_map
        .map
        .iter()
        .filter(|node| node.has_antinode)
        .collect();

    let part_one_value: usize = marked_nodes.len();

    println!("Part one: {}", part_one_value);

    let chars: Vec<char> = puzzle_map
        .map
        .iter()
        .filter(|node| node.char.is_some())
        .map(|node| node.char.expect("has char"))
        .collect();

    for char in chars {
        let char_positions: Vec<_> = puzzle_map
            .map
            .iter()
            .filter(|pos| pos.char == Some(char))
            .collect();
        let char_permutations: Vec<_> = char_positions.iter().permutations(2).collect();

        char_permutations.iter().for_each(|val| {
            let left = *val[0];
            let right = *val[1];
            calculate_antinodes_n(left, right, (puzzle_map.max_x + 1).try_into().unwrap())
                .iter()
                .for_each(|node| possible_antinodes.push(*node));
            possible_antinodes.push(*left);
            possible_antinodes.push(*right);
        });
    }

    let in_bound_antinodes: Vec<_> = possible_antinodes
        .iter()
        .filter(|node| puzzle_map.is_pos_in_bounds(**node))
        .collect();

    in_bound_antinodes
        .iter()
        .for_each(|node| puzzle_map.mark_antinode(node.x, node.y));

    let marked_nodes: Vec<_> = puzzle_map
        .map
        .iter()
        .filter(|node| node.has_antinode)
        .collect();

    // marked_nodes
    //     .iter()
    //     .for_each(|node| println!("{},{}", node.x, node.y));

    let part_two_value: usize = marked_nodes.len();

    println!("Part two: {}", part_two_value);
}

pub fn calculate_antinodes(pos1: &Position, pos2: &Position) -> Vec<Position> {
    let delta_x_one = pos1.x - pos2.x;
    let delta_y_one = pos1.y - pos2.y;
    let delta_x_two = pos2.x - pos1.x;
    let delta_y_two = pos2.y - pos1.y;
    [
        Position {
            x: pos1.x + delta_x_one,
            y: pos1.y + delta_y_one,
            char: pos1.char,
            has_antinode: false,
        },
        Position {
            x: pos2.x + delta_x_two,
            y: pos2.y + delta_y_two,
            char: pos2.char,
            has_antinode: false,
        },
    ]
    .to_vec()
}

pub fn calculate_antinodes_n(pos1: &Position, pos2: &Position, n: usize) -> Vec<Position> {
    let delta_x_one = pos1.x - pos2.x;
    let delta_y_one = pos1.y - pos2.y;
    let delta_x_two = pos2.x - pos1.x;
    let delta_y_two = pos2.y - pos1.y;
    let mut antinodes: Vec<Position> = Vec::new();
    for i in 1..n {
        let i = i as isize;
        antinodes.push(Position {
            x: pos1.x + i * delta_x_one,
            y: pos1.y + i * delta_y_one,
            char: pos1.char,
            has_antinode: false,
        });
        antinodes.push(Position {
            x: pos2.x + i * delta_x_two,
            y: pos2.y + i * delta_y_two,
            char: pos2.char,
            has_antinode: false,
        });
    }
    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_y_smaller() {
        let pos1 = Position {
            x: 2,
            y: 2,
            char: Some('a'),
            has_antinode: false,
        };

        let pos2 = Position {
            x: 4,
            y: 4,
            char: Some('a'),
            has_antinode: false,
        };

        assert_eq!(
            [
                Position {
                    x: 0,
                    y: 0,
                    char: Some('a'),
                    has_antinode: false
                },
                Position {
                    x: 6,
                    y: 6,
                    char: Some('a'),
                    has_antinode: false
                }
            ]
            .to_vec(),
            calculate_antinodes(&pos1, &pos2)
        )
    }

    #[test]
    fn x_smaller_y_larger() {
        let pos1 = Position {
            x: 2,
            y: 4,
            char: Some('a'),
            has_antinode: false,
        };

        let pos2 = Position {
            x: 4,
            y: 2,
            char: Some('a'),
            has_antinode: false,
        };

        assert_eq!(
            [
                Position {
                    x: 0,
                    y: 6,
                    char: Some('a'),
                    has_antinode: false
                },
                Position {
                    x: 6,
                    y: 0,
                    char: Some('a'),
                    has_antinode: false
                }
            ]
            .to_vec(),
            calculate_antinodes(&pos1, &pos2)
        )
    }
}
