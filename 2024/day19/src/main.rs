use std::{
    collections::{HashMap, HashSet},
    fs, println,
};

fn dfs<'a>(
    i: usize,
    towels: &HashSet<&str>,
    outcome: &'a str,
    cache: &mut HashMap<(usize, &'a str), i64>,
) -> i64 {
    if i == outcome.len() {
        return 1;
    }

    if let Some(&count) = cache.get(&(i, outcome)) {
        return count;
    }

    let mut count = 0;
    for &towel in towels {
        if outcome[i..].starts_with(towel) {
            count += dfs(i + towel.len(), towels, outcome, cache);
        }
    }

    // Cache a failure only after every matching towel has been tried.
    cache.insert((i, outcome), count);
    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");
    let mut towels = HashSet::new();
    let input_towels: Vec<&str> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(str::trim)
        .collect();

    for towel in input_towels {
        towels.insert(towel);
    }

    let mut towel_combination = HashSet::new();
    for towel in input.lines().skip(2) {
        towel_combination.insert(towel);
    }

    let mut res = 0;
    let mut cache = HashMap::new();

    for towel in towel_combination {
        res += dfs(0, &towels, towel, &mut cache)
    }

    println!("There is {} combination of towels!", res);
}
