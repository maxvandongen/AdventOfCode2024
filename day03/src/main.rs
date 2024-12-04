use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let file_name = "input";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    content.push_str("do()");
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");
    content.push_str("don't()");
    let expression = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").expect("Is valid Regex");
    let mut values: Vec<(usize, usize)> = Vec::new();
    for (_, [left, right]) in expression.captures_iter(&content).map(|c| c.extract()) {
        values.push((
            left.parse().expect("parsed digits for left"),
            right.parse().expect("parsed digits for right"),
        ));
    }
    let part_one_answer: usize = values.iter().map(|(l, r)| l * r).sum();
    println!("Part 01: {}", part_one_answer);
    let mut indexed_strs: Vec<_> = Vec::new();
    content.split("do()").enumerate().for_each(|(i, do_part)| {
        do_part
            .split("don't()")
            .enumerate()
            .for_each(|(j, dont_part)| {
                indexed_strs.push((i, j, dont_part));
            })
    });
    let mut values_second: Vec<(usize, usize)> = Vec::new();
    indexed_strs
        .iter()
        .filter(|(_i, j, _part)| *j == 0)
        .map(|(_, _, part)| *part)
        .for_each(|haystack| {
        for (_, [left, right]) in expression.captures_iter(haystack).map(|c| c.extract()) {
            values_second.push((
                left.parse().expect("parsed digits for left"),
                right.parse().expect("parsed digits for right"),
            ));
        }
    });
    let part_two_answer: usize = values_second.iter().map(|(l, r)| l * r).sum();
    println!("Part 02: {}", part_two_answer);
}
