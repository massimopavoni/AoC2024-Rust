use include_dir::{include_dir, Dir};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, sync::LazyLock};

mod random_utils;

mod bridge_repair;
mod ceres_search;
mod claw_contraption;
mod disk_fragmenter;
mod garden_groups;
mod guard_gallivant;
mod historian_hysteria;
mod hoof_it;
mod mull_it_over;
mod plutonian_pebbles;
mod print_queue;
mod red_nosed_reports;
mod resonant_collinearity;
mod restroom_redoubt;

use bridge_repair::{total_calibration_plus_times, total_calibration_plus_times_concat};
use ceres_search::{x_mas_occurrences_count, xmas_occurrences_count};
use claw_contraption::{fewest_tokens_all_prizes_huge, fewest_tokens_all_prizes_small};
use disk_fragmenter::{compact_disk_checksum, whole_files_compact_disk_checksum};
use garden_groups::{fences_total_cost_perimeter, fences_total_cost_sides};
use guard_gallivant::{possible_obstruction_loops_count, unique_guard_positions_count};
use historian_hysteria::{lists_similarity_score, lists_total_distance};
use hoof_it::{trailheads_total_rating, trailheads_total_score};
use mull_it_over::{do_dont_multiplications_sum, multiplications_sum};
use plutonian_pebbles::{stones_expansion_25_blinks, stones_expansion_75_blinks};
use print_queue::{fixed_invalid_updates_middle_sum, valid_updates_middle_sum};
use red_nosed_reports::{problem_dampener_safe_reports_count, safe_reports_count};
use resonant_collinearity::{unique_antinodes_count, unique_resonant_harmonics_antinode_count};
use restroom_redoubt::{robots_christmas_tree, robots_safety_factor};

// ------------------------------------------------------------------------------------------------
// Resources

static RESOURCES_DIR: Dir = include_dir!("src/resources");

macro_rules! get_resource {
    ($file:expr) => {
        RESOURCES_DIR
            .get_file($file)
            .expect("Resource not found")
            .contents_utf8()
            .expect("Resource is not UTF-8")
    };
}

static PUZZLE_ANSWERS: LazyLock<HashMap<&str, [&str; 2]>> = LazyLock::new(|| {
    get_resource!("PuzzleAnswers.out")
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect_vec();

            (parts[0], [parts[1], parts[2]])
        })
        .collect()
});

// ------------------------------------------------------------------------------------------------
// Functions

#[inline]
fn pretty_solution<R>(puzzle: &str, part: usize, solution: fn(&str) -> R, input: &str)
where
    R: Display + PartialEq,
{
    let solution = solution(input);

    let answer = PUZZLE_ANSWERS.get(puzzle).expect("Puzzle answer not found")[part - 1];

    assert!(
        solution.to_string() == answer,
        "Wrong solution for {puzzle} part {part}: expected {answer}, but got {solution}"
    );

    println!("{part} -> {answer}");
}

macro_rules! pretty_solution_2 {
    ($day:literal, $puzzle: literal, $solution1:ident $(,$solution2:ident)?) => {
        println!("Day {}: {}", $day, $puzzle);

        let input = get_resource!($puzzle.to_string() + ".in");

        pretty_solution($puzzle, 1, $solution1, input);

        $(pretty_solution($puzzle, 2, $solution2, input);)?

        println!();
    };
}

// ------------------------------------------------------------------------------------------------
// Exports

pub fn main() {
    println!("AoC 2024 - Rust\n");

    pretty_solution_2!(
        1,
        "HistorianHysteria",
        lists_total_distance,
        lists_similarity_score
    );

    pretty_solution_2!(
        2,
        "RedNosedReports",
        safe_reports_count,
        problem_dampener_safe_reports_count
    );

    pretty_solution_2!(
        3,
        "MullItOver",
        multiplications_sum,
        do_dont_multiplications_sum
    );

    pretty_solution_2!(
        4,
        "CeresSearch",
        xmas_occurrences_count,
        x_mas_occurrences_count
    );

    pretty_solution_2!(
        5,
        "PrintQueue",
        valid_updates_middle_sum,
        fixed_invalid_updates_middle_sum
    );

    pretty_solution_2!(
        6,
        "GuardGallivant",
        unique_guard_positions_count,
        possible_obstruction_loops_count
    );

    pretty_solution_2!(
        7,
        "BridgeRepair",
        total_calibration_plus_times,
        total_calibration_plus_times_concat
    );

    pretty_solution_2!(
        8,
        "ResonantCollinearity",
        unique_antinodes_count,
        unique_resonant_harmonics_antinode_count
    );

    pretty_solution_2!(
        9,
        "DiskFragmenter",
        compact_disk_checksum,
        whole_files_compact_disk_checksum
    );

    pretty_solution_2!(
        10,
        "HoofIt",
        trailheads_total_score,
        trailheads_total_rating
    );

    pretty_solution_2!(
        11,
        "PlutonianPebbles",
        stones_expansion_25_blinks,
        stones_expansion_75_blinks
    );

    pretty_solution_2!(
        12,
        "GardenGroups",
        fences_total_cost_perimeter,
        fences_total_cost_sides
    );

    pretty_solution_2!(
        13,
        "ClawContraption",
        fewest_tokens_all_prizes_small,
        fewest_tokens_all_prizes_huge
    );

    pretty_solution_2!(
        14,
        "RestroomRedoubt",
        robots_safety_factor,
        robots_christmas_tree
    );
}
