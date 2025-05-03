use std::env;
use std::process;
use MiniGrep::Config;
fn main() {
    let _args: Vec<String> = env::args().collect();
    // println!("{:?}", _args);

    let config = Config::new(&_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = MiniGrep::run(config) {
        println!("Application error: {}", e );
        process::exit(1)
        
    };
}

