use std::{fs::File, io::Read};

fn main() {
    let file_name = "example";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");
    let line_length = content.clone().lines().next().expect("Has line").len();
    let chars: Vec<_> = content.clone().chars().filter(|c| !c.is_ascii_whitespace()).map(|c| c.to_string()).collect();

    println!("{} {:?}", line_length, chars);
}
