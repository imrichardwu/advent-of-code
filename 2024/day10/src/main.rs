use std::{collections::HashSet, fs};

// fn backtrack(
//     matrix: &Vec<Vec<i32>>,
//     row: i32,
//     col: i32,
//     prev: i32,
//     visited: &mut HashSet<(i32, i32)>,
// ) -> i32 {
//     if row < 0
//         || row >= matrix.len() as i32
//         || col < 0
//         || col >= matrix[0].len() as i32
//         || visited.contains(&(row, col))
//         || matrix[row as usize][col as usize] != prev + 1
//     {
//         return 0;
//     }

//     visited.insert((row, col));

//     let curr = matrix[row as usize][col as usize];

//     if curr == 9 {
//         return 1;
//     }

//     let directions = [[1, 0], [0, 1], [-1, 0], [0, -1]];
//     let mut res = 0;

//     for [dr, dc] in directions {
//         let nr = row + dr;
//         let nc = col + dc;

//         res += backtrack(matrix, nr, nc, curr, visited);
//     }

//     res
// }

fn backtrack(matrix: &Vec<Vec<i32>>, row: i32, col: i32, prev: i32) -> i32 {
    if row < 0
        || row >= matrix.len() as i32
        || col < 0
        || col >= matrix[0].len() as i32
        || matrix[row as usize][col as usize] != prev + 1
    {
        return 0;
    }

    let curr = matrix[row as usize][col as usize];

    if curr == 9 {
        return 1;
    }

    let directions = [[1, 0], [0, 1], [-1, 0], [0, -1]];
    let mut res = 0;

    for [dr, dc] in directions {
        let nr = row + dr;
        let nc = col + dc;

        res += backtrack(matrix, nr, nc, curr);
    }

    res
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let matrix: Vec<Vec<i32>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut res = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 0 {
                res += backtrack(&matrix, row as i32, col as i32, -1);
            }
        }
    }

    println!("Sum of all trailheads: {}", res);
}
