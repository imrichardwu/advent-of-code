use std::collections::HashMap;
use std::fs;

fn blink(stone: u64, blinks: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone, blinks)) {
        return result;
    }

    let result = if stone == 0 {
        blink(1, blinks - 1, cache)
    } else {
        let s = stone.to_string();

        if s.len() % 2 == 0 {
            let mid = s.len() / 2;

            let left: u64 = s[..mid].parse().unwrap();
            let right: u64 = s[mid..].parse().unwrap();

            blink(left, blinks - 1, cache) + blink(right, blinks - 1, cache)
        } else {
            blink(stone * 2024, blinks - 1, cache)
        }
    };

    cache.insert((stone, blinks), result);

    result
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");

    let stones: Vec<u64> = data
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();

    let total: u64 = stones
        .iter()
        .map(|&stone| blink(stone, 75, &mut cache))
        .sum();

    println!("{}", total);
}
