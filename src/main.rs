use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_string = &args[1];
    let file_path = &args[2];
    println!("Searching for {target_string} in file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("An error occurred when attempting to read from the specified file");
    println!("Contents read as {contents}");
}