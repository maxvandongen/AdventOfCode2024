use std::{fs::File, io::Read};

fn main() {
    let file_name = "input";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");
    let x = content
        .lines()
        .map(|str| str.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for line in x {
        left.push(line[0].parse().expect("Left side is a number"));
        right.push(line[1].parse().expect("right side is a number"));
    }
    left.sort();
    right.sort();
    let part_one_result: usize = left.iter().zip(right.iter()).map(|(l, r)| r.abs_diff(*l)).sum();
    let part_two_result: usize = left.iter().map(|l_val| right.clone().iter().filter(|r_val| *r_val == l_val).count() * l_val).sum();
    println!("Part 01: {}", part_one_result);
    println!("Part 02: {}", part_two_result);

}
