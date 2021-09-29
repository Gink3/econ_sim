#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::fs;

use serde::{Serialize, Deserialize};
use serde_json::Result;
mod sim;
use crate::sim::Sim;


fn main() {
    let mut s = Sim::new();
    s.init(50);
    s.run(10000, 90);
    // println!("{:?}",s);
}
