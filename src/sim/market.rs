
// Example Table
// Good, Price, Quantity
// String, usize, usize
#[derive(Debug)]
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
}

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
}