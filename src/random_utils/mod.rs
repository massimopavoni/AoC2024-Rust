use grid::Grid;
use itertools::Itertools;
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

pub trait ParseNum: Copy + Default {
    fn add_digit(self, digit: u8) -> Self;

    fn negate(self) -> Self;
}

macro_rules! parse_num_impl {
    ($type:ty, signed) => {
        impl ParseNum for $type {
            fn add_digit(self, digit: u8) -> Self {
                self * 10 + digit as $type
            }

            fn negate(self) -> Self {
                -self
            }
        }
    };

    ($type:ty, unsigned) => {
        impl ParseNum for $type {
            fn add_digit(self, digit: u8) -> Self {
                self * 10 + digit as $type
            }

            fn negate(self) -> Self {
                self
            }
        }
    };
}

parse_num_impl!(i16, signed);
parse_num_impl!(i32, signed);
parse_num_impl!(i64, signed);
parse_num_impl!(isize, signed);

parse_num_impl!(u16, unsigned);
parse_num_impl!(u32, unsigned);
parse_num_impl!(u64, unsigned);
parse_num_impl!(usize, unsigned);

pub fn parse_number<N>(input: &str) -> N
where
    N: ParseNum,
{
    let input = input.as_bytes();
    let (mut number, mut byte, mut negative) = (N::default(), 0, false);

    if input[byte] == b'-' {
        negative = true;
        byte += 1;
    }

    while byte < input.len() && input[byte].is_ascii_digit() {
        number = number.add_digit(input[byte] & 0xf);
        byte += 1;
    }

    if negative { number.negate() } else { number }
}

pub fn parse_numbers<const COUNT: usize, N>(input: &str) -> [N; COUNT]
where
    N: ParseNum,
{
    let input = input.as_bytes();

    let (mut numbers, mut i, mut byte, mut negative) = ([N::default(); COUNT], 0, 0, false);

    while byte < input.len() {
        if input[byte] == b'-' {
            negative = true;
            byte += 1;
        }

        if input[byte].is_ascii_digit() {
            while byte < input.len() && input[byte].is_ascii_digit() {
                numbers[i] = numbers[i].add_digit(input[byte] & 0xf);
                byte += 1;
            }

            if negative {
                numbers[i] = numbers[i].negate();
            }

            i += 1;
        } else {
            negative = false;
            byte += 1;
        }
    }

    numbers
}
