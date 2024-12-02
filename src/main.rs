use std::{collections::HashMap, sync::LazyLock};

use historian_hysteria::{lists_similarity_score, lists_total_distance};

mod historian_hysteria;
mod red_nosed_reports;

use include_dir::{include_dir, Dir};
use red_nosed_reports::{problem_dampener_safe_reports_count, safe_reports_count};

// ------------------------------------------------------------------------------------------------
// Resources

static RESOURCES_DIR: Dir = include_dir!("src/resources");

macro_rules! get_resource_unsafe {
    ($file:expr) => {
        RESOURCES_DIR
            .get_file($file)
            .expect("Resource not found")
            .contents_utf8()
            .expect("Resource is not UTF-8")
    };
}

static PUZZLE_ANSWERS: LazyLock<HashMap<String, [u64; 2]>> = LazyLock::new(|| {
    get_resource_unsafe!("PuzzleAnswers.out")
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();

            (
                parts[0].to_string(),
                [
                    parts[1]
                        .parse()
                        .expect("Puzzle solution should be an integer"),
                    parts[2]
                        .parse()
                        .expect("Puzzle solution should be an integer"),
                ],
            )
        })
        .collect()
});

// ------------------------------------------------------------------------------------------------
// Functions

fn pretty_solution(puzzle: &str, part: u8, solution: fn(&str) -> u64, input: &str) {
    let solution = solution(input);

    let answer = PUZZLE_ANSWERS.get(puzzle).expect("Resource not found")[part as usize - 1];

    assert!(
        solution == answer,
        "Wrong solution for {puzzle} part {part}: expected {answer}, but got {solution}"
    );

    println!("{part}. {puzzle} -> {answer}");
}

macro_rules! pretty_solution_2 {
    ($day:literal, $puzzle: literal, $solution1:ident $(, $solution2:ident)?) => {
        println!("Day {}", $day);

        let input = get_resource_unsafe!($puzzle.to_string() + ".in");

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
}
