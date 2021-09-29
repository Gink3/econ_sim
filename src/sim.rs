
use rand::Rng;
use std::collections::HashMap;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;

use plotters::prelude::*;

mod trader;
use crate::sim::trader::Trader;

// s_price stock price
// traders hashmap info
// traders[0] - money
// trader[1] - amount of stock owned
#[derive(Debug, Deserialize)]
pub struct Sim {
    s_price: f64,
    s_quantity:u64,
    traders: Vec<Trader>,
    s_history: Vec<f64>,
    s_max: f64,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            s_price: 100.0,
            s_quantity: 1000,
            traders: Vec::new(),
            s_history: Vec::new(),
            s_max: 0.0,
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
            //End of Day

            // Track end of day price history
            self.s_history.push(self.s_price);

            // Tracks max price of the stock
            if self.s_price > self.s_max {
                self.s_max = self.s_price;
            }
            
        }
        for t in self.traders.iter_mut() {
            t.set_trade_freq(days);
        }
        self.plot_stock_price(days);
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

    //Plots and illiustrates stock price
    fn plot_stock_price(&self, days: i32) {
        let root_area = BitMapBackend::new("images/s_hist.png", (600, 400))
            .into_drawing_area();
            root_area.fill(&WHITE).unwrap();

        let margin = self.s_max * 1.1;

        let mut ctx = ChartBuilder::on(&root_area)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption("Stock daily close price", ("sans-serif", 40))
                .build_cartesian_2d(0..days, 50.0..margin)
                .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(
            LineSeries::new(
                (0..).zip(self.s_history.iter()).map(|(day, price)| {
                    (day, *price)
                }),
                &RED,
            )
            ).unwrap();
        println!("Succesfully Plotted s_history");
    }
}