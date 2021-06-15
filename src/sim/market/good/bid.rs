
#[derive(Debug,Clone,Copy)]
pub struct Bid {
    price: f32,
    tid: usize,
    quantity: usize,
}

impl Bid {
    pub fn new(p: f32, t: usize, q: usize) -> Bid {
        Bid {
            price: p,
            tid: t,
            quantity: q,
        }
    }
    pub fn get_tid(self) -> usize {
        self.tid
    }
}