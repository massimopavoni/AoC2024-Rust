use grid::Grid;
use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn xmas_occurrences_count(input: &str) -> u64 {
    // Find Xs and look for XMAS patterns
    pattern_occurrences(
        input,
        |(x, y)| {
            [
                [(x - 1, y), (x - 2, y), (x - 3, y)],
                [(x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)],
                [(x, y + 1), (x, y + 2), (x, y + 3)],
                [(x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
                [(x + 1, y), (x + 2, y), (x + 3, y)],
                [(x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
                [(x, y - 1), (x, y - 2), (x, y - 3)],
                [(x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)],
            ]
        },
        b'X',
        |slice| slice == b"MAS",
    )
}

pub fn x_mas_occurrences_count(input: &str) -> u64 {
    // Find As and look for the X-MAS pattern
    pattern_occurrences(
        input,
        |(x, y)| {
            [[
                (x - 1, y - 1),
                (x + 1, y + 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
            ]]
        },
        b'A',
        |slice| slice == b"MSMS" || slice == b"MSSM" || slice == b"SMSM" || slice == b"SMMS",
    )
}

// ------------------------------------------------------------------------------------------------
// Functions

fn pattern_occurrences<const M: usize, const N: usize, Idx, Filter>(
    input: &str,
    indices: Idx,
    origin: u8,
    mut filter: Filter,
) -> u64
where
    Idx: Fn((usize, usize)) -> [[(usize, usize); M]; N],
    Filter: FnMut(&Vec<u8>) -> bool,
{
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
        .map(|(index, &c)| {
            if c != origin {
                return 0;
            }

            indices(index)
                .into_iter()
                .map(|slice| {
                    slice
                        .iter()
                        .map(|&(x, y)| *letters_grid.get(x, y).unwrap_or(&0))
                        .collect_vec()
                })
                .filter(&mut filter)
                .count() as u64
        })
        .sum()
}
