use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let _filetext = fs::read_to_string(config.filename)?;

    let search_results = if config.case_sensitive{
        search(&config.query, &_filetext)
    } else{
        search_case_insensitivity(&config.query, &_filetext)
    };
    for line in search_results{
        println!{"'{}' found at: {}", &config.query, line}
    }
    
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}
impl Config{
    pub fn new(args: &[String]) -> Result <Config, &str>{
        if args.len() < 3{
            return Err("You need to enter more arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive})
    }
}

pub fn search< 'a>(_query: &str, _filetext: & 'a str) -> Vec<& 'a str>{
    let mut results: Vec<&str> = Vec::new();
    for line in _filetext.lines() {
        if line.contains(_query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitivity< 'a>(query: &str, filetext: & 'a str) -> Vec<& 'a str>{
    let mut results: Vec<&str> = Vec::new();
    let _query = query.to_lowercase();
    for line in filetext.lines(){
        if line.to_lowercase().contains(&_query){
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;
}
 #[test]
 fn case_sensitive(){
    let query = "Here";
    let filetext = "/
this is the first line
second line here
ah the third line, finally";
    assert_eq!(vec!["second line here"], search_case_insensitivity(query, filetext));
 }

 #[test]
 fn case_insensitive(){
    let query = "Ine";
    let filetext = "/
this is the first line
second line here
ah the third line, finally";
    assert_eq!(
        vec![
        "this is the first line",
        "second line here",
        "ah the third line, finally"],
        search_case_insensitivity(query, filetext)
    );

 }
