use atoi::{atoi, FromRadix10SignedChecked};
use grid::Grid;
use itertools::Itertools;
use regex::bytes::Match;
use std::{
    any::type_name,
    fmt::Debug,
    ops::{Add, Mul, Neg},
    str::FromStr,
};

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
                numbers[i] = numbers[i] * ten + N::from(input[byte] - 48);
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
