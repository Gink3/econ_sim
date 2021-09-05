
use rand::Rng;

use std::collections::HashMap;

mod trader;
use crate::sim::trader::Trader;

// s_price stock price
// traders hashmap info
// traders[0] - money
// trader[1] - amount of stock owned
#[derive(Debug)]
pub struct Sim {
    s_price: u64,
    s_quantity:u64,
    traders: Vec<Trader>,
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
        for _i in 0..nt {
            let t = Trader::new();
            self.traders.push(t);
        }
    }
    // Main simulation function
    // t - number of time steps
    pub fn run(&mut self, days: i32) {
        for _d in 0..days {
            for t in self.traders.iter_mut() {
                let action = t.trader_action();
                match action {
                    0 => (),    // Do Nothing
                    1 => {
                        if t.get_money() < self.s_price {

                        } else {
                            t.buy_stock(self.s_price);
                            self.s_price += 1;
                            self.s_quantity -= 1;
                        }  
                    },    // Buy
                    2 => (),    // Sell
                    3 => (),    // Trade
                    _ => (),    // Error
                }
            }
        }
    }

}