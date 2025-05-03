use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
     let _filetext = fs::read_to_string(config.filename)?;

    // println!("File text: {}", _filetext);
    for line in search(&config.query, &_filetext){
        println!{"'{}' found at: {}", &config.query, line}
    }
    
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config{
    pub fn new(args: &[String]) -> Result <Config, &str>{
        if args.len() < 3{
            return Err("You need to enter more arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename})
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
        if line.contains(&_query){
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
    assert_eq!(vec!["second line here"], search_case_insensitivity(query, &filetext.to_lowercase()));
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
        search_case_insensitivity(query, &filetext.to_lowercase())
    );

 }
