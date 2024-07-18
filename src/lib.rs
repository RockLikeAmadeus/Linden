use std::fs;
use std::error::Error;

pub struct Config {
    pub target_string: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided. Expected string and file path.");
        }
        let target_string = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { target_string, file_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //println!("Contents read as {contents}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "three";
        let contents = "\
1. one
2. two
3. three
4. four
5. five";

        assert_eq!(vec!["3. three"], search(query, contents));
    }
}