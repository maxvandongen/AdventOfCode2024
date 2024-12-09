use crate::Direction::{
    BottomToTop, LeftToRight, NorthEastToSouthWest, NorthWestToSouthEast, RightToLeft,
    SouthEastToNorthWest, SouthWestToNorthEast, TopToBottom,
};

use std::{
    fmt::{Display, Formatter, Result},
    fs::File,
    io::Read,
    slice::Iter,
};

enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    NorthWestToSouthEast,
    SouthEastToNorthWest,
    NorthEastToSouthWest,
    SouthWestToNorthEast,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            LeftToRight,
            RightToLeft,
            TopToBottom,
            BottomToTop,
            NorthWestToSouthEast,
            SouthEastToNorthWest,
            NorthEastToSouthWest,
            SouthWestToNorthEast,
        ];
        DIRECTIONS.iter()
    }
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
    value: char,
    used: bool,
}

#[derive(Debug)]
struct GameMap {
    points: Vec<Point>,
    max_line: usize,
    xmas_count: usize,
}

fn get_index(x: usize, y: usize, max_y: usize) -> usize {
    return (max_y) * y + x;
}

fn valid_position(x: isize, y: isize, max: isize) -> bool {
    x < max && x >= 0 && y >= 0 && y < max
}

fn next_x_y(x: isize, y: isize, step: isize, direction: &Direction) -> (isize, isize) {
    match direction {
        LeftToRight => (x + step, y),
        RightToLeft => (x - step, y),
        TopToBottom => (x, y + step),
        BottomToTop => (x, y - step),
        NorthWestToSouthEast => (x + step, y + step),
        SouthEastToNorthWest => (x - step, y - step),
        NorthEastToSouthWest => (x - step, y + step),
        SouthWestToNorthEast => (x + step, y - step),
    }
}

impl GameMap {
    fn mark_point_used(&mut self, x: usize, y: usize) {
        self.points
            .get_mut(get_index(x, y, self.max_line))
            .expect("Point exists")
            .used = true
    }

    fn get_char(&self, x: isize, y: isize) -> &char {
        &self
            .points
            .get(get_index(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                self.max_line,
            ))
            .expect("Point should exist")
            .value
    }
}

impl Display for GameMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut str_out = String::new();
        for line in self.points.chunks_exact(self.max_line) {
            for point in line {
                match point.used {
                    true => str_out.push_str(&format!("{}", point.value)),
                    false => str_out.push_str(&format!("{}", ".")),
                }
            }
            str_out.push_str("\n");
        }
        writeln!(f, "{}", str_out)
    }
}

fn build_game_map(file_name: &str) -> GameMap {
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");
    let line_length = content.clone().lines().next().expect("Has line").len();
    let mut points: Vec<Point> = Vec::new();
    content
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(y_idx, line)| {
            line.chars().enumerate().for_each(|(x_idx, c)| {
                points.push(Point {
                    x: x_idx as isize,
                    y: y_idx as isize,
                    value: c,
                    used: false,
                });
            });
        });
    GameMap {
        points,
        max_line: line_length,
        xmas_count: 0,
    }
}

fn main() {
    let file_name = "input";

    let mut map = build_game_map(&file_name);

    for direction in Direction::iterator() {
        for y in 0..map.max_line {
            'XY: for x in 0..map.max_line {
                let mut points_to_mark: Vec<(usize, usize)> = Vec::new();
                for (idx, c) in "XMAS".chars().enumerate() {
                    let (new_x, new_y) = next_x_y(x as isize, y as isize, idx as isize, direction);
                    // println!(
                    //     "{},{} is {}",
                    //     new_x,
                    //     new_y,
                    //     valid_position(new_x, new_y, map.max_line as isize)
                    // );
                    if !valid_position(new_x, new_y, map.max_line as isize)
                        || *map.get_char(new_x, new_y) != c
                    {
                        continue 'XY;
                    }
                    points_to_mark.push((new_x.try_into().unwrap(), new_y.try_into().unwrap()));
                }
                for (mark_x, mark_y) in points_to_mark {
                    map.mark_point_used(mark_x, mark_y);
                }
                map.xmas_count += 1;
            }
        }
    }

    println!("{}", map);
    println!("Part one: {}", map.xmas_count);

    let mut map = build_game_map(&file_name);
    for y in 1..map.max_line - 1 {
        for x in 1..map.max_line - 1 {
            let mut points_to_mark: Vec<(usize, usize)> = Vec::new();
            if *map.get_char(x as isize, y as isize) == 'A' {
                if (*map.get_char((x - 1) as isize, (y - 1) as isize) == 'M'
                    && *map.get_char((x + 1) as isize, (y + 1) as isize) == 'S'
                    && *map.get_char((x - 1) as isize, (y + 1) as isize) == 'M'
                    && *map.get_char((x + 1) as isize, (y - 1) as isize) == 'S')
                    || (*map.get_char((x - 1) as isize, (y - 1) as isize) == 'M'
                        && *map.get_char((x + 1) as isize, (y + 1) as isize) == 'S'
                        && *map.get_char((x - 1) as isize, (y + 1) as isize) == 'S'
                        && *map.get_char((x + 1) as isize, (y - 1) as isize) == 'M')
                    || (*map.get_char((x - 1) as isize, (y - 1) as isize) == 'S'
                        && *map.get_char((x + 1) as isize, (y + 1) as isize) == 'M'
                        && *map.get_char((x - 1) as isize, (y + 1) as isize) == 'S'
                        && *map.get_char((x + 1) as isize, (y - 1) as isize) == 'M')
                    || (*map.get_char((x - 1) as isize, (y - 1) as isize) == 'S'
                        && *map.get_char((x + 1) as isize, (y + 1) as isize) == 'M'
                        && *map.get_char((x - 1) as isize, (y + 1) as isize) == 'M'
                        && *map.get_char((x + 1) as isize, (y - 1) as isize) == 'S')
                {
                    map.mark_point_used(x - 1, y - 1);
                    map.mark_point_used(x - 1, y + 1);
                    map.mark_point_used(x + 1, y + 1);
                    map.mark_point_used(x + 1, y - 1);
                    map.mark_point_used(x, y);

                    map.xmas_count += 1;
                }
            }
        }
    }
    println!("{}", map);
    println!("Part two: {}", map.xmas_count);
}
