use std::{fs::File, io::Read};

#[derive(Clone, Copy, Debug)]
enum PositionType {
    OPEN,
    BLOCKED,
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
    visited: bool,
    pos_type: PositionType,
}
struct Game {
    map: Vec<Position>,
    max_x: usize,
    max_y: usize,
    current_position: Position,
    current_direction: Direction,
    steps_taken: usize,
}

fn get_index(x: usize, y: usize, max_y: usize) -> usize {
    return (max_y + 1) * y + x;
}

impl Game {
    fn add_block(&mut self, x: usize, y: usize) {
        self.map
            .get_mut(get_index(x, y, self.max_y))
            .expect("Position should exist")
            .pos_type = PositionType::BLOCKED;
    }

    fn get_map_position(&self, x: usize, y: usize) -> Position {
        self.map
            .get(get_index(x, y, self.max_y))
            .expect("Position should exists")
            .clone()
    }

    fn mark_visited(&mut self, x: usize, y: usize) {
        self.map
            .get_mut(get_index(x, y, self.max_y))
            .expect("Position should exist")
            .visited = true;
        self.steps_taken += 1;
    }

    fn can_move(&self) -> bool {
        if !self.is_move_in_bounds() {
            return false;
        }
        let pos_to_move = match self.current_direction {
            Direction::UP => {
                self.get_map_position(self.current_position.x, self.current_position.y - 1)
            }
            Direction::DOWN => {
                self.get_map_position(self.current_position.x, self.current_position.y + 1)
            }
            Direction::LEFT => {
                self.get_map_position(self.current_position.x - 1, self.current_position.y)
            }
            Direction::RIGHT => {
                self.get_map_position(self.current_position.x + 1, self.current_position.y)
            }
        };
        match pos_to_move.pos_type {
            PositionType::OPEN => true,
            PositionType::BLOCKED => false,
        }
    }

    fn is_move_in_bounds(&self) -> bool {
        match self.current_direction {
            Direction::UP => self.current_position.y >= 1,
            Direction::DOWN => self.current_position.y < self.max_y,
            Direction::LEFT => self.current_position.x >= 1,
            Direction::RIGHT => self.current_position.x < self.max_x,
        }
    }

    fn turn_right(&mut self) {
        self.current_direction = match self.current_direction {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }

    fn perform_move(&mut self) -> bool {
        // println!("Move is in bounds: {}", self.is_move_in_bounds());
        if !self.is_move_in_bounds() {
            return false;
        }
        if self.can_move() {
            // Move
            let cur_pos = self.current_position.clone();
            let new_pos = match self.current_direction {
                Direction::UP => self.get_map_position(cur_pos.x, cur_pos.y - 1),
                Direction::DOWN => self.get_map_position(cur_pos.x, cur_pos.y + 1),
                Direction::LEFT => self.get_map_position(cur_pos.x - 1, cur_pos.y),
                Direction::RIGHT => self.get_map_position(cur_pos.x + 1, cur_pos.y),
            };
            self.current_position = new_pos.clone();
            self.mark_visited(new_pos.x, new_pos.y);
            // println!(
            //     "Moving from {},{} to {},{} due to direction {:?}",
            //     cur_pos.x, cur_pos.y, new_pos.x, new_pos.y, self.current_direction
            // );
            return true;
        }
        // println!("Turning right");
        self.turn_right();
        true
    }

    fn is_in_infinite_loop(&self) -> bool {
        self.steps_taken > (self.max_x + 1) * (self.max_y + 1)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn build_game(file_name: &str) -> Game {
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");

    let line_length = content.clone().lines().next().expect("Has line").len();
    let mut map: Vec<Position> = Vec::new();
    let mut current_pos: Option<Position> = None;

    content.split("\n").enumerate().for_each(|(y_idx, line)| {
        line.chars().enumerate().for_each(|(x_idx, c)| {
            let pos = Position {
                x: x_idx,
                y: y_idx,
                visited: c == '^',
                pos_type: match c {
                    '^' | '.' => PositionType::OPEN,
                    '#' => PositionType::BLOCKED,
                    _ => unreachable!(),
                },
            };
            map.push(pos);
            if c == '^' {
                current_pos = Some(pos.clone());
            }
        });
    });

    Game {
        map,
        max_x: line_length - 1,
        max_y: line_length - 1,
        current_position: current_pos.expect("Starting position exists"),
        current_direction: Direction::UP,
        steps_taken: 0,
    }
}

fn main() {
    let file_name = "input";

    let mut game = build_game(&file_name);
    loop {
        let has_moved = game.perform_move();

        if !has_moved {
            break;
        }
    }
    let visited_positions: Vec<_> = game.map.iter().filter(|val| val.visited).collect();
    println!("Part one: {}", visited_positions.len());
    let mut part_two_count: usize = 0;
    for x in 0..=game.max_x {
        for y in 0..=game.max_y {
            let mut updated_game = build_game(&file_name);

            updated_game.add_block(x, y);
            loop {
                let has_moved = updated_game.perform_move();

                if updated_game.is_in_infinite_loop() {
                    part_two_count += 1;
                    break;
                }

                if !has_moved {
                    break;
                }
            }
        }
    }
    println!("Part two: {}", part_two_count);
}
