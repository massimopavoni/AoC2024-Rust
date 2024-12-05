use grid::Grid;
use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn xmas_occurrences_count(input: &str) -> u64 {
    // Find Xs and look for XMAS patterns
    pattern_occurrences(
        input,
        [
            [(-1, 0), (-2, 0), (-3, 0)],
            [(-1, 1), (-2, 2), (-3, 3)],
            [(0, 1), (0, 2), (0, 3)],
            [(1, 1), (2, 2), (3, 3)],
            [(1, 0), (2, 0), (3, 0)],
            [(1, -1), (2, -2), (3, -3)],
            [(0, -1), (0, -2), (0, -3)],
            [(-1, -1), (-2, -2), (-3, -3)],
        ],
        b'X',
        &[*b"MAS"],
    )
}

pub fn x_mas_occurrences_count(input: &str) -> u64 {
    // Find As and look for the X-MAS pattern
    pattern_occurrences(
        input,
        [[(-1, -1), (1, 1), (1, -1), (-1, 1)]],
        b'A',
        &[*b"MSMS", *b"MSSM", *b"SMSM", *b"SMMS"],
    )
}

// ------------------------------------------------------------------------------------------------
// Functions

#[allow(clippy::cast_possible_wrap)]
fn pattern_occurrences<const M: usize, const N: usize>(
    input: &str,
    slices: [[(isize, isize); M]; N],
    origin: u8,
    patterns: &[[u8; M]],
) -> u64 {
    // Create bytes grid from input
    let letters_grid = Grid::from(
        input
            .lines()
            .map(|line| line.bytes().collect())
            .collect_vec(),
    );

    // Find origin, filter surrounding slices and count occurrences
    letters_grid
        .indexed_iter()
        .map(|((x, y), &c)| {
            if c != origin {
                return 0;
            }

            slices
                .into_iter()
                .map(|slice| {
                    slice
                        .into_iter()
                        .map(|(x_, y_)| {
                            *letters_grid
                                .get(x as isize + x_, y as isize + y_)
                                .unwrap_or(&0)
                        })
                        .collect_vec()
                })
                .filter(|slice| patterns.iter().any(|pattern| slice == pattern))
                .count() as u64
        })
        .sum()
}
