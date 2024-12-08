use grid::Grid;
use std::collections::HashSet;

use crate::random_utils::{bytes_grid, Direction, Pos, PosGet};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn unique_guard_positions_count(input: &str) -> u64 {
    // Count unique guard positions
    guard_lab_pos_dirs(input, |count, _, _, _| *count += 1) + 1
}

pub fn possible_obstruction_loops_count(input: &str) -> u64 {
    // Count possible obstruction loops
    guard_lab_pos_dirs(input, |count, lab_map, position, direction| {
        // Insert time travel obstruction
        let (mut position, mut direction) = (*position, *direction);
        let obstruction_position = position;

        *lab_map
            .pos_get_mut(obstruction_position)
            .expect("Expected obstruction position") = b'#';

        position.move_dir_mut(direction.opposite());
        let mut visited_obstacles = HashSet::new();

        loop {
            // Return if loop found
            if !visited_obstacles.insert((position, direction)) {
                *lab_map
                    .pos_get_mut(obstruction_position)
                    .expect("Expected obstruction position") = b'.';
                *count += 1;
                return;
            }

            // Follow path
            loop {
                match lab_map.pos_get(position.move_dir(direction)) {
                    Some(&b'#') => break,
                    Some(_) => position.move_dir_mut(direction),
                    None => {
                        *lab_map
                            .pos_get_mut(obstruction_position)
                            .expect("Expected obstruction position") = b'.';
                        return;
                    }
                }
            }

            direction.rotate_cw_mut();
        }
    })
}

// ------------------------------------------------------------------------------------------------
// Functions

fn guard_lab_pos_dirs<Count>(input: &str, counting_function: Count) -> u64
where
    Count: Fn(&mut u64, &mut Grid<u8>, &mut Pos, &mut Direction),
{
    let mut lab_map = bytes_grid(input);

    // Find guard starting position
    let mut position = lab_map
        .indexed_iter()
        .find(|(_, &c)| c == b'^')
        .expect("Expected guard starting position")
        .0
        .into();

    let mut direction = Direction::N;
    let mut count = 0;

    // Follow path and store unique (position, direction) first unique tuple
    while lab_map.pos_get(position).is_some() {
        if lab_map.pos_get(position.move_dir(direction)) == Some(&b'#') {
            direction.rotate_cw_mut();
        }

        position.move_dir_mut(direction);

        if lab_map.pos_get(position) == Some(&b'.') {
            counting_function(&mut count, &mut lab_map, &mut position, &mut direction);

            *lab_map
                .pos_get_mut(position)
                .expect("Expected position within grid") = b'X';
        }
    }

    count
}
