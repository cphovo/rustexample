use std::{env, process};

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // the program’s name takes up the first value
    // let config = Config::new(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

