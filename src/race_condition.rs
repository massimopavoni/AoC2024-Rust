use crate::random_utils::{
    bytes_grid,
    pos::{GridPosGet, Pos},
};
use grid::Grid;
use itertools::{iproduct, Itertools};
use pathfinding::directed::dfs::dfs;
use rayon::prelude::*;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn best_2_picos_cheat_paths_count(input: &str) -> usize {
    // Find best cheated paths with cheat limit of 2 picoseconds
    best_cheated_paths(input, 2)
}

pub fn best_20_picos_cheat_paths_count(input: &str) -> usize {
    // Find best cheated paths with cheat limit of 20 picoseconds
    best_cheated_paths(input, 20)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn best_cheated_paths(input: &str, cheat_radius: isize) -> usize {
    // Get racetrack grid and find single path
    let mut racetrack = bytes_grid(input);

    let (start, end) = (
        racetrack
            .indexed_iter()
            .find(|(_, &b)| b == b'S')
            .expect("Expected start position")
            .0,
        racetrack
            .indexed_iter()
            .find(|(_, &b)| b == b'E')
            .expect("Expected end position")
            .0,
    );

    racetrack[start] = b'.';
    racetrack[end] = b'.';

    let (start, end) = (Pos::from(start), Pos::from(end));
    let single_path = dfs(
        start,
        |&position| {
            position
                .neighbors()
                .filter(|&pos| racetrack.pos_get(pos) == Some(&b'.'))
        },
        |&position| position == end,
    )
    .expect("Expected single path");

    // Create second grid of path costs for fast lookup
    let mut path_costs = Grid::new(racetrack.rows(), racetrack.cols());
    path_costs.fill(usize::MAX);

    for (cost, &position) in single_path.iter().enumerate() {
        *path_costs.pos_get_mut_expect(position) = cost;
    }

    // Prepare possible cheat jumps
    let possible_cheat_jumps =
        iproduct!(-cheat_radius..=cheat_radius, -cheat_radius..=cheat_radius)
            .filter(|&(dx, dy)| {
                let (dx, dy) = (dx.abs(), dy.abs());

                (dx > 1 || dy > 1) && dx + dy <= cheat_radius
            })
            .map(Pos::from)
            .collect_vec();
    let minimum_time_save = 100;

    // Count cheated paths with minimum time save
    single_path
        .par_iter()
        .take(single_path.len() - minimum_time_save)
        .enumerate()
        .map(|(cost, &position)| {
            possible_cheat_jumps
                .iter()
                .map(|&jump| {
                    let cheat_position = position + jump;

                    usize::from(
                        path_costs
                            .pos_get(cheat_position)
                            .is_some_and(|&cheat_cost| {
                                cheat_cost != usize::MAX
                                    && cheat_cost
                                        .saturating_sub(cost)
                                        .saturating_sub(position.manhattan_distance(cheat_position))
                                        >= minimum_time_save
                            }),
                    )
                })
                .sum::<usize>()
        })
        .sum()
}
