use itertools::Itertools;
use pathfinding::directed::astar::astar;
use rustc_hash::FxHashSet;

use crate::random_utils::{parse_numbers, pos::Pos};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn minimum_steps_exit_kilobyte(input: &str) -> usize {
    // Simply find shortest path with first fallen KiB
    let corrupted_memory_bytes = corrupted_memory_bytes(input)
        .take(1024)
        .collect::<FxHashSet<_>>();

    memory_region_astar(&corrupted_memory_bytes).expect("Expected shortest path cost")
}

pub fn first_path_cutoff_byte(input: &str) -> String {
    let corrupted_memory_bytes = corrupted_memory_bytes(input).collect_vec();

    let (mut low, mut high) = (1024, corrupted_memory_bytes.len());
    let (mut middle, mut cut_off) = (0, false);

    // Binary search first fallen byte cutting off shortest path
    while (high - low) > 2 {
        middle = low + (high - low) / 2;

        if memory_region_astar(
            &corrupted_memory_bytes[..=middle]
                .iter()
                .copied()
                .collect::<FxHashSet<_>>(),
        )
        .is_some()
        {
            low = middle;
            cut_off = false;
        } else {
            high = middle;
            cut_off = true;
        }
    }

    (if cut_off {
        corrupted_memory_bytes[middle]
    } else {
        corrupted_memory_bytes[middle + 1]
    })
    .to_string()
}

// ------------------------------------------------------------------------------------------------
// Functions

fn memory_region_astar(corrupted_memory_bytes: &FxHashSet<Pos>) -> Option<usize> {
    let (start_position, end_position) = (Pos::new(0, 0), Pos::new(70, 70));

    // Find shortest path through memory region avoiding corrupted bytes
    astar(
        &start_position,
        |&position| {
            position
                .neighbors()
                .filter(|neighbor| {
                    !corrupted_memory_bytes.contains(neighbor)
                        && neighbor.in_bounds((start_position, end_position))
                })
                .map(|neighbor| (neighbor, 1))
        },
        |&position| position.manhattan_distance(end_position),
        |&position| position == end_position,
    )
    .map(|(_, cost)| cost)
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn corrupted_memory_bytes(input: &str) -> impl Iterator<Item = Pos> + '_ {
    input.lines().map(|line| {
        let coordinates: (isize, isize) = parse_numbers::<2, isize>(line).into();

        coordinates.into()
    })
}
