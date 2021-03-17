use rand::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::fs::{File,OpenOptions};
use std::io::{self, Write};

// Producer Class
// pid - producer id
// tid - trader id of owner
// cost - cost calculated by the factors of the production from the controller
// prod - hashmap with product name, amount pairs
// needs - daily needs to proudce goods if not met no production occurs
// 
#[derive(Debug)]
pub struct Producer<'a> {
    pid: usize,
    tid: usize,
    cost: usize,
    prod: HashMap<&'a str, i64>,
    needs: HashMap<&'a str, i64>,
    holds: HashMap<&'a str, i64>,
}

impl<'a> Producer<'a> {
    // Controller needs to provide p and c
    // p - producer id
    // c - cost
    // holds - current holdings
    pub fn new(p: usize,t: usize,c: usize) -> Producer<'a> {
        let mut p = Producer {
            pid: p,
            tid: t,
            cost: c,
            prod: HashMap::new(),
            needs: HashMap::new(),
            holds: HashMap::new(),
        };
        p.init();
        p
    }

    fn init(&mut self) {
        let _r1 = fs::remove_file(".\\logs\\producer.log");
        let _r2 = File::create(".\\logs\\producer.log");
        let mut rng = thread_rng();
        let r: u32 = rng.gen_range(0..100);
        match r {
            0 ..= 11 => {
                self.prod.insert("Steel",20);
                self.needs.insert("Iron",20);
                self.needs.insert("Coal",20);
            },
            _ => {
                let mut s = "Prodution Selection r:".to_string();
                s.push_str(&r.to_string());
                log_error(s);
            }
        }
    }
    
    pub fn get_pid(self) -> usize {
        self.pid
    }

    pub fn get_tid(self) -> usize {
        self.tid
    }
    pub fn get_cost(self) -> usize {
        self.cost
    }

}

fn log_error(e: String) {
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(".\\logs\\producer.log")
    .unwrap();

    writeln!(file, "Error: {}",e).expect("Unable to write to Producer Log");
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn has_pid() {
        let p = Producer::new(1,0,1000);
        assert_eq!(1,p.get_pid());
    }


}