use crate::random_utils::bytes_grid;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn xmas_occurrences_count(input: &str) -> usize {
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

pub fn x_mas_occurrences_count(input: &str) -> usize {
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
) -> usize {
    let letters_grid = bytes_grid(input);

    // Find origin, filter surrounding slices and count occurrences
    letters_grid.indexed_iter().fold(0, |acc, ((x, y), &c)| {
        if c == origin {
            acc + slices
                .into_iter()
                .map(|slice| {
                    slice.map(|(x_, y_)| {
                        *letters_grid
                            .get(x as isize + x_, y as isize + y_)
                            .unwrap_or(&0)
                    })
                })
                .filter(|slice| patterns.iter().any(|pattern| slice == pattern))
                .count()
        } else {
            acc
        }
    })
}
