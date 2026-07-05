use std::{fs};
use std::collections::{HashMap, HashSet};

fn is_valid(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let in_update: HashSet<u32> = update.iter().copied().collect();
    let mut visited: HashSet<u32> = HashSet::new();
    

    for &page in update {
        for &(x, y) in rules {
            if y == page && in_update.contains(&x) && !visited.contains(&x) {
                return false;
            }
        }
        visited.insert(page);
    }

    true
}

fn fix_order(update: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut pages = update.to_vec();

    loop {
        let mut fixed = false;

        for &(x, y) in rules {
            let Some(xi) = pages.iter().position(|&p| p == x) else { continue };
            let Some(yi) = pages.iter().position(|&p| p == y) else { continue };

            if xi > yi {
                // x should be before y, but isn't — move x to just before y
                let x_page = pages.remove(xi);
                pages.insert(yi, x_page);
                fixed = true;
                break;
            }
        }

        if !fixed {
            break;
        }
    }

    pages
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");
    let (rules_text, updates_text) = content
    .split_once("\n\n")
    .expect("input should have rules, blank line, then updates");

    let rules: Vec<(u32, u32)> = rules_text.lines().map(|line| {
        let (left, right) = line.split_once("|").unwrap();
        (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
    }).collect();

    let updates: Vec<Vec<u32>> = updates_text.lines().map(|line| {
        line.split(',').map(|s| s.parse().unwrap()).collect()
    }).collect();

    let mut graph = HashMap::new();
    for (src, dst) in &rules {
        graph.entry(src).or_insert(vec![]).push(dst);
    }

    let mut res = 0;
    for update in updates {
        if !is_valid(&update, &rules) {
            let fixed = fix_order(&update, &rules);
            res += fixed[fixed.len() / 2];
        }
    }
    println!("{}", res);

}
