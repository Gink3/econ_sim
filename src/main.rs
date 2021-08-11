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
    // Set config file
    let mut s: Sim = Sim::new("./sim_config/sim.cfg".to_string());
    // write simulation data before initalization
    let mut data = serde_json::to_string_pretty(&s).unwrap();
    fs::write("./sim_out/s_debug_pre.dat", data).expect("Unable to write file");
    // Initalize simulation and write inital state to output file
    s.init();
    data = serde_json::to_string_pretty(&s).unwrap();
    fs::write("./sim_out/s_debug0.dat", data).expect("Unable to write file");
    // Run simulation and write output to file
    s.run(5);
    data = serde_json::to_string_pretty(&s).unwrap();
    fs::write("./sim_out/s_debug5.dat", data).expect("Unable to write file");

    s.print_market();

}
