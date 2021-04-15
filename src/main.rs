#![allow(unused_imports)]
#![allow(dead_code)]

mod sim;
use crate::sim::Sim;

fn main() {
    let mut s: Sim = Sim::new(".\\sim_config\\sim.cfg".to_string());
    println!{"{:?}",s};
    s.init();
    println!("{:?}",s);
}
