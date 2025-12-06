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

#[inline]
pub fn bytes_grid(input: &str) -> Grid<u8> {
    Grid::from(
        input
            .lines()
            .map(|line| line.bytes().collect())
            .collect_vec(),
    )
}

#[inline]
pub fn parse_number_bytes<N>(input: &[u8]) -> N
where
    N: FromLexical,
{
    lexical_core::parse(input)
        .unwrap_or_else(|_| panic!("Expected {}, got {input:?}", type_name::<N>()))
}

#[inline]
pub fn parse_number<N>(input: &str) -> N
where
    N: FromLexical,
{
    parse_number_bytes(input.as_bytes())
}

#[inline]
pub fn parse_numbers_whitespace<N>(input: &str) -> impl Iterator<Item = N>
where
    N: FromLexical,
{
    input.split_ascii_whitespace().map(move |string| {
        lexical_core::parse(string.as_bytes())
            .unwrap_or_else(|_| panic!("Expected {}, got {input}", type_name::<N>()))
    })
}

#[inline]
pub fn parse_numbers<N>(input: &str) -> impl Iterator<Item = N>
where
    N: FromLexical,
{
    input
        .as_bytes()
        .split(|&b| !(b.is_ascii_digit() || b == b'-'))
        .filter(|s| !s.is_empty())
        .map(move |bytes| {
            lexical_core::parse(bytes)
                .unwrap_or_else(|_| panic!("Expected {}, got {input}", type_name::<N>()))
        })
}

#[inline]
pub fn parse_numbers_array<const COUNT: usize, N>(input: &str) -> [N; COUNT]
where
    N: FromLexical,
{
    parse_numbers(input)
        .collect_array()
        .unwrap_or_else(|| panic!("Expected exactly {COUNT} {}", type_name::<N>()))
}
