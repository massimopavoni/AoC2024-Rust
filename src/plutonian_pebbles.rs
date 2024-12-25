use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::random_utils::parse_expect;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn stones_expansion_25_blinks(input: &str) -> usize {
    stones_expansion(input, 25)
}

pub fn stones_expansion_75_blinks(input: &str) -> usize {
    stones_expansion(input, 75)
}

// ------------------------------------------------------------------------------------------------
// Functions

#[allow(clippy::cast_possible_truncation)]
fn stones_expansion(input: &str, blinks: u8) -> usize {
    #[inline]
    fn add_count(map: &mut FxHashMap<u64, usize>, stone: u64, count: usize) {
        *map.entry(stone).or_default() += count;
    }

    let mut stones = FxHashMap::default();
    input
        .split_ascii_whitespace()
        .map(parse_expect::<u64>)
        .for_each(|stone| *stones.entry(stone).or_default() += 1);

    // Map stones to counts and blink many times
    (0..blinks).for_each(|_| {
        let previous_blink = stones.drain().collect_vec();

        for (stone, count) in previous_blink {
            if stone == 0 {
                add_count(&mut stones, 1, count);
            } else {
                let digits_count = stone.ilog10() + 1;

                if digits_count % 2 == 0 {
                    let digits_pow = 10_u64.pow(digits_count / 2);

                    add_count(&mut stones, stone / digits_pow, count);
                    add_count(&mut stones, stone % digits_pow, count);
                } else {
                    add_count(&mut stones, stone * 2024, count);
                }
            }
        }
    });

    stones.values().sum()
}
