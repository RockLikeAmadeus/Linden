use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_input(&args);
    println!("Searching for {} in file {}", config.target_string, config.file_path);
    let contents = fs::read_to_string(config.file_path)
        .expect("An error occurred when attempting to read from the specified file");
    println!("Contents read as {contents}");
}

struct Config {
    target_string: String,
    file_path: String,
}

fn parse_input(args: &[String]) -> Config {
    let target_string = args[1].clone();
    let file_path = args[2].clone();

    Config { target_string, file_path }
}