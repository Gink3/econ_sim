use rand::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufRead, BufReader};
use std::path::Path;


mod producer;
use crate::sim::producer::Producer;

mod trader;
use crate::sim::trader::Trader;

// Sim Class
// Controller of the simulation
// land - generated from a sim config file
// poprate - percent of land, determines population
// tRate - percent of land, determines trader count
// next_tid - tracks next usable trader id
// num_traders - number of traders in a simulation
// traders - array that stores trader data
// prods - array that stores production data
// avg_age - end of simulation metric
#[derive(Debug)]
pub struct Sim<'a> {
    //Statistics of the simulation
    cfg: String,
    land: usize,
    prate: usize,
    trate: usize,
    next_tid: usize,
    next_pid: usize,
    num_traders: usize,
    traders: Vec<Trader<'a>>,
    prods: Vec<Producer<'a>>,
    avg_age: u64,
}

impl<'a> Sim<'a> {
    pub fn new(cpath: String) -> Sim<'static> {
        Sim {
            cfg: cpath,
            land: 0,
            prate: 0,
            trate: 0,
            next_tid: 0,
            next_pid: 0,
            num_traders: 0,
            traders: Vec::new(),
            prods: Vec::new(),
            avg_age: 0,
        }
    }
    // calculates and prints out simulation statistics
    pub fn end_sim(&mut self) {
        self.cal_avg_age();
        println!("Average Age: {}", self.avg_age);
        
    }
    pub fn init(&mut self) {
        // Reference https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
        let path = Path::new(&self.cfg);

        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let mut line = line.unwrap(); // Ignore errors.
            // Devug print statement
            //println!("{}",index);
            match index {
                0 => {
                    line = line.replace("land:", "");
                    self.land = line.parse::<usize>().unwrap();
                }
                1 => {
                    line = line.replace("poprate:","");
                    self.prate = line.parse::<usize>().unwrap();
                }
                2 => {
                    line = line.replace("traderrate:","");
                    self.trate = line.parse::<usize>().unwrap();
                }
                _ => {}
            }
        }

        for _x in 0..10 {
            self.create_trader();
        }
        self.num_traders = self.traders.iter().count();
    }
    fn create_trader(&mut self) {
        let x = 0;
        let t: Trader = Trader::new(self.next_tid,x);
        self.next_tid+=1;

        self.traders.push(t);
    }
    fn cal_avg_age(&mut self) {
        let mut sum: u64 = 0;
        for t in &self.traders {
            sum = sum + u64::from(t.age);
        }
        sum = sum / self.traders.len() as u64;
        self.avg_age = sum;
    }

    pub fn get_land(self) -> usize {
        self.land
    }
    pub fn get_prate(self) -> usize {
        self.prate
    }
    pub fn get_trate(self) -> usize {
        self.trate
    }

}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn reads_land() {
        let mut s = Sim::new(".\\sim_config\\sample.cfg".to_string());
        s.init();
        assert_eq!(10000,s.get_land());
    }
    #[test]
    fn reads_prate() {
        let mut s = Sim::new(".\\sim_config\\sample.cfg".to_string());
        s.init();
        assert_eq!(35,s.get_prate());
    }
    #[test]
    fn reads_trate() {
        let mut s = Sim::new(".\\sim_config\\sample.cfg".to_string());
        s.init();
        assert_eq!(5,s.get_trate());
    }
}