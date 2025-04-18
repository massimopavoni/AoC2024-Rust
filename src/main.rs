use std::{fmt::Display, sync::LazyLock};

use include_dir::{Dir, include_dir};
use itertools::Itertools;
use rustc_hash::FxHashMap;

mod random_utils;

mod bridge_repair;
mod ceres_search;
mod chronospatial_computer;
mod claw_contraption;
mod code_chronicle;
mod crossed_wires;
mod disk_fragmenter;
mod garden_groups;
mod guard_gallivant;
mod historian_hysteria;
mod hoof_it;
mod keypad_conundrum;
mod lan_party;
mod linen_layout;
mod monkey_market;
mod mull_it_over;
mod plutonian_pebbles;
mod print_queue;
mod race_condition;
mod ram_run;
mod red_nosed_reports;
mod reindeer_maze;
mod resonant_collinearity;
mod restroom_redoubt;
mod warehouse_woes;

use bridge_repair::{total_calibration_plus_times, total_calibration_plus_times_concat};
use ceres_search::{x_mas_occurrences_count, xmas_occurrences_count};
use chronospatial_computer::{program_output, program_quine_register_value};
use claw_contraption::{fewest_tokens_all_prizes_huge, fewest_tokens_all_prizes_small};
use code_chronicle::unique_key_lock_pairs_count;
use crossed_wires::{final_z_wires_value, ripple_carry_adder_swapped_wires};
use disk_fragmenter::{compact_disk_checksum, whole_files_compact_disk_checksum};
use garden_groups::{fences_total_cost_perimeter, fences_total_cost_sides};
use guard_gallivant::{possible_obstruction_loops_count, unique_guard_positions_count};
use historian_hysteria::{lists_similarity_score, lists_total_distance};
use hoof_it::{trailheads_total_rating, trailheads_total_score};
use keypad_conundrum::{codes_complexity_3_robots, codes_complexity_26_robots};
use lan_party::{graph_triangles_count, maximum_clique_password};
use linen_layout::{possible_designs_count, possible_designs_possible_ways_count};
use monkey_market::{best_selling_sequence_bananas_count, buyers_2000th_secret_numbers_sum};
use mull_it_over::{do_dont_multiplications_sum, multiplications_sum};
use plutonian_pebbles::{stones_expansion_25_blinks, stones_expansion_75_blinks};
use print_queue::{fixed_invalid_updates_middle_sum, valid_updates_middle_sum};
use race_condition::{best_2_picos_cheat_paths_count, best_20_picos_cheat_paths_count};
use ram_run::{first_path_cutoff_byte, minimum_steps_exit_kilobyte};
use red_nosed_reports::{problem_dampener_safe_reports_count, safe_reports_count};
use reindeer_maze::{maze_best_path_score, maze_best_seats_count};
use resonant_collinearity::{unique_antinodes_count, unique_resonant_harmonics_antinode_count};
use restroom_redoubt::{robots_christmas_tree, robots_safety_factor};
use warehouse_woes::{final_thin_boxes_coordinates_sum, final_wide_boxes_coordinates_sum};

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

static PUZZLE_ANSWERS: LazyLock<FxHashMap<&str, [&str; 2]>> = LazyLock::new(|| {
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

#[allow(clippy::too_many_lines)]
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

    pretty_solution_2!(
        15,
        "WarehouseWoes",
        final_thin_boxes_coordinates_sum,
        final_wide_boxes_coordinates_sum
    );

    pretty_solution_2!(
        16,
        "ReindeerMaze",
        maze_best_path_score,
        maze_best_seats_count
    );

    pretty_solution_2!(
        17,
        "ChronospatialComputer",
        program_output,
        program_quine_register_value
    );

    pretty_solution_2!(
        18,
        "RAMRun",
        minimum_steps_exit_kilobyte,
        first_path_cutoff_byte
    );

    pretty_solution_2!(
        19,
        "LinenLayout",
        possible_designs_count,
        possible_designs_possible_ways_count
    );

    pretty_solution_2!(
        20,
        "RaceCondition",
        best_2_picos_cheat_paths_count,
        best_20_picos_cheat_paths_count
    );

    pretty_solution_2!(
        21,
        "KeypadConundrum",
        codes_complexity_3_robots,
        codes_complexity_26_robots
    );

    pretty_solution_2!(
        22,
        "MonkeyMarket",
        buyers_2000th_secret_numbers_sum,
        best_selling_sequence_bananas_count
    );

    pretty_solution_2!(
        23,
        "LANParty",
        graph_triangles_count,
        maximum_clique_password
    );

    pretty_solution_2!(
        24,
        "CrossedWires",
        final_z_wires_value,
        ripple_carry_adder_swapped_wires
    );

    pretty_solution_2!(25, "CodeChronicle", unique_key_lock_pairs_count);
}
