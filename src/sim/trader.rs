
use rand::Rng;

// money - monetary units
// stock - integer stock values
// num_t - number of transactions a trader takes
// trade_freq - how often a trade is made in trades/day
//
#[derive(Debug)]
pub struct Trader {
    money: f64,
    stock: u64,
    num_t: i32,
    trade_freq: f64,
}

impl Trader {
    pub fn new() -> Trader {
        Trader {
            money: 1000.0,
            stock: 0,
            num_t: 0,
            trade_freq: 0.0,
        }
    }
    
    pub fn trader_action(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..3);
        // println!("{}",num);
        num
    }
    pub fn buy_stock(&mut self, p:f64) {
        self.money -= p;
        self.stock += 1;
        self.num_t += 1;
    }

    pub fn sell_stock(&mut self, p:f64) {
        self.money += p;
        self.stock -= 1;
        self.num_t += 1;
    }
    pub fn get_money(&self) -> f64 {
        self.money
    }
    pub fn add_money(&mut self, change: f64) {
        self.money += change;
    }

    pub fn get_stock(&self) -> u64 {
        self.stock
    }

    pub fn change_stock(&mut self, a:u64) {
        self.stock += a;
    }
    
    // gets number of trades
    pub fn get_num_t(self) -> i32 {
        self.num_t
    }
    pub fn set_trade_freq(&mut self, days: i32) {
        self.trade_freq = self.num_t as f64 / (days as f64); 
    }
}