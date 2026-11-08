use std::{collections::HashSet, fs, println};

fn dfs(
    row: i32,
    col: i32,
    grid: &Vec<Vec<char>>,
    visit: &mut HashSet<(i32, i32)>,
    current: char,
) -> (i32, i32) {
    if row < 0
        || row >= grid.len() as i32
        || col < 0
        || col >= grid[0].len() as i32
        || grid[row as usize][col as usize] != current
    {
        return (0, 0);
    }

    if visit.contains(&(row, col)) {
        return (0, 0);
    }

    visit.insert((row, col));

    let mut area = 1; // Count this plot.
    let same = |r: i32, c: i32| -> bool {
        r >= 0
            && r < grid.len() as i32
            && c >= 0
            && c < grid[0].len() as i32
            && grid[r as usize][c as usize] == current
    };

    // A region has one side per corner, including corners around holes.
    let mut sides = 0;
    for (dr, dc) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let vertical = same(row + dr, col);
        let horizontal = same(row, col + dc);
        let diagonal = same(row + dr, col + dc);

        let outer_corner = !vertical && !horizontal;
        let inner_corner = vertical && horizontal && !diagonal;
        if outer_corner || inner_corner {
            sides += 1;
        }
    }

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for (dr, dc) in directions {
        let (neighbor_area, neighbor_sides) = dfs(row + dr, col + dc, grid, visit, current);

        area += neighbor_area;
        sides += neighbor_sides;
    }

    (area, sides)
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let mut result = 0;
    let mut visit: HashSet<(i32, i32)> = HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let (area, sides) = dfs(row as i32, col as i32, &grid, &mut visit, grid[row][col]);

            result += area * sides;
        }
    }

    println!("{}", result);
}
