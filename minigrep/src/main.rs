use minigrep::Config;
use std::{self, env, process};
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // println!("Search for {}", config.query);
    // println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
