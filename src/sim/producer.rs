use rand::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::fs::{File,OpenOptions};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
// Producer Class
// pid - producer id
// ptype - code for different type of good produced
//      0 - undefined
//      1 - primary
//      2 - Intermediate
//      3 - final
// ownerid - trader id of owner
// cost - cost calculated by the factors of the production from the controller
// prod - hashmap with product name, amount pairs
// needs - daily needs to proudce goods if not met no production occurs
// 
#[derive(Debug, Serialize)]
pub struct Producer<'a> {
    pid: usize,
    ptype: u8,
    ownerid: usize,
    cost: i64,
    prod: HashMap<&'a str, u64>,
    needs: HashMap<&'a str, u64>,
    holds: HashMap<&'a str, u64>,
}

impl<'a> Producer<'a> {
    // Controller needs to provide p and c
    // p - producer id
    // t - trader id of owner
    // c - cost
    // holds - current holdings
    pub fn new(p: usize,t: usize,c: i64) -> Producer<'a> {
        let mut pro = Producer {
            pid: p,
            ptype: 0,
            ownerid: t,
            cost: c,
            prod: HashMap::new(),
            needs: HashMap::new(),
            holds: HashMap::new(),
        };
        pro.init();
        pro
    }

    fn init(&mut self) {
        // Clears previous logs
        let _r1 = fs::remove_file(".\\logs\\producer.log");
        let _r2 = File::create(".\\logs\\producer.log");

        // Creates the rng thread and generates our random number
        let mut rng = thread_rng();
        let r: u32 = rng.gen_range(0..11);
        // Selection for the production type and sets production rates and needs
        match r {
            0 ..= 11 => {
                self.prod.insert("Steel",20);
                self.needs.insert("Iron",20);
                self.needs.insert("Coal",20);
            },
            _ => {
                // Error handling for unknown number
                let mut s = "Prodution Selection r:".to_string();
                s.push_str(&r.to_string());
                log_error(s);
            }
        }
    }
    // Returns Values of product
    // p - product name as string
    // returns current value in the producer
    pub fn check_hold(&self,p: &str) -> u64 {
        // if found return value
        // else return 0
        if self.holds.contains_key(&p) {
            *self.holds.get(&p).unwrap()
        } else {
            0
        }
    }
    // Trader sends materials to producer
    // p - product string
    // am - amount
    pub fn ship_materials(&mut self,p: &'a str,am: u64) {
        if self.holds.contains_key(p) {
            *self.holds.get_mut(p).unwrap() += am;
        } else {
            self.holds.insert(p,am);
        }
    }
    // Checks needs against current holdings and returns a bool
    // p - product key
    // am - amount
    pub fn daily_check(&self) -> bool {
        for (p,am) in self.needs.iter() {
            if self.check_hold(p) <= *am  {
                return false
            }
        }
        true
    }
    // Change owner id
    // no - new owner id
    pub fn change_owner(&mut self, no: usize) {
        self.ownerid = no;
    }
    pub fn get_ptype(self) -> u8 {
        self.ptype
    }
    pub fn get_pid(self) -> usize {
        self.pid
    }

    pub fn get_ownerid(self) -> usize {
        self.ownerid
    }
    pub fn get_cost(self) -> i64 {
        self.cost
    }

}
// Producer error log function
// Log file found in .\\logs\\producer.log
// Input
//      e - string with the message to 

fn log_error(e: String) {
    // Opens log file to append
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(".\\logs\\producer.log")
    .unwrap();

    writeln!(file, "{}",e).expect("Unable to write to Producer Log");
}

#[cfg(test)]
mod tests {
    #[warn(unused_imports)]
    use super::*;

    #[test]
    fn test_has_pid() {
        let p = Producer::new(1,0,1000);
        assert_eq!(1,p.get_pid());
    }

    #[test]
    fn test_materials_recieved_not_already_found() {
        let mut p = Producer::new(1,0,1000);
        p.ship_materials("Iron",300);
        assert_eq!(300,p.check_hold("Iron"));
    }
    #[test]
    fn test_materials_recieved_already_found() {
        let mut p = Producer::new(1,0,1000);
        p.ship_materials("Iron",300);
        p.ship_materials("Iron",300);
        assert_eq!(600,p.check_hold("Iron"));
    }
    #[test]
    fn test_daily_check_pass() {
        let mut p = Producer::new(1,0,1000);
        p.ship_materials("Iron",300);
        p.ship_materials("Coal",300);
        assert_eq!(true,p.daily_check());
    }
    #[test]
    fn test_daily_check_fail() {
        let p = Producer::new(1,0,1000);
        assert_eq!(false,p.daily_check());
    }
    #[test]
    fn test_change_owner() {
        let mut p = Producer::new(1,0,1000);
        p.change_owner(14);
        assert_eq!(14,p.get_ownerid());
    }
}