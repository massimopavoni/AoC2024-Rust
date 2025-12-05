use grid::Grid;
use itertools::{Itertools, iproduct};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::random_utils::{
    bytes_grid,
    pos::{GridPosGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn best_2_picos_cheat_paths_count(input: &str) -> usize {
    // Find best cheated paths with cheat limit of 2 picoseconds
    best_cheat_paths_count::<2>(input)
}

pub fn best_20_picos_cheat_paths_count(input: &str) -> usize {
    // Find best cheated paths with cheat limit of 20 picoseconds
    best_cheat_paths_count::<20>(input)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn best_cheat_paths_count<const CHEAT_RADIUS: isize>(input: &str) -> usize {
    // Get racetrack grid and find single path
    let mut racetrack = bytes_grid(input);

    let (position, end) = (
        racetrack
            .indexed_iter()
            .find(|&(_, &b)| b == b'S')
            .expect("Expected start position")
            .0,
        racetrack
            .indexed_iter()
            .find(|&(_, &b)| b == b'E')
            .expect("Expected end position")
            .0,
    );

    racetrack[position] = b'.';
    racetrack[end] = b'.';

    let (mut position, end) = (Pos::from(position), Pos::from(end));
    let mut single_path = Vec::with_capacity(position.manhattan_distance(end));
    single_path.push(position);

    while position != end {
        *racetrack.pos_index_mut(position) = b'#';

        position = position
            .adjacent()
            .find(|&pos| racetrack.pos_index(pos) == &b'.')
            .expect("Expected path");

        single_path.push(position);
    }

    // Create second grid of path costs for fast lookup
    let mut path_costs = Grid::new(racetrack.rows(), racetrack.cols());
    path_costs.fill(usize::MAX);

    for (cost, &position) in single_path.iter().enumerate() {
        *path_costs.pos_index_mut(position) = cost;
    }

    // Prepare possible cheat jumps
    let possible_cheat_jumps =
        iproduct!(-CHEAT_RADIUS..=CHEAT_RADIUS, -CHEAT_RADIUS..=CHEAT_RADIUS)
            .filter(|&(dx, dy)| {
                let (dx, dy) = (dx.abs(), dy.abs());

                (dx > 1 || dy > 1) && dx + dy <= CHEAT_RADIUS
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
