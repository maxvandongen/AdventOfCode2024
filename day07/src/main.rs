use std::{fs::File, io::Read, iter};

use itertools::Itertools;

#[derive(Clone)]
enum Operation {
    ADD,
    MULTIPLY,
    CONCAT
}

#[derive(Debug)]
struct Equation {
    result: usize,
    components: Vec<usize>,
}

struct SolutionAttempt {
    operations: Vec<Operation>,
}

impl From<Vec<char>> for SolutionAttempt {
    fn from(values: Vec<char>) -> Self {
        SolutionAttempt {
            operations: values
                .iter()
                .map(|c| match c {
                    '+' => Operation::ADD,
                    '*' => Operation::MULTIPLY,
                    '|' => Operation::CONCAT,
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl Equation {
    fn is_solveable_without_concat(&self) -> bool {
        iter::repeat_n(['+', '*'], self.components.len() - 1)
            .multi_cartesian_product()
            .map(|att| att.into())
            .any(|att| self.solves(att))
    }

    fn is_solveable_with_concat(&self) -> bool {
        iter::repeat_n(['+', '*', '|'], self.components.len() - 1)
            .multi_cartesian_product()
            .map(|att| att.into())
            .any(|att| self.solves(att))
    }

    fn solves(&self, attempt: SolutionAttempt) -> bool {
        let components = self.components.clone();
        let operations = attempt.operations.clone();
        let mut component_iter = components.iter();
        let mut operations_iter = operations.iter();
        let mut accumulator: usize = *component_iter.next().expect("has initial value");
        loop {
            match operations_iter.next() {
                Some(op) => match op {
                    Operation::ADD => {
                        accumulator =
                            accumulator + component_iter.next().expect("component should exist")
                    }
                    Operation::MULTIPLY => {
                        accumulator =
                            accumulator * component_iter.next().expect("component should exist")
                    }
                    Operation::CONCAT => {
                        let next_comp = component_iter.next().expect("component should exist");
                        let factor = 10_usize.pow(next_comp.to_string().len().try_into().unwrap()) ;
                        accumulator = accumulator*factor+next_comp;
                    },
                },
                None => return accumulator == self.result,
            }
        }
    }
}

fn main() {
    let file_name = "input";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");

    let equations: Vec<Equation> = content
        .split("\n")
        .map(|line| {
            let mut split_line = line.split(": ");
            let result: usize = split_line
                .next()
                .expect("Has result")
                .parse()
                .expect("result is an integer");
            let components: Vec<usize> = split_line
                .next()
                .expect("Has components")
                .split(" ")
                .map(|val| val.parse().expect("component is an integer"))
                .collect();
            Equation { result, components }
        })
        .collect();
    let part_one_solution: usize = equations
        .iter()
        .filter(|eq| eq.is_solveable_without_concat())
        .map(|eq| eq.result)
        .sum();
    println!("Part one: {}", part_one_solution);
    let part_two_solution: usize = equations
        .iter()
        .filter(|eq| eq.is_solveable_with_concat())
        .map(|eq| eq.result)
        .sum();
    println!("Part two: {}", part_two_solution);
}
