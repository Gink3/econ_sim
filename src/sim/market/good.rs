use serde::{Serialize, Deserialize};

mod bid;
use crate::sim::market::good::bid::Bid;

// Good is a single item on the market
// name of the good
// price of a single unit of the good
// quantity of goods availble to buy at any 1 time
#[derive(Debug, Serialize, Deserialize)]
pub struct Good {
    name: String,
    price: usize,
    quantity: usize,
    sell_bids: Vec<Bid>,
    buy_bids: Vec<Bid>,
}

impl<'a> Good {
    pub fn new(n: String,p: usize, q: usize) -> Good {
        Good {
            name: n,
            price: p,
            quantity: q,
            sell_bids: Vec::new(),
            buy_bids: Vec::new(),
        }
    }
    // Cancels a bid corresponding to the tid
    pub fn cancel_buy_bid(&mut self, t: usize) {
        let mut index = 0;
        for b in &self.buy_bids {
            if b.get_tid() == t {
                break;
            } else {
                index+=1;
            }
        }
        self.buy_bids.remove(index);
    }
    // Cancels a bid corresponding to the tid
    pub fn cancel_sell_bid_(&mut self, t: usize) {
        let mut index = 0;
        for b in &self.buy_bids {
            if b.get_tid() == t {
                break;
            } else {
                index+=1;
            }
        }
        self.sell_bids.remove(index);
    }
    // adds a buy bid inserting from high to low
    pub fn add_buy_bid(&mut self, p:f32,t: usize, q:usize) {
        let bid = Bid::new(p,t,q);
        // check if list is empty
        if self.buy_bids.is_empty() {
            self.buy_bids.push(bid);
        } else {
            // checks for where a bid should be placed
            let mut index = 0;
            for b in &self.buy_bids {
                if p > b.get_price() {
                    break;
                } 
                index+=1;
            }
            self.buy_bids.insert(index, bid);
        }
    }
    // adds a sell bid inserting from low to high
    pub fn add_sell_bid() {
        todo!()
    }
    // updates display price
    pub fn update_price() {
        todo!()
    }
    // resolves any compatible bids
    pub fn resolve_bids() {

    }
    // order bids may be unneccessary if additions are inserted in order
    fn order_sell_bids(&mut self) {
        todo!()
    }
    // order bids may be unneccessary if additions are inserted in order
    fn order_buy_bids(&mut self) {
        todo!()
    }
    // returns highest buy price
    fn get_highest_buy_bid(self) -> f32 {
        if self.buy_bids.is_empty() {
            -1.0
        } else {
            self.buy_bids[0].get_price()
        }
    }
    // returns lowest sell price
    fn get_lowest_sell_bid(self) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn add_buy_bid() {
        let mut g = Good::new("Test".to_string(),70,70);

        g.add_buy_bid(69.0,1,1);
        assert_eq!(69.0,g.get_highest_buy_bid())
    }
    #[test]
    fn add_two_buy_bids() {
        let mut g = Good::new("Test".to_string(),70,70);

        g.add_buy_bid(69.0,1,1);
        g.add_buy_bid(76.0,1,1);
        assert_eq!(76.0,g.get_highest_buy_bid())
    }
}