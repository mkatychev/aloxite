use std::str::Chars;

struct Resettable<'a> {
    trigger: &'a str,
    pub iter: Chars<'a>
}

impl<'a> Resettable<'a> {
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

pub struct Beacon {
    pub start: Resettable,
    pub stop: Resettable,
    pub longest: usize,
    pub active: bool,
}

impl Beacon {
    pub fn new(start: , stop: &[char]) -> Beacon {
        Beacon {
            start: Resettable::new(start),
            stop: Resettable::new(stop),
            longest: match start.len() > stop.len() {
                true => start.len(),
                false => stop.len(),
            },
        }
    }
}

pub fn start_stop(item: &str, beacon: &mut Beacon) -> bool {
    pub fn st_compare(item: &char, st_iter: &mut Resettable, is_active: &mut bool) -> bool {
        let st_item = *st_iter.next();
        match st_item {
            None => {
                // if st_iter is exhausted then start/stop item collection
                *is_active = *is_active ^ true;
            }
            Some(i) => {
                // if items do not match, reset iterator
                if st_item != Some(item) {
                    st_iter.reset();
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

fn test() {
    let mut s = vec!["alox"].into_iter(); // simulate args passed through CLI
    assert_eq!("alox", s.next().unwrap());
    let mut test = Resettable::new(s.by_ref()).unwrap();
    assert_eq!('a', test.next());
    assert_eq!('l', test.next());
    assert_eq!('o', test.next());
    assert_eq!('x', test.next());
    assert_eq!(None, test.next());
    test.reset();
    assert_eq!('a', test.next());
}
