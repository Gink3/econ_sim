
use std::collections::HashMap;

// Producer Class
// pid - producer id
// tid - trader id of owner
// cost - cost calculated by the factors of the production from the controller
// prod - hashmap with product name, amount pairs
// needs - daily needs to proudce goods if not met no production occurs
// 
#[derive(Debug)]
pub struct Producer<'a> {
    pid: usize,
    tid: usize,
    cost: usize,
    prod: HashMap<&'a str, i64>,
    needs: HashMap<&'a str, i64>,
    holds: HashMap<&'a str, i64>,
}

impl<'a> Producer<'a> {
    // Controller needs to provide p and c
    // p - producer id
    // c - cost
    // holds - current holdings
    pub fn new(p: usize,t: usize,c: usize) -> Producer<'a> {
        Producer {
            pid: p,
            tid: t,
            cost: c,
            prod: HashMap::new(),
            needs: HashMap::new(),
            holds: HashMap::new(),
        }
    }

    
    pub fn get_pid(self) -> usize {
        self.pid
    }

    pub fn get_tid(self) -> usize {
        self.tid
    }
    pub fn get_cost(self) -> usize {
        self.cost
    }

}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn has_pid() {
        let p = Producer::new(1,0,1000);
        assert_eq!(1,p.get_pid());
    }


}