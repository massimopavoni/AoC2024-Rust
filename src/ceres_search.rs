use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};
use itertools::Itertools;
use ndarray::{Array2, ArrayView2};

// ------------------------------------------------------------------------------------------------
// Constants

const XMAS: [&u8; 4] = [&b'X', &b'M', &b'A', &b'S'];
const SAMX: [&u8; 4] = [&b'S', &b'A', &b'M', &b'X'];

const MAS: &[u8; 3] = b"MAS";
const SAM: &[u8; 3] = b"SAM";

// ------------------------------------------------------------------------------------------------
// Exports

pub fn xmas_occurrences_count(input: &str) -> u64 {
    // Get Vec<Vec<u8>> from input
    let letters_grid = input
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();

    // Count the number of XMAS in every direction using the diagonal crate functions
    [straight_x, straight_y, diagonal_pos_pos, diagonal_pos_neg]
        .into_iter()
        .map(|function| {
            function(&letters_grid)
                .into_iter()
                .map(|vec| vec.windows(4).filter(|&w| w == XMAS || w == SAMX).count() as u64)
                .sum::<u64>()
        })
        .sum()
}

pub fn x_mas_occurrences_count(input: &str) -> u64 {
    fn is_x_mas(window: ArrayView2<u8>) -> bool {
        // Get rows for diagonals and check if window is an X-MAS
        let (row1, row2, row3) = (
            window.row(0).to_slice().expect("Expected bytes slice"),
            window.row(1).to_slice().expect("Expected bytes slice"),
            window.row(2).to_slice().expect("Expected bytes slice"),
        );

        let main_diag = &[row1[0], row2[1], row3[2]];
        let anti_diag = &[row1[2], row2[1], row3[0]];

        (main_diag == MAS || main_diag == SAM) && (anti_diag == MAS || anti_diag == SAM)
    }

    // Create Array2 from flat Vec and count 3x3 windows with an X-MAS
    let input_lines = input.lines().collect_vec();

    let letters_grid = Array2::from_shape_vec(
        (input_lines.len(), input_lines[0].len()),
        input_lines
            .into_iter()
            .flat_map(|line| line.bytes())
            .collect(),
    )
    .expect("Expected rectangular bytes grid");

    letters_grid
        .windows((3, 3))
        .into_iter()
        .filter(|&w| is_x_mas(w))
        .count() as u64
}
