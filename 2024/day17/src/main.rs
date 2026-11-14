use std::{fs, println};

fn combo(operand: i64, a: i64, b: i64, c: i64) -> i64 {
    if (0..=3).contains(&operand) {
        return operand;
    }

    if operand == 4 {
        return a;
    }

    if operand == 5 {
        return b;
    }

    if operand == 6 {
        return c;
    }

    panic!("Invalid combo operand");
}

fn calc(mut a: i64, mut b: i64, mut c: i64, program: &[i64]) -> Vec<i64> {
    let mut ip: usize = 0;
    let mut output = Vec::new();

    while ip + 1 < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];

        match opcode {
            0 => {
                // adv: A = A / 2^combo
                let power = combo(operand, a, b, c);
                a = if power >= 63 { 0 } else { a / (1_i64 << power) };
            }

            1 => {
                // bxl: B = B XOR literal operand
                b ^= operand;
            }

            2 => {
                // bst: B = combo % 8
                b = combo(operand, a, b, c) % 8;
            }

            3 => {
                // jnz: Jump if A != 0
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }

            4 => {
                // bxc: B = B XOR C
                b ^= c;
            }

            5 => {
                // out: Output combo % 8
                let value = combo(operand, a, b, c) % 8;
                output.push(value);
            }

            6 => {
                // bdv: B = A / 2^combo
                let power = combo(operand, a, b, c);
                b = if power >= 63 { 0 } else { a / (1_i64 << power) };
            }

            7 => {
                // cdv: C = A / 2^combo
                let power = combo(operand, a, b, c);
                c = if power >= 63 { 0 } else { a / (1_i64 << power) };
            }

            _ => panic!("Invalid opcode"),
        }

        ip += 2;
    }

    output
}

fn find_initial_a(b: i64, c: i64, program: &[i64]) -> Option<i64> {
    let mut candidates = vec![0_i64];
    for start in (0..program.len()).rev() {
        let mut next = Vec::new();
        for prefix in candidates {
            for digit in 0..8 {
                if let Some(a) = prefix.checked_mul(8).and_then(|v| v.checked_add(digit)) {
                    if calc(a, b, c, program) == program[start..] {
                        next.push(a);
                    }
                }
            }
        }
        candidates = next;
    }
    candidates.into_iter().filter(|&a| a > 0).min()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read file");

    let a = data
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let b = data
        .lines()
        .nth(1)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let c = data
        .lines()
        .nth(2)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let program: Vec<i64> = data
        .lines()
        .nth(4)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let output = calc(a, b, c, &program);
    println!(
        "Part 1: {}",
        output
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
    let initial_a = find_initial_a(b, c, &program).expect("No solution found");
    println!("Part 2: {initial_a}");
}
