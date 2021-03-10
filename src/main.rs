#![allow(unused_imports)]
#![allow(dead_code)]

mod sim;
use crate::sim::Sim;

fn main() {
    let mut s: Sim = Sim::new();
    println!{"{:?}",s};
    s.init();
    println!("{:?}",s);
}
