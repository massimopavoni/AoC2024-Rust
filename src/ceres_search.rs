use crate::random_utils::bytes_grid;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn xmas_occurrences_count(input: &str) -> usize {
    // Find Xs and look for XMAS patterns
    pattern_occurrences::<3, 8, b'X'>(
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
        &[*b"MAS"],
    )
}

pub fn x_mas_occurrences_count(input: &str) -> usize {
    // Find As and look for X-MAS patterns
    pattern_occurrences::<4, 1, b'A'>(
        input,
        [[(-1, -1), (1, 1), (1, -1), (-1, 1)]],
        &[*b"MSMS", *b"MSSM", *b"SMSM", *b"SMMS"],
    )
}

// ------------------------------------------------------------------------------------------------
// Functions

fn pattern_occurrences<const M: usize, const N: usize, const ORIGIN: u8>(
    input: &str,
    slices: [[(isize, isize); M]; N],
    patterns: &[[u8; M]],
) -> usize {
    let letters = bytes_grid(input);

    // Find origin, filter surrounding slices and count occurrences
    letters.indexed_iter().fold(0, |xmas, ((x, y), &c)| {
        if c == ORIGIN {
            xmas + slices
                .into_iter()
                .map(|slice| {
                    #[allow(clippy::cast_possible_wrap)]
                    slice.map(|(x_, y_)| {
                        *letters.get(x as isize + x_, y as isize + y_).unwrap_or(&0)
                    })
                })
                .filter(|slice| patterns.iter().any(|pattern| slice == pattern))
                .count()
        } else {
            xmas
        }
    })
}
