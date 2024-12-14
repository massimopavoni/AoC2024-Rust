use itertools::Itertools;
use regex::bytes::Regex;

use crate::random_utils::re_match_atoi;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn robots_safety_factor(input: &str) -> u64 {
    let (mut qc1, mut qc2, mut qc3, mut qc4) = (0, 0, 0, 0);

    // Find robot positions after 100 seconds and calculate safety factor
    Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")
        .expect("Invalid regex")
        .captures_iter(input.as_bytes())
        .for_each(|captures| {
            let (x, y, vx, vy) = (
                re_match_atoi::<i64>(captures.get(1)),
                re_match_atoi::<i64>(captures.get(2)),
                re_match_atoi::<i64>(captures.get(3)),
                re_match_atoi::<i64>(captures.get(4)),
            );

            let (x, y) = (
                (x + 100 * vx).rem_euclid(101),
                (y + 100 * vy).rem_euclid(103),
            );

            match (x, y) {
                (0..=49, 0..=50) => qc1 += 1,
                (0..=49, 52..=103) => qc2 += 1,
                (51..=101, 0..=50) => qc3 += 1,
                (51..=101, 52..=103) => qc4 += 1,
                _ => {}
            }
        });

    qc1 * qc2 * qc3 * qc4
}

pub fn robots_christmas_tree(input: &str) -> i64 {
    let robots_info = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")
        .expect("Invalid regex")
        .captures_iter(input.as_bytes())
        .map(|captures| {
            (
                re_match_atoi::<i64>(captures.get(1)),
                re_match_atoi::<i64>(captures.get(2)),
                re_match_atoi::<i64>(captures.get(3)),
                re_match_atoi::<i64>(captures.get(4)),
            )
        })
        .collect_vec();

    // Find robot positions that minimize manhattan distance from the center
    (101..101 * 103)
        .map(|second| {
            let mut total_manhattan_distance = 0;

            for (x, y, vx, vy) in &robots_info {
                let (x, y) = (
                    (x + second * vx).rem_euclid(101),
                    (y + second * vy).rem_euclid(103),
                );

                total_manhattan_distance += (x - 50).abs() + (y - 51).abs();
            }

            (total_manhattan_distance, second)
        })
        .min()
        .expect("Expected christmas tree")
        .1
}
