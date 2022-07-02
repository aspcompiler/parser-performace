use std::time::Instant;

use serde_json::Value;

fn main() {
    let start = Instant::now();
    let input_path = std::env::args()
        .nth(1)
        .expect("Please enter an input Json file path");
    let data = std::fs::read_to_string(input_path).expect("Could not read input file");

    let json: Value = serde_json::from_str(&data).unwrap();
    assert!(matches!(json, Value::Array(_)));
    let array = if let Value::Array(array) = json { array } else { todo!() };
    assert!(array.len() == 5000);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
