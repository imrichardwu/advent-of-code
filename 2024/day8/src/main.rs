use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn solution(matrix: &Vec<String>) -> i32 {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (r, row) in matrix.iter().enumerate() {
        for (c, cell) in row.chars().enumerate() {
            if cell != '.' {
                antennas.entry(cell).or_default().push((r as i32, c as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                let dr = r2 - r1;
                let dc = c2 - c1;

                // Normalize the direction.
                let divisor = gcd(dr, dc);
                let step_r = dr / divisor;
                let step_c = dc / divisor;

                // Walk backwards from the first antenna.
                let mut r = r1;
                let mut c = c1;

                while (0..rows).contains(&r) && (0..cols).contains(&c) {
                    antinodes.insert((r, c));

                    r -= step_r;
                    c -= step_c;
                }

                // Walk forwards from the first antenna.
                let mut r = r1 + step_r;
                let mut c = c1 + step_c;

                while (0..rows).contains(&r) && (0..cols).contains(&c) {
                    antinodes.insert((r, c));

                    r += step_r;
                    c += step_c;
                }
            }
        }
    }

    antinodes.len() as i32
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");
    let matrix: Vec<String> = data.lines().map(str::to_owned).collect();

    println!("{}", solution(&matrix));
}
