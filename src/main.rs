use bridge_repair::{total_calibration_plus_times, total_calibration_plus_times_concat};
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, sync::LazyLock};

mod random_utils;

mod bridge_repair;
mod ceres_search;
mod guard_gallivant;
mod historian_hysteria;
mod mull_it_over;
mod print_queue;
mod red_nosed_reports;

use ceres_search::{x_mas_occurrences_count, xmas_occurrences_count};
use guard_gallivant::{possible_obstruction_loops_count, unique_guard_positions_count};
use historian_hysteria::{lists_similarity_score, lists_total_distance};
use mull_it_over::{do_dont_multiplications_sum, multiplications_sum};
use print_queue::{fixed_invalid_updates_middle_sum, valid_updates_middle_sum};
use red_nosed_reports::{problem_dampener_safe_reports_count, safe_reports_count};

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

fn pretty_solution<R>(puzzle: &str, part: usize, solution: fn(&str) -> R, input: &str)
where
    R: Display + PartialEq,
{
    let solution = solution(input);

    let answer = PUZZLE_ANSWERS.get(puzzle).expect("Resource not found")[part - 1];

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
}
