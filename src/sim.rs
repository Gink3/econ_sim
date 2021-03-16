use rand::prelude::*;

mod producer;
use crate::sim::producer::Producer;

mod trader;
use crate::sim::trader::Trader;

#[derive(Debug)]
pub struct Sim<'a> {
    //Statistics of the simulation
    next_tid: usize,
    num_traders: usize,
    avg_age: u64,
    traders: Vec<Trader<'a>>,
    prods: Vec<Producer<'a>>,

}

impl<'a> Sim<'a> {
    pub fn new() -> Sim<'static> {
        Sim {
            next_tid: 0,
            num_traders: 0,
            avg_age: 0,
            traders: Vec::new(),
            prods: Vec::new(),
        }
    }
    // calculates and prints out simulation statistics
    pub fn end_sim(&mut self) {
        self.cal_avg_age();
        println!("Average Age: {}", self.avg_age);
        
    }
    pub fn init(&mut self) {
        for _x in 0..20 {
            self.create_trader();
        }
        self.num_traders = self.traders.iter().count();
    }
    fn create_trader(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(30..80);
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

}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
}