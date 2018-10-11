extern crate itertools;

use itertools::Itertools;
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
        Ok(Config {
            start,
            stop,
            filename,
        })
    }
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

struct Beacon {
    start: &[str],
    stop: &[str],
    longest: usize,
}

//impl Beacon {
//fn new(coll: T)
//}

fn start_stop(item: &str, beacon: &mut Beacon) -> bool {
    fn st_compare(item: &str, st_iter: &Iterator, is_active: &mut bool) -> bool {
        let st_item = *st_iter.next();
        match st_item {
            None => {
                // if st_iter is exhausted then start/stop item collection
                *is_active = *is_active ^ true;
            }
            Some(i) => {
                // if items do not match, reset iterator
                if st_item != Some(item) {
                    reset(st_iter);
                }
            }
        }
        return *is_active; // start/stop collection with this item
    }

    match beacon.active {
        false => st_compare(item, beacon.start, beacon.active),
        true => st_compare(item, beacon.stop, beacon.active),
    }
}

pub fn search<'a>(beacon: Beacon, contents: &'a str) -> Vec<&'a str> {
    //fn remingon(
    let split = contents.lines().flat_map(|line| line.split_whitespace());
    let mut by_longest = split.batching(|it| Some(it.take(longest).map(|x| *x).collect::<Vec<&str>>()));
}

#[cfg(test)]
mod tests {
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
