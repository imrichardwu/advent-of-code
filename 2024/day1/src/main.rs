use std::{collections::HashMap, fs};

fn calc(distance: &Vec<[i32; 2]>, count: &HashMap<i32, i32>) -> i32 {
    let mut left: Vec<i32> = distance.iter().map(|pair| pair[0]).collect();
    let mut right: Vec<i32> = distance.iter().map(|pair| pair[1]).collect();
    let mut res = 0;

    left.sort();
    right.sort();

    for (l, _) in left.iter().zip(right.iter()) {
        let val = l * count.get(l).unwrap_or(&0);
        res += val;
    }

    res
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let input_data: Vec<[i32; 2]> = data.lines().filter(|line| !line.trim().is_empty()).map(|line| {
        let mut parts = line.split_whitespace();
        let left: i32 = parts.next().unwrap().parse().unwrap();
        let right: i32 = parts.next().unwrap().parse().unwrap();
        [left, right]
    }).collect();

    let mut count: HashMap<i32, i32> = HashMap::new();
    for data in input_data.iter() {
        *count.entry(data[1]).or_insert(0) += 1;
    }

    let total_distance = calc(&input_data, &count);
    println!("The total distance is {}", total_distance);
}
