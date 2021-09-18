
use rand::Rng;
use std::collections::HashMap;
use num_format::{Locale, ToFormattedString};


mod trader;
use crate::sim::trader::Trader;

// s_price stock price
// traders hashmap info
// traders[0] - money
// trader[1] - amount of stock owned
#[derive(Debug)]
pub struct Sim {
    s_price: f64,
    s_quantity:u64,
    traders: Vec<Trader>,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            s_price: 100.0,
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
    pub fn run(&mut self, days: i32, dividend_period: i32) {
        for _d in 0..days {
            // TODO add stock dividend payout
            if _d % dividend_period == 0 {
                self.payout_dividends(1200, self.s_quantity);
            }
            for t in self.traders.iter_mut() {
                let action = t.trader_action();
                match action {
                    0 => (),    // Do Nothing
                    1 => {
                        if t.get_money() < self.s_price {

                        } else {
                            t.buy_stock(self.s_price);
                            self.s_price += 1.0;
                            self.s_quantity -= 1;
                        }  
                    },    // Buy
                    2 => {
                        if t.get_stock() < 1 {

                        } else {
                            t.sell_stock(self.s_price);
                            self.s_price -= 1.0;
                            self.s_quantity += 1;
                        }
                    },    // Sell
                    _ => (),    // Error
                }
            }
        }
        for t in self.traders.iter_mut() {
            t.set_trade_freq(days);
        }
        println!("Days run: {}", days.to_formatted_string(&Locale::en));
    }

    // iterates over all traders
    // and adds a share of corporate profits
    fn payout_dividends(&mut self, profits: u64,total_shares: u64) {
        let profits_per_share  = (profits / total_shares) as f64;
        for t in self.traders.iter_mut() {
            t.add_money(t.get_stock() as f64 * profits_per_share);
        }
    }
}