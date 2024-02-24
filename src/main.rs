use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_string = &args[1];
    let file_path = &args[2];
    println!("Searching for {target_string} in file {file_path}");
}