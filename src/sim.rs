
use rand::Rng;

use std::collections::HashMap;


// s_price stock price
// traders hashmap info
// traders[0] - money
// trader[1] - amount of stock owned
#[derive(Debug)]
pub struct Sim {
    s_price: i64,
    s_quantity:i64,
    traders: Vec<Vec<i32>>,

}


impl Sim {
    pub fn new() -> Sim {
        Sim {
            s_price: 100,
            s_quantity: 1000,
            traders: Vec::new(),
        }
    }
    // Simulation initialization function
    // nt - number of traders
    pub fn init(&mut self,nt: i32) {
        for i in 0..nt {
            let n = vec![1000,0];
            self.traders.push(n)
        }
    }
    // Main simulation function
    // t - number of time steps
    pub fn run(&mut self, t: i32) {
        for d in 0..t {
            for trader in &self.traders {

            }
        }
    }
    fn trader_action(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..4);
        num
    }
}