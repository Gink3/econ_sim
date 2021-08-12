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
    // Checks market for good by string
    // Return Codes
    // -1 - Not Found
    // 0 - Market is empty
    // 1 - Good Found
    pub fn has_good(&self, n:String) -> i32 {
        if self.goods.is_empty() {
            return 0
        }
        for g in &self.goods {
            if g.get_name() == n {
                return 1
            }
        }
        return -1
    }
    // resolves any compatible bids and updates the corresponding quantities
    pub fn step1() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn market_empty() {
        let m = Market::new();
        assert_eq!(0,m.has_good("Wood".to_string()));
    }

    #[test]
    fn can_add_good() {
        let mut m = Market::new();
        m.add_good_to_market("Wood".to_string(),75, 1000);
        assert_eq!(1,m.has_good("Wood".to_string()));
    }

}