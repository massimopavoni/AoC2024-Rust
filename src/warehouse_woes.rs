use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::random_utils::pos::{Direction, Pos};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn final_thin_boxes_coordinates_sum(input: &str) -> isize {
    final_boxes_coordinates_sum(
        input,
        false,
        b'O',
        |warehouse, position, next, direction| {
            let mut next_next = next.move_dir(direction);

            while warehouse.get(&next_next) == Some(&b'O') {
                next_next.move_dir_mut(direction);
            }

            match warehouse.get(&next_next) {
                Some(b'#') => {}
                None => {
                    warehouse.remove(&next);
                    warehouse.insert(next_next, b'O');
                    *position = next;
                }
                _ => unreachable!("Invalid warehouse tile"),
            }
        },
    )
}

pub fn final_wide_boxes_coordinates_sum(input: &str) -> isize {
    final_boxes_coordinates_sum(input, true, b'[', |warehouse, position, next, direction| {
        match direction {
            Direction::S | Direction::N => {
                let mut box_positions = vec![HashSet::from([next])];

                let next_side = next.move_dir(if warehouse.get(&next) == Some(&b'[') {
                    Direction::E
                } else {
                    Direction::W
                });

                box_positions[0].insert(next_side);

                loop {
                    let mut more_boxes = HashSet::new();

                    for &box_position in box_positions.last().expect("Expected warehouse tiles") {
                        let another_box_position = box_position.move_dir(direction);

                        match warehouse.get(&another_box_position) {
                            None => {}
                            Some(b'[') => {
                                more_boxes.insert(another_box_position);
                                more_boxes.insert(another_box_position.move_dir(Direction::E));
                            }
                            Some(b']') => {
                                more_boxes.insert(another_box_position);
                                more_boxes.insert(another_box_position.move_dir(Direction::W));
                            }
                            Some(b'#') => return,
                            _ => unreachable!("Invalid warehouse tile"),
                        }
                    }

                    if more_boxes.is_empty() {
                        break;
                    }

                    box_positions.push(more_boxes);
                }

                for position in box_positions.into_iter().rev().flatten() {
                    let box_tile = warehouse
                        .remove(&position)
                        .expect("Expected warehouse tile");
                    warehouse.insert(position.move_dir(direction), box_tile);
                }

                *position = next;
            }
            Direction::E | Direction::W => {
                let mut box_positions = vec![next];
                let mut next_next = next.move_dir(direction);
                let mut next_next_tile = warehouse.get(&next_next);

                while next_next_tile == Some(&b'[') || next_next_tile == Some(&b']') {
                    box_positions.push(next_next);
                    next_next.move_dir_mut(direction);
                    next_next_tile = warehouse.get(&next_next);
                }

                match next_next_tile {
                    Some(b'#') => {}
                    None => {
                        for position in box_positions.into_iter().rev() {
                            let box_tile = warehouse
                                .remove(&position)
                                .expect("Expected warehouse tile");
                            warehouse.insert(position.move_dir(direction), box_tile);
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
) -> isize
where
    BoxMove: Fn(&mut HashMap<Pos, u8>, &mut Pos, Pos, Direction),
{
    let (warehouse_str, movements) = input.split_once("\n\n").expect("Expected two sections");

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

    let (mut warehouse, mut position) = (HashMap::new(), Pos::new(0, 0));

    for (x, line) in warehouse_str.lines().enumerate() {
        for (y, c) in line.bytes().enumerate() {
            match c {
                b'#' | b'O' | b'[' | b']' => {
                    warehouse.insert(Pos::from((x, y)), c);
                }
                b'.' => {}
                b'@' => {
                    position = Pos::from((x, y));
                }
                _ => unreachable!("Invalid warehouse tile"),
            }
        }
    }

    let movements = movements
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
        .collect_vec();

    for direction in movements {
        let next = position.move_dir(direction);

        match warehouse.get(&next) {
            None => position = next,
            Some(b'#') => {}
            _ => box_move(&mut warehouse, &mut position, next, direction),
        }
    }

    warehouse
        .into_iter()
        .filter(|&(_, c)| c == box_edge)
        .fold(0, |acc, (position, _)| acc + 100 * position.x + position.y)
}
