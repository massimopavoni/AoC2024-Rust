use regex::bytes::Regex;

use crate::random_utils::re_match_atoi;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn fewest_tokens_all_prizes_small(input: &str) -> i64 {
    // Solve claw machines equations with no offset
    fewest_tokens_all_prizes::<0>(input)
}

pub fn fewest_tokens_all_prizes_huge(input: &str) -> i64 {
    // Solve claw machines equations with huge offset
    fewest_tokens_all_prizes::<10_000_000_000_000>(input)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn fewest_tokens_all_prizes<const PRIZE_OFFSET: i64>(input: &str) -> i64 {
    // Parse input and solve integer linear system by inverting the matrix
    Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .expect("Invalid regex")
    .captures_iter(input.as_bytes())
    .filter_map(|captures| {
        let (x1, y1, x2, y2, p1, p2) = (
            re_match_atoi::<i64>(captures.get(1)),
            re_match_atoi::<i64>(captures.get(2)),
            re_match_atoi::<i64>(captures.get(3)),
            re_match_atoi::<i64>(captures.get(4)),
            re_match_atoi::<i64>(captures.get(5)) + PRIZE_OFFSET,
            re_match_atoi::<i64>(captures.get(6)) + PRIZE_OFFSET,
        );

        let (x1, x2, y1, y2) = (y2, -x2, -y1, x1);
        let inverse_determinant = x1 * y2 - x2 * y1;
        let (a, b) = (x1 * p1 + x2 * p2, y1 * p1 + y2 * p2);

        if inverse_determinant != 0 && a % inverse_determinant == 0 && b % inverse_determinant == 0
        {
            Some(a / inverse_determinant * 3 + b / inverse_determinant)
        } else {
            None
        }
    })
    .sum()
}
