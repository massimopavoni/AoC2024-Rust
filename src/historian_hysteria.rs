use std::{convert::identity, vec::IntoIter};

use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn lists_total_distance(input: &str) -> u64 {
    let (list1, list2) = location_lists(input);

    // Sort both lists, zip them and sum the differences
    iter_map(
        list1,
        |i| i.sorted_unstable().zip(list2.into_iter().sorted_unstable()),
        |(a, b)| a.abs_diff(b),
    )
}

pub fn lists_similarity_score(input: &str) -> u64 {
    let (list1, list2) = location_lists(input);

    // Multiply each element of list1 by the number of times it appears in list2
    let mut list2_counts = list2.into_iter().counts();

    iter_map(list1, identity, |a| {
        a * list2_counts.remove(&a).unwrap_or_default() as u64
    })
}

// ------------------------------------------------------------------------------------------------
// Functions

fn iter_map<T, Trans, Iter, Map>(list1: Vec<u64>, transform: Trans, map: Map) -> u64
where
    Trans: FnOnce(IntoIter<u64>) -> Iter,
    Iter: Iterator<Item = T>,
    Map: FnMut(T) -> u64,
{
    // Transform the list1 iterator, map over it and sum results
    transform(list1.into_iter()).map(map).sum()
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn location_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    // Split lines and unzip into 2 vectors
    input
        .lines()
        .map(|line| {
            let line = line
                .split_once("   ")
                .expect("Expected 2 numbers separated by 3 spaces");
            (
                line.0.parse::<u64>().expect("Expected an unsigned integer"),
                line.1.parse::<u64>().expect("Expected an unsigned integer"),
            )
        })
        .unzip()
}
