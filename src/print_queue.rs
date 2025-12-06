use std::cmp::Ordering;

use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::random_utils::parse_numbers;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn valid_updates_middle_sum(input: &str) -> u16 {
    let (rules, updates) = page_rules_and_updates(input);

    // Process n choose 2 combinations of elements to find valid updates
    updates
        .filter(|update| {
            update
                .iter()
                .tuple_combinations()
                .all(|(&a, &b)| rules.contains(&(a, b)))
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn fixed_invalid_updates_middle_sum(input: &str) -> u16 {
    let (rules, updates) = page_rules_and_updates(input);

    // Find invalid items and sort them thanks to precedence rules
    updates
        .filter(|update| {
            update
                .iter()
                .tuple_combinations()
                .any(|(&a, &b)| !rules.contains(&(a, b)))
        })
        .map(|mut update| {
            update.sort_unstable_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            update[update.len() / 2]
        })
        .sum()
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn page_rules_and_updates(
    input: &str,
) -> (FxHashSet<(u16, u16)>, impl Iterator<Item = Vec<u16>> + '_) {
    // Split input into rules and updates
    let (rules, updates) = input.split_once("\n\n").expect("Expected two sections");

    (
        parse_numbers(rules).tuples().collect(),
        updates
            .lines()
            .map(|line| parse_numbers(line).collect_vec()),
    )
}
