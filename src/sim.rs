use rand::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

mod producer;
use crate::sim::producer::Producer;

mod trader;
use crate::sim::trader::Trader;

mod market;
use crate::sim::market::Market;

// Sim Class
// Controller of the simulation
// land - generated from a sim config file
// poprate - percent of land, determines population
// tRate - percent of land, determines trader count
// next_tid - tracks next usable trader id
// num_traders - number of traders in a simulation
// traders - array that stores trader data
// prods - array that stores production data
// mp - market prices for all goods (item name, price)
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
    mp: HashMap<&'a str, u64>,
    avg_age: u64,
    daily_order: Vec<usize>,
    market: Market,
}

impl<'a> Sim<'a> {
    pub fn new(cpath: String) -> Sim<'static> {
        Sim {
            cfg: cpath,
            land: 0,
            prate: 0,
            trate: 0,
            next_tid: 1,
            next_pid: 0,
            num_traders: 0,
            traders: Vec::new(),
            prods: Vec::new(),
            mp: HashMap::new(),
            avg_age: 0,
            daily_order: Vec::new(),
            market: Market::new(),
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
        
        let f = File::open(&path);

        let mut f = match f {
            Ok(file) => file,
            Err(error) => {panic!("{:?}", error);},
        };
        

        let reader = BufReader::new(f);

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
            self.create_traders();
        }
        self.num_traders = self.traders.iter().count();
    }
    // Creates the inital vector of traders
    fn create_traders(&mut self) {
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
    // Returns the index in the vector of trader
    // given t - the tid of a given trader
    // Returns as an Option so you must unwrap
    fn get_index_tid(self, t: usize) -> Option<usize> {
        for (index, trader) in self.traders.into_iter().enumerate() {
            if trader.get_tid() == t {
                return Some(index)
            }
        }
        None
    }
    // Main function of this class, runs the simulation
    // d - number of days the simulation runs for
    pub fn run(&mut self, d: usize) {
        for day in 0..d {
            self.gen_d_order();
            for t in 0..self.num_traders {
                //traders action code goes here
            }

            self.daily_order.clear();
        }
    }

    // Fills vector with a psuedo random order of indices
    // used for iterating over trader objects at random
    fn gen_d_order(&mut self) {
        let mut rng = thread_rng();
        for t in 0..self.num_traders {
            let mut added = false;
            //println!("{:?}",t);
            while !added {
                let r: usize = rng.gen_range(0..self.num_traders);
                //println!("{:?}",r);
                if self.daily_order.contains(&r) {
                    added = false;
                } else {
                    added = true;
                    self.daily_order.push(r);
                }
                //println!("{:?}",self.daily_order);
            }

        }
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
    // NOTE tid is initially 1 greater than index
    #[test]
    fn finds_trader_index() {
        let mut s = Sim::new(".\\sim_config\\sample.cfg".to_string());
        s.init();
        assert_eq!(s.get_index_tid(5).unwrap(), 4);
    }
}