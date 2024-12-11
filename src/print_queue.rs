use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

use crate::random_utils::parse_expect;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn valid_updates_middle_sum(input: &str) -> u64 {
    let (rules, updates) = page_rules_and_updates(input);

    // Process n choose 2 combinations of elements to find valid updates
    updates
        .into_iter()
        .filter(|update| {
            update
                .iter()
                .tuple_combinations()
                .all(|(&a, &b)| rules.contains(&(a, b)))
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn fixed_invalid_updates_middle_sum(input: &str) -> u64 {
    let (rules, updates) = page_rules_and_updates(input);

    // Find invalid items and sort them thanks to precedence rules
    updates
        .into_iter()
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

fn page_rules_and_updates(input: &str) -> (HashSet<(u64, u64)>, Vec<Vec<u64>>) {
    // Split input into rules and updates
    let (rules, updates) = input.split_once("\n\n").expect("Expected two sections");

    (
        rules
            .lines()
            .map(|line| {
                let line = line.split_once('|').expect("Expected two numbers");

                (parse_expect(line.0), parse_expect(line.1))
            })
            .collect(),
        updates
            .lines()
            .map(|line| line.split(',').map(parse_expect).collect())
            .collect(),
    )
}
