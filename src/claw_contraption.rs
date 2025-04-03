use crate::random_utils::parse_numbers;

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
    input
        .split("\n\n")
        .filter_map(|machine| {
            let coordinates = parse_numbers::<6, i64>(machine);

            let (x1, x2, y1, y2, px, py) = (
                coordinates[3],
                -coordinates[2],
                -coordinates[1],
                coordinates[0],
                coordinates[4] + PRIZE_OFFSET,
                coordinates[5] + PRIZE_OFFSET,
            );

            let inverse_determinant = x1 * y2 - x2 * y1;
            let (a, b) = (x1 * px + x2 * py, y1 * px + y2 * py);

            if inverse_determinant != 0
                && a % inverse_determinant == 0
                && b % inverse_determinant == 0
            {
                Some(a / inverse_determinant * 3 + b / inverse_determinant)
            } else {
                None
            }
        })
        .sum()
}
