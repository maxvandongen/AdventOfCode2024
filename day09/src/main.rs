use std::{
    fmt::{Display, Formatter, Result},
    fs::File,
    io::Read,
};

#[derive(Debug, Clone)]
struct Block {
    value: Option<String>,
    len: usize,
}

#[derive(Debug, Clone)]
struct BlockMap {
    blocks: Vec<Block>,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.value {
            Some(s) => write!(f, "{}", &s.repeat(self.len)),
            None => write!(f, "{}", &".".repeat(self.len)),
        }
    }
}

impl Display for BlockMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut out_str = String::new();
        for block in &self.blocks {
            out_str.push_str(&format!("{}", block));
        }
        write!(f, "{}", out_str)
    }
}

impl BlockMap {}

pub fn calculate_checksum(chars: Vec<char>) -> usize {
    let mut acc: usize = 0;
    chars.iter().enumerate().for_each(|(idx, c)| match c {
        '0'..='9' => acc += c.to_string().parse::<usize>().expect("this is a digit") * idx,
        '.' => acc += 0,
        _ => unreachable!(),
    });
    acc
}

fn calculate_checksum_block(blocks: Vec<Block>) -> usize {
    let mut acc: usize = 0;
    blocks
        .iter()
        .enumerate()
        .for_each(|(idx, b)| match &b.value {
            Some(val) => acc += val.parse::<usize>().expect("this is a number") * idx,
            None => acc += 0,
            // '0'..='9' => acc += c.to_string().parse::<usize>().expect("this is a digit") * idx,
            // '.' => acc += 0,
            // _ => unreachable!(),
        });
    acc
}

fn main() {
    let file_name = "input";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");
    let mut count: usize = 0;
    let mut filled: bool = true;
    let mut blocks: Vec<Block> = Vec::new();
    content.chars().for_each(|c| {
        for _ in 0..c.to_string().parse().expect("all chars used are numbers") {
            blocks.push(Block {
                value: match filled {
                    true => Some(count.to_string()),
                    false => None,
                },
                len: 1,
            });
        }
        if filled {
            count += 1;
        }
        filled = !filled;
    });
    let mut map = BlockMap { blocks };
    let max_swaps: usize = map
        .blocks
        .clone()
        .iter()
        .filter(|c| c.value.is_none())
        .count();
    for _ in 0..max_swaps {
        let first_dot_index = map
            .blocks
            .iter()
            .position(|c| c.value.is_none())
            .expect("will contain dot");
        let last_char_index = map
            .blocks
            .iter()
            .rposition(|c| c.value.is_some())
            .expect("wil; contain character");
        map.blocks.swap(first_dot_index, last_char_index);
        // println!("{}", format_chars(part_one_string.clone()));
    }
    // let part_one_string: Vec<_> = format!("{}", map.clone()).chars().collect();
    // println!("{}", format_chars(part_one_string.clone()));
    println!("Part one: {}", calculate_checksum_block(map.blocks));
    let mut blocks: Vec<Block> = Vec::new();
    filled = true;
    count = 0;
    content.chars().for_each(|c| {
        blocks.push(Block {
            value: match filled {
                true => Some(count.to_string()),
                false => None,
            },
            len: c.to_string().parse::<usize>().expect("count is number"),
        });
        if filled {
            count += 1;
        }
        filled = !filled;
    });
    let mut new_map = BlockMap { blocks };
    // println!("{}", new_map);
    let binding = new_map.blocks.clone();
    let last: usize = binding
        .last()
        .expect("Last block exist")
        .value
        .clone()
        .expect("has value")
        .parse()
        .expect("can be parsed to usize");
    for i in (0..=last).rev() {
        let vec = new_map.blocks.clone();
        let block_to_move = vec
            .iter()
            .find(|b| b.value == Some(i.to_string()))
            .expect("Block can be found");
        let block_to_move_position = vec
            .iter()
            .position(|b| b.value == Some(i.to_string()))
            .expect("Block position can be found");
        let block_to_move_to = vec
            .iter()
            .find(|block| block.value.is_none() && block.len >= block_to_move.clone().len);
        let block_to_move_to_position = vec
            .iter()
            .position(|block| block.value.is_none() && block.len >= block_to_move.len);
        match block_to_move_to {
            Some(block) => {
                if block_to_move_to_position.expect("Has position") > block_to_move_position {
                    continue;
                }
                match block.len > block_to_move.len {
                    true => {
                        let leftover_block = Block {
                            value: None,
                            len: block.len - block_to_move.len,
                        };
                        new_map
                            .blocks
                            .remove(block_to_move_to_position.expect("block exists"));
                        new_map.blocks.insert(
                            block_to_move_to_position.expect("block exists"),
                            block_to_move.clone(),
                        );
                        new_map.blocks.insert(
                            block_to_move_to_position.expect("position exists") + 1,
                            leftover_block,
                        );
                        new_map.blocks.remove(block_to_move_position + 1);
                        new_map.blocks.insert(
                            block_to_move_position + 1,
                            Block {
                                value: None,
                                len: block_to_move.len,
                            },
                        );
                        // new_map.blocks.remo
                        // splice magic
                    }
                    false => {
                        new_map.blocks.swap(
                            block_to_move_position,
                            block_to_move_to_position.expect("Block was found so position exists"),
                        );
                    }
                }
            }
            None => continue,
        }
    }
    // let part_two_string: Vec<_> = format!("{}", new_map.clone()).chars().collect();
    let mut final_blocks: Vec<Block> = Vec::new();
    new_map.blocks.iter().for_each(|block| {
        for _ in 0..block.len {
            final_blocks.push(Block {
                value: block.value.clone(),
                len: 1,
            });
        }
    });
    let final_map = BlockMap {
        blocks: final_blocks,
    };
    // println!("{}", final_map);
    println!("Part two: {}", calculate_checksum_block(final_map.blocks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum() {
        let char_vec: Vec<_> = "0099811188827773336446555566.............."
            .chars()
            .collect();
        assert_eq!(1928_usize, calculate_checksum(char_vec))
    }
}
