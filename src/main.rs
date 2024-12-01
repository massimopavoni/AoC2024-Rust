use std::{collections::HashMap, sync::LazyLock};

use historian_hysteria::{lists_similarity_score, lists_total_distance};

mod historian_hysteria;

use include_dir::{include_dir, Dir};

// ------------------------------------------------------------------------------------------------
// Resources

static RESOURCES_DIR: Dir = include_dir!("src/resources");

macro_rules! get_resource_unsafe {
    ($file:expr) => {
        RESOURCES_DIR
            .get_file($file)
            .unwrap()
            .contents_utf8()
            .unwrap()
    };
}

static PUZZLE_ANSWERS: LazyLock<HashMap<String, [u64; 2]>> = LazyLock::new(|| {
    get_resource_unsafe!("PuzzleAnswers.out")
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            (
                parts[0].to_string(),
                [parts[1].parse().unwrap(), parts[2].parse().unwrap()],
            )
        })
        .collect()
});

// ------------------------------------------------------------------------------------------------
// Functions

fn pretty_solution(puzzle: &str, part: u8, solution: fn(&str) -> u64) {
    let solution = solution(get_resource_unsafe!(puzzle.to_string() + ".in"));
    let answer = PUZZLE_ANSWERS.get(puzzle).unwrap()[part as usize - 1];
    if solution != answer {
        panic!(
            "Wrong solution for {} part {}: expected {}, but got {}",
            puzzle, part, answer, solution,
        );
    }
    println!("{}. {} -> {}", part, puzzle, answer);
}

macro_rules! pretty_solution_2 {
    ($day:literal, $puzzle: literal, $solution1:ident $(, $solution2:ident)?) => {
        println!("Day {}", $day);
        pretty_solution($puzzle, 1, $solution1);
        $(pretty_solution($puzzle, 2, $solution2);)?
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
}
