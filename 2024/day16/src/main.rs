use std::{
    collections::{HashMap, HashSet},
    fs, println,
};

type State = (i64, i64, char);

fn dfs(
    row: i64,
    col: i64,
    board: &Vec<Vec<char>>,
    visit: &mut HashMap<State, i64>,
    points: i64,
    current_directions: char,
) -> (i64, usize) {
    let directions = [
        (1, 0, 'S'),  // Down
        (0, 1, 'E'),  // Right
        (-1, 0, 'N'), // Up
        (0, -1, 'W'), // Left
    ];

    let mut res = i64::MAX;
    let mut parents: HashMap<State, Vec<State>> = HashMap::new();

    let mut stack = vec![(row, col, points, current_directions, None)];
    while let Some((row, col, points, current_directions, parent)) = stack.pop() {
        if points > res
            || row < 0
            || row >= board.len() as i64
            || col < 0
            || col >= board[row as usize].len() as i64
            || board[row as usize][col as usize] == '#'
        {
            continue;
        }

        let state = (row, col, current_directions);
        if let Some(&best) = visit.get(&state) {
            if best < points {
                continue;
            }
            if best == points {
                // Keep every equally good route into this state.
                if let Some(parent) = parent {
                    parents.entry(state).or_default().push(parent);
                }
                continue;
            }
        }
        visit.insert(state, points);
        // A cheaper route replaces the parents of the old, more expensive route.
        parents.insert(state, parent.into_iter().collect());

        if board[row as usize][col as usize] == 'E' {
            res = res.min(points);
            continue;
        }

        for (dr, dc, direction) in directions.into_iter().rev() {
            let cost = if current_directions == direction {
                1
            } else if matches!(
                (current_directions, direction),
                ('N', 'S') | ('S', 'N') | ('E', 'W') | ('W', 'E')
            ) {
                2001
            } else {
                1001
            };
            stack.push((row + dr, col + dc, points + cost, direction, Some(state)));
        }
    }

    let mut stack: Vec<State> = visit
        .iter()
        .filter_map(|(&state, &score)| {
            (score == res && board[state.0 as usize][state.1 as usize] == 'E').then_some(state)
        })
        .collect();
    let mut seen_states = HashSet::new();
    let mut tiles = HashSet::new();
    while let Some(state) = stack.pop() {
        if !seen_states.insert(state) {
            continue;
        }
        tiles.insert((state.0, state.1));
        if let Some(previous_states) = parents.get(&state) {
            stack.extend(previous_states.iter().copied());
        }
    }

    (res, tiles.len())
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("failed to read input.txt");
    let board: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let mut visit = HashMap::new();
    let mut start_row = None;
    let mut start_col = None;

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] == 'S' {
                start_row = Some(row as i64);
                start_col = Some(col as i64);
                break;
            }
        }
        if start_row.is_some() {
            break;
        }
    }

    let (res, tiles) = dfs(
        start_row.unwrap(),
        start_col.unwrap(),
        &board,
        &mut visit,
        0,
        'E',
    );

    if res == i64::MAX {
        println!("No path found");
    } else {
        println!("Total point {}", res);
        println!("Best-path tiles {}", tiles);
    }
}

#[cfg(test)]
mod tests;
