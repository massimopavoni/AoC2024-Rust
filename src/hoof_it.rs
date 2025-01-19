use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::convert::identity;

use crate::random_utils::{bytes_grid, pos::Pos, FxHashWithCapacity};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn trailheads_total_score(input: &str) -> usize {
    // Find trailheads total score based on unique peaks reachable
    trailheads_total(
        input,
        || FxHashSet::with_capacity(8),
        |visited_peaks, position| {
            visited_peaks.insert(position);
        },
        |visited_peaks| visited_peaks.len(),
    )
}

pub fn trailheads_total_rating(input: &str) -> usize {
    // Find trailheads total rating based on total unique paths to some peaks
    trailheads_total(
        input,
        || 0,
        |visited_peaks, _| *visited_peaks += 1,
        identity,
    )
}

// ------------------------------------------------------------------------------------------------
// Functions

fn trailheads_total<I, Init, Func, Total>(
    input: &str,
    init: Init,
    peaks_function: Func,
    peaks_total: Total,
) -> usize
where
    Init: Fn() -> I,
    Func: Fn(&mut I, Pos),
    Total: Fn(I) -> usize,
{
    let topographic_map = bytes_grid(input);

    // Start from level 0 and BFS keeping track of peaks reached
    topographic_map
        .indexed_iter()
        .filter(|(_, &v)| v == b'0')
        .map(|(position, _)| {
            let mut positions: Vec<Pos> = Vec::with_capacity(24);
            positions.push(position.into());
            let mut visited_peaks = init();

            while !positions.is_empty() {
                let map_ref = &topographic_map;

                positions = positions
                    .into_iter()
                    .flat_map(|pos| {
                        pos.neighbors().filter(move |&next| {
                            map_ref.get(next.x, next.y)
                                == Some(
                                    &(map_ref
                                        .get(pos.x, pos.y)
                                        .expect("Expected position inside map")
                                        + 1),
                                )
                        })
                    })
                    .filter(|&pos| {
                        if topographic_map.get(pos.x, pos.y) == Some(&b'9') {
                            peaks_function(&mut visited_peaks, pos);
                            false
                        } else {
                            true
                        }
                    })
                    .collect_vec();
            }

            peaks_total(visited_peaks)
        })
        .sum()
}
