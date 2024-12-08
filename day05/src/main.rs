use std::{fs::File, io::Read};

#[derive(Debug, Clone)]
struct Rule {
    before: usize,
    after: usize,
}

impl Rule {
    fn applies(&self, update: &Update) -> bool {
        update.values.contains(&self.before) && update.values.contains(&self.after)
    }

    fn complies(&self, update: &Update) -> bool {
        let before_index = update
                .values
                .iter()
                .position(|val| *val == self.before)
                .expect("Before value exists in update");
            let after_index = update
                .values
                .iter()
                .position(|val| *val == self.after)
                .expect("After value exists in update");
            return before_index < after_index;
    }
}

#[derive(Debug, Clone)]
struct Update {
    values: Vec<usize>,
}

impl Update {
    fn get_middle_value(&self) -> usize {
        self.values[(self.values.len() - 1) / 2]
    }

    fn complies_with_rules(&self, rules: &Vec<Rule>) -> bool {
        rules.iter().all(|rule| self.complies_with_rule(rule))
    }

    fn complies_with_rule(&self, rule: &Rule) -> bool {
        if rule.applies(&self) {
            return rule.complies(&self)
        }
        true
    }

    fn correct(&mut self, rules: &Vec<Rule>) -> &Update {
        loop {
            let mut rule_applied = false;
            for rule in rules {
                if rule.applies(&self) && !rule.complies(&self) {
                    self.ensure_rule(rule);
                    rule_applied = true;
                }
            }
            if !rule_applied {
                break;
            }
        }
        return self
    }

    fn ensure_rule(&mut self, rule: &Rule) -> &Self {
        let move_index = self.values.iter().position(|val| *val == rule.before).expect("moveable value exists");
        let move_to_index = self.values.iter().position(|val| *val == rule.after).expect("to move value exists");
        let new_values = if move_to_index == 0 {
            let mut temp: Vec<usize> = Vec::new();
            temp.push(self.values[move_index]);
            temp.append(&mut self.values.clone().iter().filter(|v| **v != rule.before).map(|a| *a).collect::<Vec<_>>());
            temp
        }else {
            let mut temp: Vec<usize> = Vec::new();
            self.values[0..move_to_index].iter().for_each(|v| temp.push(*v));
            temp.push(self.values[move_index]);
            temp.append(&mut self.values[move_to_index..].iter().filter(|v| **v != rule.before).map(|v| *v).collect());
            temp
        };
        self.values = new_values;
        self
    }
}

fn main() {
    let file_name = "input";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");

    let mut split_content = content.split("\n\n");
    let rules: Vec<Rule> = split_content
        .next()
        .expect("Rules exist")
        .split("\n")
        .map(|line| {
            let mut parts = line.split("|");
            let left = parts.next().expect("Has left part");
            let right = parts.next().expect("has right part").trim();
            Rule {
                before: left.parse().expect("first part is number"),
                after: right.parse().expect("second part is number"),
            }
        })
        .collect();

    let mut updates: Vec<Update> = split_content
        .next()
        .expect("Updates exists")
        .split("\n")
        .map(|line| Update {
            values: line
                .split(",")
                .map(|number| number.parse().expect("Updates contain numbers"))
                .collect(),
        })
        .collect();

    let first_part_value: usize = updates
        .iter()
        .filter(|update| update.complies_with_rules(&rules))
        .map(|update| update.get_middle_value())
        .sum();
    println!("Part one: {}", first_part_value);
    let second_part_value: usize = updates
        .iter_mut()
        .filter(|update| !update.complies_with_rules(&rules))
        .map(|update| update.correct(&rules))
        .map(|update| update.get_middle_value())
        .sum();
    println!("Part two: {}", second_part_value);
}
