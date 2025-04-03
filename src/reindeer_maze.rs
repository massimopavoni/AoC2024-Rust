use std::rc::Rc;

use itertools::Itertools;
use pathfinding::directed::astar::{astar, astar_bag};

use crate::random_utils::{
    bytes_grid,
    pos::{Dir, GridPosGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn maze_best_path_score(input: &str) -> usize {
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
        .unique()
        .count()
}

// ------------------------------------------------------------------------------------------------
// Functions

fn maze_astar<Astar, Solution>(input: &str, astar_function: Astar) -> Option<Solution>
where
    Astar: Fn(
        &(Pos, Dir),
        Box<dyn FnMut(&(Pos, Dir)) -> Box<dyn Iterator<Item = ((Pos, Dir), usize)>>>,
        Box<dyn FnMut(&(Pos, Dir)) -> usize>,
        Box<dyn FnMut(&(Pos, Dir)) -> bool>,
    ) -> Option<Solution>,
{
    let maze = Rc::new(bytes_grid(input));

    // Get start (position, direction) and end position
    let start_direction = Dir::E;
    let (start_position, end_position) = (
        Pos::from(
            maze.indexed_iter()
                .find(|&(_, &c)| c == b'S')
                .expect("Expected starting position")
                .0,
        ),
        Pos::from(
            maze.indexed_iter()
                .find(|&(_, &c)| c == b'E')
                .expect("Expected ending position")
                .0,
        ),
    );

    // Use A*
    astar_function(
        // Start position and direction
        &(start_position, start_direction),
        // Successors function
        Box::new(move |&(position, direction)| {
            let maze = Rc::clone(&maze);

            Box::new(
                [
                    (direction.rotate_ccw(), 1001),
                    (direction, 1),
                    (direction.rotate_cw(), 1001),
                ]
                .map(|(dir, cost)| (position.move_dir(dir), dir, cost))
                .into_iter()
                .filter_map(move |(next_pos, next_dir, cost)| {
                    maze.pos_get(next_pos).and_then(|&c| {
                        if c == b'#' {
                            None
                        } else {
                            Some(((next_pos, next_dir), cost))
                        }
                    })
                }),
            )
        }),
        // Heuristic function uses Manhattan distance to end position
        Box::new(move |&(position, _)| position.manhattan_distance(end_position)),
        // Goal function
        Box::new(move |&(position, _)| position == end_position),
    )
}
