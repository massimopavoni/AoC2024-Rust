use std::collections::HashSet;

use grid::Grid;

use crate::random_utils::{bytes_grid, Direction, Pos, PosGet};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn unique_guard_positions_count(input: &str) -> u64 {
    let mut lab_map = bytes_grid(input);

    // Count unique guard positions
    guard_lab_pos_dirs(&mut lab_map).len() as u64 + 1
}

pub fn possible_obstruction_loops_count(input: &str) -> u64 {
    let mut lab_map = bytes_grid(input);

    // Count possible obstruction loops
    guard_lab_pos_dirs(&mut lab_map.clone())
        .into_iter()
        .filter(|&(mut position, mut direction)| {
            // Insert time travel obstruction
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
                    return true;
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
                            return false;
                        }
                    }
                }

                direction.rotate_cw_mut();
            }
        })
        .count() as u64
}

// ------------------------------------------------------------------------------------------------
// Functions

fn guard_lab_pos_dirs(lab_map: &mut Grid<u8>) -> Vec<(Pos, Direction)> {
    // Find guard starting position
    let mut position = lab_map
        .indexed_iter()
        .find(|(_, &c)| c == b'^')
        .expect("Expected guard starting position")
        .0
        .into();

    let mut direction = Direction::N;
    let mut unique_pos_dirs = Vec::new();

    // Follow path and store unique (position, direction) first unique tuple
    while lab_map.pos_get(position).is_some() {
        if lab_map.pos_get(position.move_dir(direction)) == Some(&b'#') {
            direction.rotate_cw_mut();
        }

        position.move_dir_mut(direction);

        if lab_map.pos_get(position) == Some(&b'.') {
            unique_pos_dirs.push((position, direction));
            *lab_map
                .pos_get_mut(position)
                .expect("Expected position within grid") = b'X';
        }
    }

    unique_pos_dirs
}
