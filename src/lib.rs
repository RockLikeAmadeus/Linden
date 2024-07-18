use std::error::Error;
use std::fs;

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

        Ok(Config {
            target_string,
            file_path,
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search_case_sensitive(&config.target_string, &contents) {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search_returns_expected_result() {
        let query = "three";
        let contents = "\
1. one
2. two
3. three
4. four
5. five
6. thirty ThReE";

        assert_eq!(vec!["3. three"], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive_search_returns_expected_result() {
        let query = "tHrEe";
        let contents = "\
1. one
2. two
3. three
4. four
5. five
6. thirty ThReE";

        assert_eq!(
            vec!["3. three", "6. thirty ThReE"],
            search_case_insensitive(query, contents)
        );
    }
}
