use std::fs;

// fn compact_disk(disk: &mut [Option<u64>]) {
//     if disk.is_empty() {
//         return;
//     }

//     let mut left = 0;
//     let mut right = disk.len() - 1;

//     while left < right {
//         while left < right && disk[left].is_some() {
//             left += 1;
//         }

//         while left < right && disk[right].is_none() {
//             right -= 1;
//         }

//         if left < right {
//             disk.swap(left, right);
//             left += 1;
//             right -= 1;
//         }
//     }
// }

struct File {
    id: u64,
    start: u64,
    len: u64,
}

struct FreeSpace {
    start: u64,
    len: u64,
}


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let mut files = Vec::new();
    let mut free_spaces = Vec::new();
    let mut position = 0_u64;

    for (index, digit) in input.trim().chars().enumerate() {
        let len = digit.to_digit(10).expect("Input must contain digits") as u64;

        if index % 2 == 0 {
            files.push(File {
                id: (index / 2) as u64,
                start: position,
                len,
            })
        } else {
            free_spaces.push(FreeSpace {
                start: position,
                len,
            });
        }

        position += len;
    }

    for file in files.iter_mut().rev() {
        if file.len == 0 {
            continue;
        }

        if let Some(space) = free_spaces
            .iter_mut()
            .find(|space| space.start < file.start && space.len >= file.len)
        {
            file.start = space.start;
            space.start += file.len;
            space.len -= file.len;
        }
    }

    let checksum: u64 = files
        .iter()
        .map(|file| {
            file.id
                * (file.len * file.start
                    + file.len * file.len.saturating_sub(1) / 2)
        })
        .sum();

    println!("{checksum}");

   
}