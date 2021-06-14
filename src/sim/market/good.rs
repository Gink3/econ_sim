
mod bid;
use crate::sim::market::good::bid::Bid;

// Good is a single item on the market
// name of the good
// price of a single unit of the good
// quantity of goods availble to buy at any 1 time
#[derive(Debug)]
pub struct Good {
    name: String,
    price: usize,
    quantity: usize,
}

impl<'a> Good {
    pub fn new(n: String,p: usize, q: usize) -> Good {
        Good {
            name: n,
            price: p,
            quantity: q,
        }
    }
    // Cancels a bid corresponding to the tid
    pub fn cancel_buy_bid() {
        todo!()
    }
    // Cancels a bid corresponding to the tid
    pub fn cancel_sell_bid() {
        todo!()
    }
    // adds a buy bid inserting from high to low
    pub fn add_buy_bid() {
        todo!()
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
    fn get_highest_buy_bid(self) -> usize {
        todo!()
    }
    // returns lowest sell price
    fn get_lowest_sell_bid(self) -> usize {
        todo!()
    }
}