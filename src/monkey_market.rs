use std::sync::LazyLock;

use itertools::Itertools;
use nalgebra::SMatrix;

use crate::random_utils::parse_expect;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn buyers_2000th_secret_numbers_sum(input: &str) -> u64 {
    let secret_number_transform_2000 = SECRET_NUMBER_TRANSFORM
        .pow(2000)
        .apply_into(|b| *b &= 1)
        .row_iter()
        .map(|row| bits_to_number(row.into_iter().copied()))
        .collect_vec();

    input
        .lines()
        .map(|line| {
            u64::from(apply_transformation(
                &secret_number_transform_2000,
                parse_expect(line),
            ))
        })
        .sum()
}

pub fn best_selling_sequence_bananas_count(input: &str) -> u64 {
    0
}

// ------------------------------------------------------------------------------------------------
// Functions

static SECRET_NUMBER_TRANSFORM: LazyLock<SMatrix<u8, 24, 24>> = LazyLock::new(|| {
    let right_shift_matrix = SMatrix::<u8, 24, 24>::from_fn(|r, c| u8::from(r == c + 1));

    let mut secret_number_transform = SMatrix::<u8, 24, 24>::identity();
    secret_number_transform += right_shift_matrix.pow(6) * secret_number_transform;
    secret_number_transform += right_shift_matrix.transpose().pow(5) * secret_number_transform;
    secret_number_transform += right_shift_matrix.pow(11) * secret_number_transform;

    secret_number_transform.apply_into(|b| *b &= 1)
});

#[allow(clippy::cast_possible_truncation)]
fn apply_transformation(transform_rows: &[u32], number: u32) -> u32 {
    bits_to_number(
        transform_rows
            .iter()
            .map(|row| (row & number).count_ones() as u8 & 1),
    )
}

fn bits_to_number(bits: impl IntoIterator<Item = u8>) -> u32 {
    bits.into_iter()
        .enumerate()
        .fold(0, |number, (b, bit)| number | (u32::from(bit & 1) << b))
}
