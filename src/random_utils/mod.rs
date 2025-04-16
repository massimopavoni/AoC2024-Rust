use std::{
    any::type_name,
    fmt::Debug,
    ops::{Add, Mul, Neg},
    str::FromStr,
};

use grid::Grid;
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

pub mod grid_mask;
pub mod pos;

pub trait FxHashWithCapacity {
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, V> FxHashWithCapacity for FxHashMap<K, V> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher)
    }
}

impl<V> FxHashWithCapacity for FxHashSet<V> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FxBuildHasher)
    }
}

// ------------------------------------------------------------------------------------------------
// Functions

pub fn parse_expect<T>(string: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    string
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Expected {}, got {string}", type_name::<T>()))
}

// ------------------------------------------------------------------------------------------------
// Parsers

pub fn bytes_grid(input: &str) -> Grid<u8> {
    Grid::from(
        input
            .lines()
            .map(|line| line.bytes().collect())
            .collect_vec(),
    )
}

pub fn parse_numbers<const COUNT: usize, N>(input: &str) -> [N; COUNT]
where
    N: From<u8> + Add<Output = N> + Mul<Output = N> + Neg<Output = N> + Default + Copy,
{
    let ten = N::from(10);
    let input = input.as_bytes();

    let (mut numbers, mut i, mut byte, mut negative) = ([N::default(); COUNT], 0, 0, false);

    while byte < input.len() {
        if input[byte] == b'-' {
            negative = true;
            byte += 1;
        }

        if input[byte].is_ascii_digit() {
            while byte < input.len() && input[byte].is_ascii_digit() {
                numbers[i] = numbers[i] * ten + N::from(input[byte] & 0xf);
                byte += 1;
            }

            if negative {
                numbers[i] = -numbers[i];
            }

            i += 1;
        } else {
            negative = false;
            byte += 1;
        }
    }

    numbers
}
