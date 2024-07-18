use std::env;
use std::process;

use linden::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("There was a problem with the arguments provided: {err}");
        process::exit(1);
    });
    //println!("Searching for {} in file {}.", config.target_string, config.file_path);
    if let Err(e) = linden::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
