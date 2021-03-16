
use std::collections::HashMap;

// Producer Class
// pid - producer id
// cost - cost calculated by the factors of the production from the controller
// prod - hashmap with product name, amount pairs
// needs - daily needs to proudce goods if not met no production occurs
// 
#[derive(Debug)]
pub struct Producer<'a> {
    pid: usize,
    cost: usize,
    prod: HashMap<&'a str, i64>,
    needs: HashMap<&'a str, i64>,
}

impl<'a> Producer<'a> {
    // Controller needs to provide p and c
    // p - producer id
    // c - cost
    // 
    pub fn new(p: usize,c: usize) -> Producer<'a> {
        Producer {
            pid: p,
            cost: c,
            prod: HashMap::new(),
            needs: HashMap::new(),
        }
    }
    pub fn get_pid(self) -> usize {
        self.pid
    }


}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn has_pid() {
        let p = Producer::new(1,1000);
        assert_eq!(1,p.get_pid());
    }


}