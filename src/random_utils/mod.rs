use std::any::type_name;

use grid::Grid;
use itertools::Itertools;
use lexical_core::FromLexical;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

pub mod grid_mask;
pub mod pos;

// ------------------------------------------------------------------------------------------------
// FxHash

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
// Parsers

pub fn bytes_grid(input: &str) -> Grid<u8> {
    Grid::from(
        input
            .lines()
            .map(|line| line.bytes().collect())
            .collect_vec(),
    )
}

pub fn parse_number<N>(input: &str) -> N
where
    N: FromLexical,
{
    lexical_core::parse(input.as_bytes())
        .unwrap_or_else(|_| panic!("Expected {}, got {input}", type_name::<N>()))
}

pub fn parse_numbers<const COUNT: usize, N>(input: &str) -> [N; COUNT]
where
    N: FromLexical,
{
    let mut numbers = [N::default(); COUNT];

    for (number, string) in numbers.iter_mut().zip(
        input
            .as_bytes()
            .split(|&b| !(b.is_ascii_digit() || b == b'-'))
            .filter(|s| !s.is_empty()),
    ) {
        *number = lexical_core::parse(string)
            .unwrap_or_else(|_| panic!("Expected {}, got {input}", type_name::<N>()));
    }

    numbers
}
