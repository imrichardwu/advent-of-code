use std::fs;

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe_report(levels) {
        return true
    }

    for i in 0..levels.len() {
        let mut without: Vec<i32> = Vec::with_capacity(levels.len() - 1);
        for (j, &level) in levels.iter().enumerate() {
            if i != j {
                without.push(level);
            }
        }
        if is_safe_report(&without) {
            return true;
        }

    }   

    false
}

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true
    }

    let direction = (levels[1] - levels[0]).signum();
    if direction == 0 {
        return false;
    }

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 || diff.signum() != direction {
            return false;
        }
    }
    true
}

fn count_safe(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|report| is_safe_with_dampener(report)).count()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("failed to read input.txt file");
    let report: Vec<Vec<i32>>  = data.lines().filter(|line| !line.trim().is_empty()).map(|line| {
        line.split_whitespace()
            .map(|n| n.parse().expect("invalid number"))
            .collect()
    }).collect();

    let res = count_safe(&report);
    println!("Number of reports {}", res);
}
