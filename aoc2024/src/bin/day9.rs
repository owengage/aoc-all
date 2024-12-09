use aoc::{fetch_input, text};

fn main() {
    let input = text(fetch_input(2024, 9));
    let disk = parse_disk(&input);

    let part1 = part1(&disk);
    dbg!(part1);

    let part2 = part2(&disk);
    dbg!(part2);
}

fn part1(disk: &[i32]) -> usize {
    let mut disk = disk.to_vec();
    // keep a cursor from the left and right, move things on the right to the
    // left empty blocks.
    let mut next_empty = 0;
    let mut last_occupied = disk.len();

    loop {
        next_empty = find_next_empty(&disk, next_empty);
        last_occupied = find_last_occupied(&disk, last_occupied);
        if next_empty > last_occupied {
            break;
        };

        disk[next_empty] = disk[last_occupied];
        disk[last_occupied] = -1;
    }

    checksum(&disk)
}

fn part2(disk: &[i32]) -> usize {
    let mut disk = disk.to_vec();
    let max_file_id = find_max_id(&disk);

    // Supposed to try and move each file once, so lets do exactly that.
    for id in (0..=max_file_id).rev() {
        let (file_start, file_len) = find_file_rev(&disk, id);
        let gap = find_next_empty_gap(&disk, file_len, file_start);

        if let Some(gap) = gap {
            move_file(&mut disk, file_start, file_len, gap);
        }
    }

    checksum(&disk)
}

fn move_file(disk: &mut [i32], file_start: usize, file_len: usize, gap: usize) {
    let rng = file_start..file_start + file_len;
    assert!(!rng.contains(&gap)); // shouldn't overlap. not really a sufficient check.

    let mut id = None;
    for (i, src) in rng.enumerate() {
        match id {
            // Check the file blocks are all the same.
            Some(id) => assert!(id == disk[src]),
            None => id = Some(disk[src]),
        };

        disk[gap + i] = disk[src];
        disk[src] = -1; // delete old.
    }
}

fn find_next_empty_gap(disk: &[i32], gap_needed: usize, before: usize) -> Option<usize> {
    let mut current = 0;

    loop {
        // get to the next gap.
        while disk[current] != -1 {
            current += 1;
            if current >= before {
                return None;
            }
        }

        // Get the size of this gap.
        let mut current_gap = 0;
        while disk[current] == -1 {
            current += 1;
            current_gap += 1;
            if current_gap >= gap_needed {
                // immediately return if this gap is big enough.
                return Some(current - current_gap);
            }
            if current >= before {
                return None;
            }
        }
    }
}

// (start, length)
fn find_file_rev(disk: &[i32], file_id: i32) -> (usize, usize) {
    let mut cursor = disk.len() - 1;

    // Go backwards until we find the file id.
    while disk[cursor] != file_id {
        cursor -= 1;
    }
    let end = cursor + 1;

    // Find the start.
    while disk[cursor] == file_id {
        if cursor == 0 {
            return (0, end);
        }
        cursor -= 1;
    }
    (cursor + 1, end - cursor - 1)
}

fn find_max_id(disk: &[i32]) -> i32 {
    for block in disk.iter().rev() {
        if *block != -1 {
            return *block;
        }
    }
    0
}

fn checksum(disk: &[i32]) -> usize {
    let mut total = 0;
    for (i, id) in disk.iter().enumerate() {
        if *id != -1 {
            total += i * (*id as usize);
        }
    }

    total
}

fn find_last_occupied(disk: &[i32], mut last_occupied: usize) -> usize {
    last_occupied -= 1;
    loop {
        if let Some(id) = disk.get(last_occupied) {
            if *id != -1 {
                return last_occupied;
            } else {
                last_occupied -= 1;
            }
        } else {
            return 0;
        }
    }
}

fn find_next_empty(disk: &[i32], mut next_empty: usize) -> usize {
    next_empty += 1;
    loop {
        if let Some(id) = disk.get(next_empty) {
            if *id == -1 {
                return next_empty;
            } else {
                next_empty += 1;
            }
        } else {
            return disk.len();
        }
    }
}

// Format is that each element is a block of the disk. -1 represents an empty block.
fn parse_disk(input: &str) -> Vec<i32> {
    let mut blocks = vec![];

    for (i, ch) in input.trim().bytes().enumerate() {
        let ch = (ch as char).to_digit(10).unwrap();
        let value = if i % 2 == 0 { i as i32 / 2 } else { -1 };
        blocks.resize(blocks.len() + ch as usize, value);
    }

    blocks
}
