use std::{collections::HashSet, fs};

fn push_boxes(grid: &mut [Vec<u8>], target: (isize, isize), direction: (isize, isize)) -> bool {
    let (dx, dy) = direction;
    let mut pending = vec![target];
    let mut boxes = HashSet::new();

    // Find every box that must move, checking both destination cells.
    // Do not change the map until the entire push is known to be possible.
    while let Some((x, y)) = pending.pop() {
        let left_x = match grid[y as usize][x as usize] {
            b'#' => return false,
            b'.' => continue,
            b'[' => x,
            b']' => x - 1,
            _ => panic!("invalid map tile"),
        };
        if boxes.insert((left_x, y)) {
            pending.push((left_x + dx, y + dy));
            pending.push((left_x + 1 + dx, y + dy));
        }
    }

    // Clear all old positions first so overlapping moves cannot erase boxes.
    for &(x, y) in &boxes {
        grid[y as usize][x as usize] = b'.';
        grid[y as usize][(x + 1) as usize] = b'.';
    }
    for &(x, y) in &boxes {
        grid[(y + dy) as usize][(x + dx) as usize] = b'[';
        grid[(y + dy) as usize][(x + dx + 1) as usize] = b']';
    }
    true
}

fn solve(data: &str) -> usize {
    let lines = data.lines().map(str::trim).filter(|line| !line.is_empty());
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut moves = String::new();

    for line in lines {
        if line.starts_with('#') {
            let row = line
                .bytes()
                .flat_map(|tile| match tile {
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    b'.' => [b'.', b'.'],
                    b'@' => [b'@', b'.'],
                    _ => panic!("invalid map tile"),
                })
                .collect();
            grid.push(row);
        } else {
            moves.push_str(line);
        }
    }

    let mut robot = None;
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if *tile == b'@' {
                robot = Some((x as isize, y as isize));
                *tile = b'.';
            }
        }
    }
    let (mut x, mut y) = robot.expect("robot missing from map");

    for movement in moves.bytes() {
        let (dx, dy) = match movement {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            _ => panic!("invalid movement"),
        };
        let (next_x, next_y) = (x + dx, y + dy);
        if push_boxes(&mut grid, (next_x, next_y), (dx, dy)) {
            (x, y) = (next_x, next_y);
        }
    }

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| **tile == b'[')
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("failed to read input.txt");
    println!("{}", solve(&data));
}
