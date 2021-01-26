use rand::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::path::Path;

use std::io::BufReader;
use std::io::Error;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Trader<'a> {
    age: u8,
    first: String,
    last: String,
    bank: HashMap<&'a str, i64>,
    holdings: HashMap<&'a str, i64>,
} 

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut lines = content.lines();
    lines.nth(line_num).expect("No line found at that position")
}

impl<'a> Trader<'a> {
    pub fn new(a: u8) -> Trader<'a> {
        let mut t = Trader {
            age: a,
            first: String::new(),
            last: String::new(),
            bank: HashMap::new(),
            holdings: HashMap::new(),
        };
        t.init();
        t
    }
    fn init(&mut self) {
        self.select_first();
        self.select_last();
        self.bank.insert("USD",1000);
    }
    fn select_first(&mut self) {
        let mut rng = thread_rng();
        let r: u32 = rng.gen_range(0..4946);
        let p = Path::new("assets/first-names.txt");
        let line = get_line_at(p, r as usize);

        let name = line.unwrap();

        self.first = name;

    }
    fn select_last(&mut self) {
        let mut rng = thread_rng();
        let r: u32 = rng.gen_range(0..88800);
        let p = Path::new("assets/last-names.txt");
        let line = get_line_at(p, r as usize);

        let name = line.unwrap();

        self.last = name;

    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
}