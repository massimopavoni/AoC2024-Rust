use grid::Grid;
use rustc_hash::FxHashSet;

use crate::random_utils::{
    FxHashWithCapacity, bytes_grid,
    pos::{Dir, GridPosGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn unique_guard_positions_count(input: &str) -> u64 {
    // Count unique guard positions
    guard_lab_pos_dirs(
        input,
        |lab_map, position| {
            // Setup only finds initial guard position
            *position = lab_map
                .indexed_iter()
                .find(|&(_, &c)| c == b'^')
                .expect("Expected guard position")
                .0
                .into();

            None::<()>
        },
        |_, count, _, _| *count += 1,
    ) + 1
}

pub fn possible_obstruction_loops_count(input: &str) -> u64 {
    #[inline]
    fn binary_insert(vec: &mut Vec<usize>, item: usize) -> usize {
        match vec.binary_search(&item) {
            Ok(index) | Err(index) => {
                vec.insert(index, item);
                index
            }
        }
    }

    // Count possible obstruction loops
    guard_lab_pos_dirs(
        input,
        |lab_map, position| {
            // Setup stores obstacle positions for faster loop detection
            let (mut rows_obstacles, mut cols_obstacles) = (
                vec![Vec::with_capacity(lab_map.cols() / 4); lab_map.rows()],
                vec![Vec::with_capacity(lab_map.rows() / 4); lab_map.cols()],
            );

            for ((x, y), &c) in lab_map.indexed_iter() {
                match c {
                    b'#' => {
                        rows_obstacles[x].push(y);
                        cols_obstacles[y].push(x);
                    }
                    b'.' => {}
                    b'^' => *position = Pos::from((x, y)),
                    _ => unreachable!("Invalid lab tile"),
                }
            }

            (
                FxHashSet::with_capacity(rows_obstacles.len() * cols_obstacles.len() / 32),
                rows_obstacles,
                cols_obstacles,
            )
        },
        |(visited_obstacles, rows_obstacles, cols_obstacles), count, position, direction| {
            // Insert time travel obstruction
            let (mut position, mut direction) = (position, direction);
            let (obstruction_x, obstruction_y) = position.into();

            position.move_dir_mut(direction.opposite());

            let (mut x, mut y) = position.into();
            let obstruction_index = (
                binary_insert(&mut rows_obstacles[obstruction_x], obstruction_y),
                binary_insert(&mut cols_obstacles[obstruction_y], obstruction_x),
            );

            visited_obstacles.clear();

            loop {
                direction.rotate_cw_mut();

                // Return if loop found
                if !visited_obstacles.insert((x, y, direction)) {
                    rows_obstacles[obstruction_x].remove(obstruction_index.0);
                    cols_obstacles[obstruction_y].remove(obstruction_index.1);
                    *count += 1;
                    return;
                }

                // Follow path
                if let Some(Some(next_obstacle)) = match direction {
                    Dir::S => cols_obstacles
                        .get(y)
                        .map(|vec| vec.iter().find(|&&obstacle| obstacle > x)),
                    Dir::E => rows_obstacles
                        .get(x)
                        .map(|vec| vec.iter().find(|&&obstacle| obstacle > y)),
                    Dir::N => cols_obstacles
                        .get(y)
                        .map(|vec| vec.iter().rev().find(|&&obstacle| obstacle < x)),
                    Dir::W => rows_obstacles
                        .get(x)
                        .map(|vec| vec.iter().rev().find(|&&obstacle| obstacle < y)),
                } {
                    (x, y) = (
                        match direction {
                            Dir::S => next_obstacle - 1,
                            Dir::N => next_obstacle + 1,
                            Dir::E | Dir::W => x,
                        },
                        match direction {
                            Dir::S | Dir::N => y,
                            Dir::E => next_obstacle - 1,
                            Dir::W => next_obstacle + 1,
                        },
                    );
                } else {
                    rows_obstacles[obstruction_x].remove(obstruction_index.0);
                    cols_obstacles[obstruction_y].remove(obstruction_index.1);
                    return;
                }
            }
        },
    )
}

// ------------------------------------------------------------------------------------------------
// Functions

fn guard_lab_pos_dirs<Setup, S, Count>(
    input: &str,
    setup_function: Setup,
    counting_function: Count,
) -> u64
where
    Setup: Fn(&mut Grid<u8>, &mut Pos) -> S,
    Count: Fn(&mut S, &mut u64, Pos, Dir),
{
    // Parse lab map
    let mut lab_map = bytes_grid(input);
    let (mut position, mut direction) = (Pos::new(0, 0), Dir::N);
    let mut count = 0;

    // Setup
    let mut setup = setup_function(&mut lab_map, &mut position);

    // Follow path and store (position, direction) first unique tuple
    while lab_map.pos_get(position).is_some() {
        if lab_map.pos_get(position.move_dir(direction)) == Some(&b'#') {
            direction.rotate_cw_mut();
        }

        position.move_dir_mut(direction);

        if lab_map.pos_get(position) == Some(&b'.') {
            counting_function(&mut setup, &mut count, position, direction);

            *lab_map.pos_index_mut(position) = b'X';
        }
    }

    count
}
