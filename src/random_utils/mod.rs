use atoi::{atoi, FromRadix10SignedChecked};
use grid::Grid;
use itertools::Itertools;
use regex::bytes::Match;
use std::{any::type_name, fmt::Debug, str::FromStr};

pub mod grid_mask;
pub mod pos;

// ------------------------------------------------------------------------------------------------
// Functions

pub fn parse_expect<F>(string: &str) -> F
where
    F: FromStr,
    <F as FromStr>::Err: Debug,
{
    string
        .parse::<F>()
        .unwrap_or_else(|_| panic!("Expected {}, got {string}", type_name::<F>()))
}

// ------------------------------------------------------------------------------------------------
// Parsers

#[inline]
pub fn re_match_atoi<N>(match_: Option<Match<'_>>) -> N
where
    N: FromRadix10SignedChecked,
{
    atoi::<N>(match_.expect("Expected capture").as_bytes()).expect("Expected valid integer")
}

pub fn bytes_grid(input: &str) -> Grid<u8> {
    Grid::from(
        input
            .lines()
            .map(|line| line.bytes().collect())
            .collect_vec(),
    )
}
