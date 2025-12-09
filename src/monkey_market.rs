use itertools::Itertools;
use nalgebra::SMatrix;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

use crate::random_utils::{parse_number, parse_numbers_whitespace};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn buyers_2000th_secret_numbers_sum(input: &str) -> u64 {
    fn bits_to_number(bits: impl IntoIterator<Item = u8>) -> u32 {
        bits.into_iter()
            .enumerate()
            .fold(0, |number, (b, bit)| number | (u32::from(bit & 1) << b))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn apply_transformation(transform_rows: &[u32], number: u32) -> u32 {
        bits_to_number(
            transform_rows
                .iter()
                .map(|row| (row & number).count_ones() as u8 & 1),
        )
    }

    let secret_number_transform_2000 = {
        // Create matrix transform
        let right_shift_matrix = SMatrix::<u8, 24, 24>::from_fn(|r, c| u8::from(r == c + 1));

        let mut secret_number_transform = SMatrix::<u8, 24, 24>::identity();
        secret_number_transform += right_shift_matrix.pow(6) * secret_number_transform;
        secret_number_transform += right_shift_matrix.transpose().pow(5) * secret_number_transform;
        secret_number_transform += right_shift_matrix.pow(11) * secret_number_transform;

        secret_number_transform.apply(|b| *b &= 1);

        // Matrix transform 2000th exponential
        secret_number_transform
            .pow(2000)
            .apply_into(|b| *b &= 1)
            .row_iter()
            .map(|row| bits_to_number(row.into_iter().copied()))
            .collect_vec()
    };

    // Apply matrix transform to every number and sum
    input
        .lines()
        .map(|line| {
            u64::from(apply_transformation(
                &secret_number_transform_2000,
                parse_number(line),
            ))
        })
        .sum()
}

pub fn best_selling_sequence_bananas_count(input: &str) -> u16 {
    #[inline]
    const fn next_secret_number(mut secret_number: i32) -> i32 {
        secret_number ^= secret_number << 6;
        secret_number ^= (secret_number & 0xff_ffff) >> 5;
        secret_number ^= secret_number << 11;
        secret_number & 0xff_ffff
    }

    // Bitwise 4 instructions encoding (base 19 deltas)
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    const fn encode_instructions(previous_instructions: usize, delta: i8) -> usize {
        ((previous_instructions << 5) & 0xfffff) | (delta + 9) as usize
    }

    // Maximum possible 4 instructions encoded (9, 0, 0, 0)
    const MAXIMUM_4_INSTRUCTIONS: usize = 0b10010_01001_01001_01001;

    // Parse secret numbers and calculate chunk size
    let secret_numbers = parse_numbers_whitespace(input).collect_vec();
    let chunk_size = secret_numbers.len().div_ceil(4);

    // Parallel processing of chunks of secret numbers
    secret_numbers
        .par_chunks(chunk_size)
        .map(|secret_numbers| {
            let mut instructions_bananas = vec![0; MAXIMUM_4_INSTRUCTIONS + 1];
            let mut seen_instructions = vec![usize::MAX; MAXIMUM_4_INSTRUCTIONS + 1];

            for (buyer, &secret_number) in secret_numbers.iter().enumerate() {
                let mut secret_number = secret_number;
                let mut instructions = 0;
                let mut previous_bananas = (secret_number % 10) as i8;

                for i in 0..2000 {
                    secret_number = next_secret_number(secret_number);
                    let bananas = (secret_number % 10) as i8;
                    instructions = encode_instructions(instructions, bananas - previous_bananas);
                    previous_bananas = bananas;

                    #[allow(clippy::cast_sign_loss)]
                    if i > 2 && seen_instructions[instructions] != buyer {
                        instructions_bananas[instructions] += bananas as u16;
                        seen_instructions[instructions] = buyer;
                    }
                }
            }

            (instructions_bananas, 0)
        })
        // Combine instructions bananas arrays and find maximum
        .reduce(
            || (vec![0; MAXIMUM_4_INSTRUCTIONS + 1], 0),
            |(mut acc, mut maximum_bananas), (instructions_bananas, _)| {
                for i in 0..acc.len() {
                    acc[i] += instructions_bananas[i];
                    maximum_bananas = maximum_bananas.max(acc[i]);
                }

                (acc, maximum_bananas)
            },
        )
        .1
}
