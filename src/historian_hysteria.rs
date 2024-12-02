use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn lists_total_distance(input: &str) -> u64 {
    let (list1, list2) = lists(input);

    // Sort both lists, zip them and sum the differences
    list1
        .into_iter()
        .sorted()
        .zip(list2.into_iter().sorted())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

pub fn lists_similarity_score(input: &str) -> u64 {
    let (list1, list2) = lists(input);

    // Multiply each element of list1 by the number of times it appears in list2
    let mut list2_counts = list2.into_iter().counts();

    list1
        .into_iter()
        .map(|a| a * list2_counts.remove(&a).unwrap_or_default() as u64)
        .sum()
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn lists(input: &str) -> (Vec<u64>, Vec<u64>) {
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
