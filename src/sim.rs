use rand::prelude::*;

mod trader;
use crate::sim::trader::Trader;

#[derive(Debug)]
pub struct Sim<'a> {
    //Statistics of the simulation
    avg_age: u8,
    traders: Vec<Trader<'a>>,


}

impl<'a> Sim<'a> {
    pub fn new() -> Sim<'static> {
        Sim {
            avg_age: 0,
            traders: Vec::new(),
        }
    }
    pub fn init(&mut self) {
        for _x in 0..20 {
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