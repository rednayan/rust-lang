use std::error::Error;
use std::fs;
use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case : bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        return Ok(Config { query,file_path,ignore_case});
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents){
        println!("{line}");
    }
    return Ok(());
}

pub fn search<'a>(query: &str, contents : &'a str) -> Vec<&'a str> {
    let mut results:Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_insensitive<'a>(query:&str,contents : &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results:Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    return results;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents= "\
Rust : 
safe, fast, productive
Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents= "\
Rust:
safe, fast, productive
Pick three,
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents));
    }
}
