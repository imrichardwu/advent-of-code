use std::{
    collections::{BinaryHeap, HashSet},
    fs, println, vec,
};

fn shortest_path(board: &[Vec<char>]) -> Option<i32> {
    if board[0][0] == '#' || board[board.len() - 1][board[0].len() - 1] == '#' {
        return None;
    }

    let mut min_heap = BinaryHeap::new();
    min_heap.push((0_i32, 0_usize, 0_usize));

    let mut res = None;
    let mut visit = HashSet::new();
    visit.insert((0, 0));

    while let Some((steps, row, col)) = min_heap.pop() {
        if row == board.len() - 1 && col == board[0].len() - 1 {
            res = Some(-steps);
            break;
        }

        for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr < 0 || nc < 0 || nr >= board.len() as i32 || nc >= board[0].len() as i32 {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            if board[nr][nc] != '#' && !visit.contains(&(nr, nc)) {
                visit.insert((nr, nc));
                min_heap.push((steps - 1, nr, nc));
            }
        }
    }

    res
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");
    let mut board: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    for line in input.lines() {
        let (x, y) = line
            .trim()
            .split_once(',')
            .expect("expected (x,y) coordinates");
        let x: usize = x.parse().expect("invalid x");
        let y: usize = y.parse().expect("invalid y");

        board[x][y] = '#';

        if shortest_path(&board).is_none() {
            println!("{},{}", x, y);
            break;
        }
    }
}
