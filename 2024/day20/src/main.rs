use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Clone, Copy)]
enum CheatState {
    Unused,
    Active(u8), // Moves remaining.
    Used,
}

fn backtrack(
    row: i32,
    col: i32,
    board: &Vec<Vec<char>>,
    state: CheatState,
    cache: &HashMap<(i32, i32), i32>,
    time: i32,
    visit: &mut HashSet<(i32, i32)>,
    best_time: &mut HashMap<(i32, i32), i32>,
) -> i32 {
    if row < 0 || row >= board.len() as i32 || col < 0 || col >= board[0].len() as i32 {
        return 0;
    }

    let is_wall = board[row as usize][col as usize] == '#';
    let mut count = 0;

    let next_state = match state {
        CheatState::Unused => {
            best_time.insert((row, col), time);
            CheatState::Active(19)
        }

        CheatState::Used => {
            if is_wall {
                return 0;
            }

            if let Some(&normal_time) = cache.get(&(row, col)) {
                if normal_time - time >= 100 && visit.insert((row, col)) {
                    return 1;
                }
            }

            return 0;
        }

        CheatState::Active(remaining) => {
            if let Some(&earliest) = best_time.get(&(row, col)) {
                if earliest <= time {
                    return 0;
                }
            }
            best_time.insert((row, col), time);
            count += backtrack(
                row,
                col,
                board,
                CheatState::Used,
                cache,
                time,
                visit,
                best_time,
            );

            if remaining == 0 {
                return count;
            }

            CheatState::Active(remaining - 1)
        }
    };

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for (dr, dc) in directions {
        count += backtrack(
            row + dr,
            col + dc,
            board,
            next_state,
            cache,
            time + 1,
            visit,
            best_time,
        );
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut r = None;
    let mut c = None;

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] == 'S' {
                r = Some(row);
                c = Some(col);
                break;
            }
        }

        if r.is_some() {
            break;
        }
    }

    let start = (r.unwrap() as i32, c.unwrap() as i32);
    let mut cache = HashMap::from([(start, 0)]);
    let mut queue = VecDeque::from([start]);

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((row, col)) = queue.pop_front() {
        let time = cache[&(row, col)];

        for (dr, dc) in directions {
            let nr = row + dr;
            let nc = col + dc;

            if nr < 0
                || nr >= board.len() as i32
                || nc < 0
                || nc >= board[0].len() as i32
                || board[nr as usize][nc as usize] == '#'
                || cache.contains_key(&(nr, nc))
            {
                continue;
            }

            cache.insert((nr, nc), time + 1);
            queue.push_back((nr, nc));
        }
    }

    let mut res = 0;

    for (&(row, col), &time) in &cache {
        let mut visit = HashSet::new();
        let mut best_time = HashMap::new();

        res += backtrack(
            row,
            col,
            &board,
            CheatState::Unused,
            &cache,
            time,
            &mut visit,
            &mut best_time,
        );
    }

    println!("Cheats saving at least 100 picoseconds: {res}");
}
