use rand::prelude::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::io::BufReader;
use std::io::Error;

#[derive(Debug)]
pub struct Trader {
    age: u8,
    first: String,
    last: String,
} 

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}

impl Trader {
    pub fn new(a: u8) -> Trader {
        let mut t = Trader {
            age: a,
            first: String::new(),
            last: String::new(),
        };
        t.select_first();

        t
    }
    fn select_first(&mut self) {
        let mut rng = thread_rng();
        let r: u32 = rng.gen_range(0..164385);
        let p = Path::new("assets/first-names.txt");
        let line = get_line_at(p, r as usize);

        let name = line.unwrap();

        self.first = name;

    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
}