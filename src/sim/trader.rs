
use rand::Rng;

#[derive(Debug)]
pub struct Trader {
    money: u64,
    stock: u64,
}

impl Trader {
    pub fn new() -> Trader {
        Trader {
            money: 1000,
            stock: 0,
        }
    }
    
    pub fn trader_action(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..4);
        // println!("{}",num);
        num
    }
    pub fn buy_stock(&mut self, p:u64) {
        self.money -= p;
        self.stock += 1;
    }

    pub fn sell_stock(&mut self, p:u64) {
        self.money += p;
        self.stock -= 1;
    }

    // Takes a u32 and adds it to 
    pub fn get_money(&self) -> u64 {
        self.money
    }

    pub fn get_stock(&self) -> u64 {
        self.stock
    }

    pub fn change_stock(&mut self, a:u64) {
        self.stock += a;
    }

}