extern crate start_end_regex;

use std::env;
use std::process;

use start_end_regex::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = start_end_regex::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
