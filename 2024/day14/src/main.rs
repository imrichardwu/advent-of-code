use std::{collections::HashSet, fs};

struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn coordinates(value: &str) -> (i64, i64) {
    let mut numbers = value
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap());

    (numbers.next().unwrap(), numbers.next().unwrap())
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("failed to read input.txt");
    let list: Vec<Robot> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            Robot {
                position: coordinates(parts.next().unwrap()),
                velocity: coordinates(parts.next().unwrap()),
            }
        })
        .collect();

    let width = 101;
    let height = 103;
    let mut most_neighbors = 0;
    let mut result = 0;

    assert!(!list.is_empty(), "input.txt is empty");

    // Every robot repeats its position after width * height seconds.
    for seconds in 0..width * height {
        let mut positions: HashSet<(i64, i64)> = HashSet::new();

        for robot in &list {
            let (x, y) = robot.position;
            let (vx, vy) = robot.velocity;
            let x = (x + vx * seconds).rem_euclid(width);
            let y = (y + vy * seconds).rem_euclid(height);

            positions.insert((x, y));
        }

        // The tree has many neighboring robots. Count each adjacent pair once.
        let mut neighbors = 0;
        for &(x, y) in &positions {
            for (dx, dy) in [(1, 0), (0, 1)] {
                if positions.contains(&(x + dx, y + dy)) {
                    neighbors += 1;
                }
            }
        }

        if neighbors > most_neighbors {
            most_neighbors = neighbors;
            result = seconds;
        }
    }

    println!("{result}");
}
