use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::random_utils::{bytes_grid, pos::Pos, FxHashWithCapacity};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn unique_antinodes_count(input: &str) -> usize {
    // Antinodes are two points dividing the antennas line in 3 equal length segments,
    // ecluding the antennas themselves
    find_antinodes(input, |p1, p2, dp, bounds| {
        let (p0, p3) = (p1 - dp, p2 + dp);

        match (p0.in_bounds(bounds), p3.in_bounds(bounds)) {
            (true, true) => vec![p0, p3],
            (true, false) => vec![p0],
            (false, true) => vec![p3],
            _ => vec![],
        }
    })
}

pub fn unique_resonant_harmonics_antinode_count(input: &str) -> usize {
    // Antinodes are all the points diving the antennas line in many equal length segments
    find_antinodes(input, |p1, p2, dp, bounds| {
        let mut vec = vec![p1, p2];
        let (mut p0, mut p3) = (p1 - dp, p2 + dp);

        while p0.in_bounds(bounds) {
            vec.push(p0);
            p0 -= dp;
        }

        while p3.in_bounds(bounds) {
            vec.push(p3);
            p3 += dp;
        }

        vec
    })
}

// ------------------------------------------------------------------------------------------------
// Functions

#[allow(clippy::cast_possible_wrap)]
fn find_antinodes<Antenna>(input: &str, antenna_pair_function: Antenna) -> usize
where
    Antenna: Fn(Pos, Pos, Pos, (Pos, Pos)) -> Vec<Pos>,
{
    let mut antennas = FxHashMap::with_capacity(48);
    let map_bounds;

    {
        // Read grid, get bounds and antennas positions
        let antennas_grid = bytes_grid(input);
        map_bounds = (
            Pos::from((0_isize, 0_isize)),
            Pos::from((antennas_grid.size().0 - 1, antennas_grid.size().1 - 1)),
        );

        antennas_grid
            .indexed_iter()
            .filter(|(_, &c)| c != b'.')
            .for_each(|((x, y), &c)| {
                antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as isize, y as isize));
            });
    }

    // For each pair of antennas, find the antinodes
    antennas
        .into_values()
        .flat_map(|positions| {
            positions
                .into_iter()
                .tuple_combinations()
                .flat_map(|(p1, p2)| {
                    antenna_pair_function(
                        p1.into(),
                        p2.into(),
                        (p2.0 - p1.0, p2.1 - p1.1).into(),
                        map_bounds,
                    )
                })
        })
        .unique()
        .count()
}
