extern crate argparse;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod args;
mod search;

pub struct Config {
    pub start: String,
    pub stop: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let start = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a start string"),
        };
        let stop = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a stop string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        Ok(Config {
            start,
            stop,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let mut f = File::open(config.filename)
                    .map_err(|e| "Problem opening file")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .map_err(|e| "Problem reading file")?;
    // conditional binding
    let mut beacon = search::Beacon::new(&config.start, &config.stop)?;
    let results = search::search(&mut beacon, &contents);
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
