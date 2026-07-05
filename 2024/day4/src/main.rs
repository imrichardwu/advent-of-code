use std::fs;

const WORD: &str = "XMAS";

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1), (1, 0), (0, -1), (-1, 0),
    (1, 1), (1, -1), (-1, 1), (-1, -1),
];

fn matches_xmas(
    grid: &[Vec<char>],
    row: i32,
    col: i32,
    dr: i32,
    dc: i32,
    i: usize,
) -> bool {
    if i == WORD.len() {
        return true;
    }

    if row < 0
        || row >= grid.len() as i32
        || col < 0
        || col >= grid.len() as i32
        || grid[row as usize][col as usize] != WORD.chars().nth(i).unwrap()
    {
        return false;
    }

    matches_xmas(grid, row + dr, col + dc, dr, dc, i + 1)
}

fn diagonal_mas(grid: &[Vec<char>], r1: i32, c1: i32, r2: i32, c2: i32) -> bool {
    let chars: Vec<char> = [(r1, c1), (r2, c2)]
        .iter()
        .map(|&(r, c)| grid[r as usize][c as usize])
        .collect();
    chars == ['M', 'S'] || chars == ['S', 'M']
}

fn is_xmas(grid: &[Vec<char>], r: usize, c: usize) -> bool {
    if grid[r][c] != 'A' {
        return false;
    }

    let r = r as i32;
    let c = c as i32;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // need 1 cell of padding on all sides
    if r == 0 || c == 0 || r == rows - 1 || c == cols - 1 {
        return false;
    }

    let nw_se = diagonal_mas(grid, r - 1, c - 1, r + 1, c + 1);
    let ne_sw = diagonal_mas(grid, r - 1, c + 1, r + 1, c - 1);
    nw_se && ne_sw
}

fn main() {
    let grid: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("failed to read input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut res = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if is_xmas(&grid, row, col) {
                res += 1;
            }
        }
    }

    println!("Total xmas is {}", res);
}