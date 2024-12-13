use std::{any::type_name, fmt::Debug, str::FromStr};

use grid::Grid;
use itertools::Itertools;

pub mod pos;
pub mod grid_mask;

// ------------------------------------------------------------------------------------------------
// Functions

pub fn parse_expect<F>(string: &str) -> F
where
    F: FromStr,
    <F as FromStr>::Err: Debug,
{
    string
        .parse::<F>()
        .unwrap_or_else(|_| panic!("Expected {}", type_name::<F>()))
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
