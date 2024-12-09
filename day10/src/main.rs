use std::{fs::File, io::Read};

fn main() {
    let file_name = "input";
    let mut input_file = File::open(file_name).expect("Can open file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Can read file");
    println!("{}", content);
}
