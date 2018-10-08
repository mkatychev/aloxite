use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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
        Ok(Config { start, stop, filename })
    }
}

fn start_stop(start: &str, stop: &str, item: &str, beacon: &mut bool) -> bool {
    if *beacon == false {
        match item {
            start => {
                // The beacon of Amun Din has been lit!
                *beacon = true;
                return false;
            },
            stop => {
                *beacon = false;
                return false;
            },
            _ => return true,
        }
    }
    return true;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    // conditional binding
    let results = search(&config.start, &config.stop, &contents);
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(start: &str, stop: &str, contents: &'a str) -> Vec<&'a str> {
    let mut beacon: bool = false;
    let split = contents.lines().flat_map(|line| line.split_whitespace());
    split
        .filter(|x| start_stop(start, stop, x, &mut beacon))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result() {
        let contents = "\
Rust:
safe fast productive
Pick all three.";
        assert_eq!(
            vec!["fast", "productive", "Pick"],
            search("safe", "all", contents)
        );
    }
}
