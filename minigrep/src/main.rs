use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // the program’s name takes up the first value
    let config = parse_config(&args);
    println!("{}, {}", config.query, config.file_path);
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a String,
}

fn parse_config<'a>(args: &'a [String]) -> Config<'a> {
    let query = &args[1];
    let file_path = &args[2];
    Config { query, file_path }
}
