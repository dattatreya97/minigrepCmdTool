use std::env;
use std::process;

mod lib;
use lib::Config;



fn main() {
    let args:Vec<String> = env::args().collect();

    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config){
        println!("Application Error");
        process::exit(1);
    }
}


