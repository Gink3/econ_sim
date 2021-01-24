use rand::prelude::*;

mod trader;
use crate::sim::trader::Trader;

#[derive(Debug)]
pub struct Sim {
    //Statistics of the simulation
    avg_age: u8,
    traders: Vec<Trader>,


}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            avg_age: 0,
            traders: Vec::new(),
        }
    }
    pub fn init(&mut self) {
        for x in 0..100 {
            self.create_trader();
        }
    }
    fn create_trader(&mut self) {
        let mut rng = thread_rng();
        let x = rng.gen_range(30..80);
        let t: Trader = Trader::new(x);


        self.traders.push(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}