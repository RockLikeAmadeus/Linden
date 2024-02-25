use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("There was a problem with the arguments provided: {err}");
        process::exit(1);
    });
    println!("Searching for {} in file {}", config.target_string, config.file_path);
    let contents = fs::read_to_string(config.file_path)
        .expect("An error occurred when attempting to read from the specified file");
    println!("Contents read as {contents}");
}

struct Config {
    target_string: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided. Expected string and file path.");
        }
        let target_string = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { target_string, file_path })
    }
}