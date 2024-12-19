use grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;

use crate::random_utils::{
    bytes_grid,
    pos::{Direction, GridPosGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn final_thin_boxes_coordinates_sum(input: &str) -> usize {
    // Move thin boxes around
    final_boxes_coordinates_sum(
        input,
        false,
        b'O',
        |warehouse, position, next, direction| {
            let mut next_next = next.move_dir(direction);

            // Group boxes
            while warehouse.pos_get_expect(next_next) == &b'O' {
                next_next.move_dir_mut(direction);
            }

            // Move if possible
            match warehouse.pos_get_expect(next_next) {
                b'#' => {}
                b'.' => {
                    *warehouse.pos_get_mut_expect(next) = b'.';
                    *warehouse.pos_get_mut_expect(next_next) = b'O';
                    *position = next;
                }
                _ => unreachable!("Invalid warehouse tile"),
            }
        },
    )
}

pub fn final_wide_boxes_coordinates_sum(input: &str) -> usize {
    // Move wide boxes around
    final_boxes_coordinates_sum(input, true, b'[', |warehouse, position, next, direction| {
        match direction {
            Direction::S | Direction::N => {
                // Keep moving boxes HashSet Vec
                let mut box_positions = vec![HashSet::from([next])];

                let next_side = next.move_dir(if warehouse.pos_get_expect(next) == &b'[' {
                    Direction::E
                } else {
                    Direction::W
                });

                box_positions[0].insert(next_side);

                // Gather moving boxes
                loop {
                    let mut more_boxes = HashSet::new();

                    for &box_position in box_positions.last().expect("Expected warehouse tiles") {
                        let another_box_position = box_position.move_dir(direction);

                        match warehouse.pos_get_expect(another_box_position) {
                            b'#' => return,
                            b'.' => {}
                            b'[' => {
                                more_boxes.insert(another_box_position);
                                more_boxes.insert(another_box_position.move_dir(Direction::E));
                            }
                            b']' => {
                                more_boxes.insert(another_box_position);
                                more_boxes.insert(another_box_position.move_dir(Direction::W));
                            }
                            _ => unreachable!("Invalid warehouse tile"),
                        }
                    }

                    if more_boxes.is_empty() {
                        break;
                    }

                    box_positions.push(more_boxes);
                }

                // Move them
                for position in box_positions.into_iter().rev().flatten() {
                    *warehouse.pos_get_mut_expect(position.move_dir(direction)) =
                        *warehouse.pos_get_expect(position);
                    *warehouse.pos_get_mut_expect(position) = b'.';
                }

                *position = next;
            }
            Direction::E | Direction::W => {
                // East or West is similar to thin boxes, just keep moving boxes Vec
                let mut box_positions = vec![next];
                let mut next_next = next.move_dir(direction);
                let mut next_next_tile = warehouse.pos_get_expect(next_next);

                while next_next_tile == &b'[' || next_next_tile == &b']' {
                    box_positions.push(next_next);
                    next_next.move_dir_mut(direction);
                    next_next_tile = warehouse.pos_get_expect(next_next);
                }

                // Move if possible, keeping brackets order
                match next_next_tile {
                    b'#' => {}
                    b'.' => {
                        for position in box_positions.into_iter().rev() {
                            *warehouse.pos_get_mut_expect(position.move_dir(direction)) =
                                *warehouse.pos_get_expect(position);
                            *warehouse.pos_get_mut_expect(position) = b'.';
                        }

                        *position = next;
                    }
                    _ => unreachable!("Invalid warehouse tile"),
                }
            }
        }
    })
}

// ------------------------------------------------------------------------------------------------
// Functions

fn final_boxes_coordinates_sum<BoxMove>(
    input: &str,
    wide_boxes: bool,
    box_edge: u8,
    box_move: BoxMove,
) -> usize
where
    BoxMove: Fn(&mut Grid<u8>, &mut Pos, Pos, Direction),
{
    let (warehouse_str, movements) = input.split_once("\n\n").expect("Expected two sections");

    // Widen warehouse if needed
    let warehouse_str = if wide_boxes {
        &warehouse_str
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|c| match c {
                        b'#' => "##",
                        b'.' => "..",
                        b'O' => "[]",
                        b'@' => "@.",
                        _ => unreachable!("Invalid warehouse tile"),
                    })
                    .collect::<String>()
            })
            .join("\n")
    } else {
        warehouse_str
    };

    // Parse grid and movements
    let (mut warehouse, movements) = (
        bytes_grid(warehouse_str),
        movements
            .lines()
            .flat_map(|line| {
                line.bytes()
                    .map(|c| match c {
                        b'v' => Direction::S,
                        b'>' => Direction::E,
                        b'^' => Direction::N,
                        b'<' => Direction::W,
                        _ => panic!("Invalid direction byte"),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    );

    // Get starting position and clear it
    let mut position = Pos::from(
        warehouse
            .indexed_iter()
            .find(|(_, &c)| c == b'@')
            .expect("Expected starting tile")
            .0,
    );

    *warehouse.pos_get_mut_expect(position) = b'.';

    // Follow movements
    for direction in movements {
        let next = position.move_dir(direction);

        match warehouse.pos_get_expect(next) {
            b'.' => position = next,
            b'#' => {}
            _ => box_move(&mut warehouse, &mut position, next, direction),
        }
    }

    // Calculate GPS coordinates sum
    warehouse
        .indexed_iter()
        .filter(|(_, &c)| c == box_edge)
        .fold(0, |acc, ((x, y), _)| acc + 100 * x + y)
}
