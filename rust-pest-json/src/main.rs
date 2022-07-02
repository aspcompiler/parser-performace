extern crate pest;
#[macro_use]
extern crate pest_derive;

mod json;

use std::time::Instant;

use json::parse_json;

fn main() {
    let start = Instant::now();
    let input_path = std::env::args()
        .nth(1)
        .expect("Please enter an input Json file path");
    let data = std::fs::read_to_string(input_path).expect("Could not read input file");

    parse_json(&data).unwrap();

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
