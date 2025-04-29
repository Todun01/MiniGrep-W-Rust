use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let _args: Vec<String> = env::args().collect();
    // println!("{:?}", _args);

    let config = Config::new(&_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e );
        process::exit(1)
        
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let _filetext = fs::read_to_string(config.filename)?;

    Ok(())
}
struct Config {
    query: String,
    filename: String,
}
impl Config{
    fn new(args: &[String]) -> Result <Config, &str>{
        if args.len() < 3{
            return Err("You need to enter more arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename})
    }
}

