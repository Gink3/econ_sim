use serde::{Serialize, Deserialize};

mod good;
use crate::sim::market::good::Good;
// Example Table
// Good, Price, Quantity
// String, usize, usize
#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    goods: Vec<Good>,
}

impl<'a> Market {
    pub fn new() -> Market {
        Market {
            goods: Vec::new(),
        }
    }
    // Wrapper for Good constructor
    pub fn add_good_to_market(&mut self, n:String,p: usize, q: usize) {
        self.goods.push(Good::new(n,p,q));
    }
    // resolves any compatible bids and updates the corresponding quantities
    pub fn step1() {
        todo!()
    }
}

