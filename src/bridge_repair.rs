use itertools::Itertools;

use crate::random_utils::parse_expect;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn total_calibration_plus_times(input: &str) -> u64 {
    total_calibration(input, &[SUBTRACT, DIVIDE])
}

pub fn total_calibration_plus_times_concat(input: &str) -> u64 {
    total_calibration(input, &[SUBTRACT, DIVIDE, UNJOIN])
}

// ------------------------------------------------------------------------------------------------
// Functions

const SUBTRACT: fn(u64, u64) -> Option<u64> = |a, b| if a > b { Some(a - b) } else { None };

const DIVIDE: fn(u64, u64) -> Option<u64> = |a, b| if a % b == 0 { Some(a / b) } else { None };

const UNJOIN: fn(u64, u64) -> Option<u64> = |a, b| {
    let pow_10 = 10_u64.pow(b.ilog10() + 1);

    if a % pow_10 == b {
        Some(a / pow_10)
    } else {
        None
    }
};

fn total_calibration<Operation>(input: &str, operations: &[Operation]) -> u64
where
    Operation: Fn(u64, u64) -> Option<u64>,
{
    fn try_solve<Operation>(
        test_value: u64,
        numbers: &[u64],
        current_number: usize,
        operations: &[Operation],
    ) -> bool
    where
        Operation: Fn(u64, u64) -> Option<u64>,
    {
        // Recursively try to reduce test value to first number using the available operations
        if current_number == 0 {
            return test_value == numbers[0];
        }

        operations.iter().any(|operation| {
            operation(test_value, numbers[current_number]).is_some_and(|new_test_value| {
                try_solve(new_test_value, numbers, current_number - 1, operations)
            })
        })
    }

    // Parse calibration equations
    input
        .lines()
        .map(|line| {
            let (test_value, numbers) =
                line.split_once(':').expect("Expected calibration equation");

            (
                parse_expect(test_value),
                numbers
                    .split_ascii_whitespace()
                    .map(parse_expect)
                    .collect_vec(),
            )
        })
        // Sum test values that can be calculated with the given operators
        .map(|(test_value, numbers)| {
            if try_solve(test_value, &numbers, numbers.len() - 1, operations) {
                test_value
            } else {
                0
            }
        })
        .sum()
}
