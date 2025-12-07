use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::random_utils::{FxHashWithCapacity, parse_numbers_whitespace};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn stones_expansion_25_blinks(input: &str) -> u64 {
    stones_expansion::<25>(input)
}

pub fn stones_expansion_75_blinks(input: &str) -> u64 {
    stones_expansion::<75>(input)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn stones_expansion<const BLINKS: u8>(input: &str) -> u64 {
    #[inline]
    fn add_count(map: &mut FxHashMap<u64, u64>, stone: u64, count: u64) {
        *map.entry(stone).or_default() += count;
    }

    let mut stones = FxHashMap::with_capacity(BLINKS as usize * BLINKS as usize);
    parse_numbers_whitespace(input).for_each(|stone| *stones.entry(stone).or_default() += 1);

    // Map stones to counts and blink many times
    for _ in 0..BLINKS {
        for (stone, count) in stones.drain().collect_vec() {
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
    }

    stones.values().sum()
}
