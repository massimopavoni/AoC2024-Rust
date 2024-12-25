use itertools::Itertools;
use num_modular::ModularUnaryOps;
use regex::bytes::Regex;

use crate::random_utils::{pos::Pos, re_match_atoi};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn robots_safety_factor(input: &str) -> usize {
    let (mut qc1, mut qc2, mut qc3, mut qc4) = (0, 0, 0, 0);

    // Find robot positions after 100 seconds and calculate safety factor
    Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")
        .expect("Expected valid regex")
        .captures_iter(input.as_bytes())
        .for_each(|captures| {
            let (position, velocity) = (
                Pos::new(
                    re_match_atoi::<isize>(captures.get(1)),
                    re_match_atoi::<isize>(captures.get(2)),
                ),
                Pos::new(
                    re_match_atoi::<isize>(captures.get(3)),
                    re_match_atoi::<isize>(captures.get(4)),
                ),
            );

            let (x, y) = (
                (position.x + 100 * velocity.x).rem_euclid(101),
                (position.y + 100 * velocity.y).rem_euclid(103),
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

#[allow(clippy::cast_precision_loss)]
pub fn robots_christmas_tree(input: &str) -> usize {
    let mut robots_info = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")
        .expect("Expected valid regex")
        .captures_iter(input.as_bytes())
        .map(|captures| {
            (
                Pos::new(
                    re_match_atoi::<isize>(captures.get(1)),
                    re_match_atoi::<isize>(captures.get(2)),
                ),
                Pos::new(
                    re_match_atoi::<isize>(captures.get(3)),
                    re_match_atoi::<isize>(captures.get(4)),
                ),
            )
        })
        .collect_vec();

    // Find christmas tree by minimizing x and y variance and using chinese remainder theorem
    let (mut min_variance_x, mut min_variance_y) = (f64::MAX, f64::MAX);
    let (mut min_second_x, mut min_second_y) = (0, 0);

    for second in 1..104 {
        let mut average = (0.0, 0.0);

        robots_info = robots_info
            .into_iter()
            .map(|(position, velocity)| {
                let (x, y) = (
                    (position.x + velocity.x).rem_euclid(101),
                    (position.y + velocity.y).rem_euclid(103),
                );

                average.0 += x as f64;
                average.1 += y as f64;
                (Pos::new(x, y), velocity)
            })
            .collect();

        average.0 /= robots_info.len() as f64;
        average.1 /= robots_info.len() as f64;

        let (mut variance_x, mut variance_y) = (0.0, 0.0);

        for &(position, _) in &robots_info {
            variance_x += (position.x as f64 - average.0).powi(2);
            variance_y += (position.y as f64 - average.1).powi(2);
        }

        variance_x /= robots_info.len() as f64;
        variance_y /= robots_info.len() as f64;

        if variance_x < min_variance_x {
            min_variance_x = variance_x;
            min_second_x = second;
        }

        if variance_y < min_variance_y {
            min_variance_y = variance_y;
            min_second_y = second;
        }
    }

    (min_second_x * 103 * 103.invm(&101_usize).expect("Expected modular inverse")
        + min_second_y * 101 * 101.invm(&103_usize).expect("Expected modular inverse"))
        % (101 * 103)
}
