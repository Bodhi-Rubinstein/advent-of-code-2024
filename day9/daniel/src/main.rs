use std::{cmp::min, env, fs::read_to_string};

#[derive(Debug)]
struct Block {
    size: i32,
    is_empty: bool,
    id: Option<i32>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();
    let filesystem: Vec<i32> = file.trim().chars().map(|x| (x as i32) - 48).collect();

    // PART ONE
    // o(n)!!!
    let mut position = 0;
    let mut right_block_ind = filesystem.len() - 1;
    let mut checksum: i128 = 0;
    let mut right_block_offset = 0;
    let mut left_block_offset;
    for (block_ind, block) in filesystem.iter().enumerate() {
        left_block_offset = 0;
        if block_ind >= right_block_ind {
            break;
        }
        let is_empty = block_ind % 2 == 1;

        if !is_empty {
            let file_id: i32 = block_ind as i32 / 2;
            for i in 0..*block {
                checksum += (file_id * (position + i)) as i128;
            }
            position += block;
            continue;
        }

        // yay
        loop {
            let right_empty = right_block_ind % 2 == 1;
            if right_empty {
                right_block_ind -= 1;
                if block_ind >= right_block_ind {
                    break;
                }
            }

            let initial_rbo = right_block_offset as i32;
            for i in 0..min(
                filesystem[right_block_ind] - right_block_offset,
                *block - left_block_offset,
            ) {
                let file_id: i32 = right_block_ind as i32 / 2;
                checksum += (file_id * (position + i)) as i128;
                // at end
                left_block_offset += 1;
                right_block_offset += 1;
            }

            position += right_block_offset as i32 - initial_rbo;

            if right_block_offset == filesystem[right_block_ind] {
                right_block_ind -= 1;
                right_block_offset = 0;
            } else {
                break;
            }
        }
    }

    // handle remaining right block offset
    if right_block_offset > 0 {
        for i in 0..(filesystem[right_block_ind] - right_block_offset) {
            let file_id = right_block_ind as i32 / 2;

            checksum += (file_id * (position + i)) as i128;
        }
    }

    println!("{}", checksum);

    // PART TWO
    // very much not o(n) lol

    let mut disk: Vec<Block> = Vec::new();

    // let mut expanded_disk: String = String::new();
    for (block_ind, block) in filesystem.iter().enumerate() {
        if *block == 0 {
            continue;
        }

        disk.push(Block {
            size: *block,
            is_empty: block_ind % 2 == 1,
            id: if block_ind % 2 == 0 {
                Some(block_ind as i32 / 2)
            } else {
                None
            },
        });
    }

    // loop through blocks backwards
    let mut block_ind: i32 = disk.len() as i32;
    loop {
        block_ind -= 1;
        if block_ind < 0 {
            break;
        }
        if disk[block_ind as usize].is_empty {
            continue;
        }

        // try to find a place for the last block forwards
        let mut candidate_block_ind = 0;
        loop {
            candidate_block_ind += 1;
            if candidate_block_ind >= block_ind {
                break;
            }

            // spot must be empty and big enough
            if !disk[candidate_block_ind as usize].is_empty
                || disk[candidate_block_ind as usize].size < disk[block_ind as usize].size
            {
                continue;
            }

            // move in the block
            // equal size case
            if disk[candidate_block_ind as usize].size == disk[block_ind as usize].size {
                disk[candidate_block_ind as usize].is_empty = false;
                disk[candidate_block_ind as usize].id = disk[block_ind as usize].id;

                disk[block_ind as usize].is_empty = true;
                disk[block_ind as usize].id = None;
            // more complicated less than case
            } else {
                // insert extra block
                let extra_size =
                    disk[candidate_block_ind as usize].size - disk[block_ind as usize].size;
                disk[candidate_block_ind as usize].is_empty = false;
                disk[candidate_block_ind as usize].id = disk[block_ind as usize].id;
                disk[candidate_block_ind as usize].size = disk[block_ind as usize].size;

                disk[block_ind as usize].is_empty = true;
                disk[block_ind as usize].id = None;

                disk.insert(
                    (candidate_block_ind + 1) as usize,
                    Block {
                        size: extra_size,
                        is_empty: true,
                        id: None,
                    },
                );

                block_ind += 1;
            }

            break;
        }
    }

    let mut position = 0;
    checksum = 0;
    for block in &disk {
        for _ in 0..block.size {
            if !block.is_empty {
                checksum += (block.id.unwrap() * position) as i128;
            }
            position += 1;
        }
    }

    println!("{}", checksum);
}
