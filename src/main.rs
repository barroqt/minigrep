use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // read arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if let instead of unwrap because run only returns ()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e); // print to the standard error stream
        process::exit(1);
    }

}
