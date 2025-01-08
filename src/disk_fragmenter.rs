use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn compact_disk_checksum(input: &str) -> usize {
    // Create vector of disk blocks as Options
    let disk = input
        .trim()
        .bytes()
        .enumerate()
        .filter(|&(_, c)| c != b'0')
        .flat_map(|(i, c)| vec![if i % 2 == 0 { Some(i / 2) } else { None }; (c - 48) as usize])
        .collect_vec();

    let (mut start, mut end) = (0, disk.len() - 1);
    let mut checksum = 0;

    // Use two pointers to swap blocks and calculate checksum in about one pass
    while start <= end {
        if let Some(block) = disk[start] {
            checksum += block * start;
        } else {
            while disk[end].is_none() {
                end -= 1;
            }

            if end < start {
                break;
            }

            checksum += disk[end].expect("Expected some number") * start;
            end -= 1;
        }

        start += 1;
    }

    checksum
}

pub fn whole_files_compact_disk_checksum(input: &str) -> usize {
    let mut free_space = vec![BinaryHeap::new(); 10];
    let mut block = 0;

    // Fill free space min-heaps while converting bytes to file block sizes
    let disk = input
        .trim()
        .bytes()
        .enumerate()
        .map(|(i, c)| {
            let size = (c - 48) as usize;

            if i % 2 == 1 && size > 0 {
                free_space[size].push(Reverse(block));
            }

            block += size;
            size
        })
        .collect_vec();

    let (parity, mut checksum) = (disk.len() % 2, 0);

    // Calculate checksum by moving blocks to leftmost free space while keeping min-heaps updated
    for (block_index, block_size) in disk.into_iter().enumerate().rev() {
        block -= block_size;

        // Skip empty blocks
        if block_index % 2 == parity {
            continue;
        }

        let (mut best_index, mut best_size) = (block, usize::MAX);

        // Find leftmost free space where block fits
        for (size, indices) in free_space.iter_mut().enumerate().skip(block_size) {
            if let Some(&Reverse(best)) = indices.peek() {
                if best < best_index {
                    best_index = best;
                    best_size = size;
                }
            }
        }

        // Remove all big free spaces with smallest index to the right of current
        if let Some(biggest) = free_space.last() {
            if let Some(&Reverse(first)) = biggest.peek() {
                if first > block {
                    free_space.pop();
                }
            } else {
                free_space.pop();
            }
        }

        // Update checksum with factorized formula
        checksum += block_index / 2 * (best_index * block_size + TRIANGLE[block_size]);

        // Update free spaces
        if best_size != usize::MAX {
            free_space[best_size].pop();

            let new_size = best_size - block_size;

            if new_size > 0 {
                free_space[new_size].push(Reverse(best_index + block_size));
            }
        }
    }

    checksum
}

// ------------------------------------------------------------------------------------------------
// Functions

// Offset precomputed triangle numbers for checksum calculation
const TRIANGLE: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];
