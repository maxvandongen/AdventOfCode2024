use std::{fs::File, io::Read};

#[derive(Clone, Debug)]
enum Direction {
    ASCENDING,
    DESCENDING,
}

#[derive(Clone, Debug)]
struct Level {
    steps: Vec<usize>,
    direction: Direction,
}

impl Level {
    fn is_safe(&self) -> bool {
        self.steps.windows(2).all(|w| {
            let diff = w[1] as isize - w[0] as isize;
            match self.direction {
                Direction::ASCENDING => diff >= 1 && diff <= 3,
                Direction::DESCENDING => -diff >= 1 && -diff <= 3,
            }
        })
    }

    fn is_safe_dampened(&self) -> bool {
        let len = self.steps.len();
        for i in 0..len {
            let new_steps: Vec<_> = if i == 0 {
                self.steps.clone()[1..].to_vec()
            } else if i == len{
                self.steps.clone()[..self.steps.len()-1].to_vec()
            } else {
                let mut temp = self.steps.clone()[0..i].to_vec();
                temp.extend(self.steps.clone()[i+1..].to_vec());
                temp
            };
            let new_level = Level {
                steps: new_steps,
                direction: self.direction.clone(),
            };
            if new_level.is_safe() {
                return true;
            }
        }
        false
    }
}

fn main() {
    let file_name = "input";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");
    // println!("{}", content);
    let levels: Vec<_> = content
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace();
            let steps: Vec<_> = parts
                .map(|step| step.parse::<usize>().expect("Steps are numbers"))
                .collect();
            let direction = match steps[0] > steps[steps.len() - 1] {
                true => Direction::DESCENDING,
                false => Direction::ASCENDING,
            };
            Level {
                steps: steps,
                direction: direction,
            }
        })
        .collect();
    let safe_count: usize = levels
        .iter()
        .map(|level| level.is_safe())
        .filter(|a| *a)
        .count();
    let safe_dampened_count: usize = levels
        .iter()
        .map(|level| level.is_safe_dampened())
        .filter(|a| *a)
        .count();
    println!("Part 1: {}", safe_count);
    println!("Part 2: {}", safe_dampened_count);
}
