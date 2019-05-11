use std::str::Chars;

#[derive(Clone)]
pub struct Resettable<'a> {
    trigger: &'a str,
    pub iter: Chars<'a>
}

// Resettable is for partial matches of start || stop strings
impl<'a> Resettable<'a> {
    // string has to have the same lifetime as the resettable
    pub fn new(arg: &'a str) -> Result<Resettable<'a>, &'static str> {
        let trigger: &'a str = arg;
        let mut iter = match trigger.len() {
            0 => return Err("input cannot be empty"),
            _ => trigger.chars(),
        };
        Ok(Resettable { trigger, iter })
    }

    pub fn reset(&mut self) {
        self.iter = self.trigger.chars();
    }

    pub fn next(&mut self) -> Option<char> { self.iter.next() }
}

pub struct Beacon<'a> {
    pub start: Resettable<'a>,
    pub stop: Resettable<'a>,
    pub longest: usize,
    // We have matched `start` and are working towards `stop`
    pub active: bool,
}

impl<'a> Beacon<'a> {
    pub fn new(start: &'a str, stop: &'a str) -> Result<Beacon<'a>, &'static str> {
        Ok(Beacon {
            start: Resettable::new(start)?,
            stop: Resettable::new(stop)?,
            longest: match start.len() > stop.len() {
                true => start.len(),
                false => stop.len(),
            },
            active: false,
        })
    }
}

fn st_compare(item: &str, st_iter: &mut Resettable, is_active: &mut bool) -> bool {
    let cloned_iter = st_iter.clone();
    let to_match = cloned_iter.iter.collect::<String>();
    if to_match == item {
        *is_active ^= true;
    }
    *is_active
    // let st_item = st_iter.next();
    // match st_item {
    //     None => {
    //         // if st_iter is exhausted then start/stop item collection
    //         *is_active ^= true;
    //     }
    //     Some(i) => {
    //         // if items do not match, reset iterator
    //         // can dereference because not an Option any more
    //         if i != *item {
    //             st_iter.reset();
    //         }
    //     }
    // }
    // *is_active // start/stop collection with this item
}


pub fn start_stop(item: &str, beacon: &mut Beacon) -> bool {

    match beacon.active {
        false => st_compare(item, &mut beacon.start, &mut beacon.active),
        true => st_compare(item, &mut beacon.stop, &mut beacon.active),
    }
}


// Takes a Beacon impl to iterate on a per line basis
pub fn search<'a>(beacon: &mut Beacon, contents: &'a str) -> Vec<&'a str> {
    //fn remmingon(

    let mut start_index: Option<usize> = None;
    let mut stop_index: Option<usize> = None;

    let split = contents.split_whitespace();
        // .map(|word| {println!("{}", word); word})
        // .map(|word| start_stop(&word, beacon))
        // .collect::<Vec<bool>>();

    let mut past_active = beacon.active;
    for (i, word) in split.enumerate() {
        let active = start_stop(&word, beacon);
        if active && !past_active {
            start_index = Some(i + 1);
        }
        if !active && past_active {
            stop_index = Some(i - 1);
        }
        past_active = active;
    };

    let second_split = contents.split_whitespace();
    second_split.collect::<Vec<&str>>()[start_index.unwrap()..stop_index.unwrap() + 1].into()

    // println!("{:?}, {:?}", start_index, stop_index);
        // .flat_map(|c| c.split_whitespace())
        // .batching(
        //     |it| Some(
        //         it.take(longest).map(|x| *x)
        //             .collect::<Vec<&str>>()
        //     )
        // );
    // vec![]
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
            search(&mut Beacon::new("safe", "all").unwrap(), contents)
        );
    }

    #[test]
    fn test() {
        let mut s = vec!["alox"].into_iter(); // simulate args passed through CLI
        assert_eq!("alox", s.next().unwrap());
        // let mut test = Resettable::new(s.by_ref()).unwrap();
        // assert_eq!('a', test.next());
        // assert_eq!('l', test.next());
        // assert_eq!('o', test.next());
        // assert_eq!('x', test.next());
        // assert_eq!(None, test.next());
        // test.reset();
        // assert_eq!('a', test.next());
    }

}
