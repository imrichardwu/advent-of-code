use std::fs;
mod calibration;

use crate::calibration::calculate;

fn main() {
    let data = fs::read_to_string("input.txt").expect("failed to read input.txt");
    let graph: Vec<(i64, Vec<i64>)> = data.lines().map(|line| {
        let (key, values) = line.split_once(":").expect("Error splitting the data");

        let key = key.parse::<i64>().expect("Unexpected key parsing error");
        let values = values
            .split_whitespace()
            .map(|value| value.parse::<i64>().expect("invalid value"))
            .collect();

        (key, values)
    }).collect();
    

    let res = calculate(graph);
    println!("Result is {}", res);
}
