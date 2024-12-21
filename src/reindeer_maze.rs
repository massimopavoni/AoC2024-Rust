use pathfinding::directed::astar::{astar, astar_bag};
use std::collections::HashSet;

use crate::random_utils::{
    bytes_grid,
    pos::{Dir, GridPosGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn maze_best_path_score(input: &str) -> isize {
    // Use A* to find shortest path (lowest score)
    maze_astar(input, astar).expect("Expected shortest path").1
}

pub fn maze_best_seats_count(input: &str) -> usize {
    // Use A* to find all shortest paths and then count unique positions
    maze_astar(input, astar_bag)
        .expect("Expected shortest paths")
        .0
        .flatten()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

// ------------------------------------------------------------------------------------------------
// Functions

fn maze_astar<Astar, Solution>(input: &str, astar_function: Astar) -> Option<Solution>
where
    Astar: Fn(
        &(Pos, Dir),
        Box<dyn FnMut(&(Pos, Dir)) -> Vec<((Pos, Dir), isize)>>,
        Box<dyn FnMut(&(Pos, Dir)) -> isize>,
        Box<dyn FnMut(&(Pos, Dir)) -> bool>,
    ) -> Option<Solution>,
{
    let maze = bytes_grid(input);

    // Get start (position, direction) and end position
    let start_direction = Dir::E;
    let (start_position, end_position) = (
        Pos::from(
            maze.indexed_iter()
                .find(|(_, &c)| c == b'S')
                .expect("Expected starting position")
                .0,
        ),
        Pos::from(
            maze.indexed_iter()
                .find(|(_, &c)| c == b'E')
                .expect("Expected ending position")
                .0,
        ),
    );

    // Use A*
    astar_function(
        // Start position and direction
        &(start_position, start_direction),
        // Successors function
        Box::new(move |&(position, direction): &(Pos, Dir)| {
            let (left, forward, right) = (
                position.move_dir(direction.rotate_ccw()),
                position.move_dir(direction),
                position.move_dir(direction.rotate_cw()),
            );

            let mut successors = vec![];

            if maze.pos_get(left).is_some_and(|&c| c != b'#') {
                successors.push(((left, direction.rotate_ccw()), 1001));
            }

            if maze.pos_get(forward).is_some_and(|&c| c != b'#') {
                successors.push(((forward, direction), 1));
            }

            if maze.pos_get(right).is_some_and(|&c| c != b'#') {
                successors.push(((right, direction.rotate_cw()), 1001));
            }

            successors
        }),
        // Heuristic function uses Manhattan distance to end position
        Box::new(move |&(position, _): &(Pos, Dir)| position.manhattan_distance(end_position)),
        // Goal function
        Box::new(move |&(position, _): &(Pos, Dir)| position == end_position),
    )
}
