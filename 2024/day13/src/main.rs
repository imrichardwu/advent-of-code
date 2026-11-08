use std::fs;

struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

fn coordinates(line: &str) -> (u64, u64) {
    let mut numbers = line
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());

    (numbers.next().unwrap(), numbers.next().unwrap())
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("failed to read input.txt");
    let lines: Vec<&str> = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert_eq!(lines.len() % 3, 0, "Expected three lines per machine");
    let list: Vec<Machine> = lines
        .chunks_exact(3)
        .map(|group| Machine {
            a: coordinates(group[0]),
            b: coordinates(group[1]),
            prize: coordinates(group[2]),
        })
        .collect();

    let mut total = 0i128;

    for machine in list {
        let (ax, ay) = (i128::from(machine.a.0), i128::from(machine.a.1));
        let (bx, by) = (i128::from(machine.b.0), i128::from(machine.b.1));
        let px = i128::from(machine.prize.0) + 10_000_000_000_000;
        let py = i128::from(machine.prize.1) + 10_000_000_000_000;

        // Solve ax * a + bx * b = px and ay * a + by * b = py.
        // Signed integers handle negative determinants without rounding errors.
        let determinant = ax * by - ay * bx;
        assert_ne!(
            determinant, 0,
            "Parallel button movements require a separate solver"
        );
        let a_numerator = px * by - py * bx;
        let b_numerator = ax * py - ay * px;

        // A fractional number of presses cannot win a prize.
        if a_numerator % determinant != 0 || b_numerator % determinant != 0 {
            continue;
        }

        let a = a_numerator / determinant;
        let b = b_numerator / determinant;
        if a >= 0 && b >= 0 {
            total += 3 * a + b;
        }
    }

    println!("{total}");
}
