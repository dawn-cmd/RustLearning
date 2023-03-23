use minigrep::Config; // Import the Config struct from the minigrep module
use std::{env, process}; // Import standard Rust modules

fn main() {
    let config = Config::new(env::args()) // Create a new Config object from command line arguments
        .unwrap_or_else(|err| { // If there was an error creating the object, handle it
            eprintln!("Problem parsing arguments: {err}"); // Print the error message to standard error
            process::exit(1); // Exit the program with an error code
        });
    
    if let Err(e) = minigrep::run(config) { // If there was an error running the minigrep::run function with the config object
        eprintln!("Application Error: {}", e); // Print the error message to standard error
        process::exit(1); // Exit the program with an error code
    }
}
