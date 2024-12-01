use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn lists_total_distance(input: &str) -> u64 {
    let (list1, list2) = lists(input);

    // Sort both lists, zip them and sum the differences
    list1
        .iter()
        .sorted()
        .zip(list2.iter().sorted())
        .map(|(a, &b)| a.abs_diff(b))
        .sum()
}

pub fn lists_similarity_score(input: &str) -> u64 {
    let (list1, list2) = lists(input);

    // Multiply each element of list1 by the number of times it appears in list2
    list1
        .iter()
        .map(|&a| a * list2.iter().filter(|&&b| a == b).count() as u64)
        .sum()
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    // Split lines and unzip into 2 vectors
    input
        .lines()
        .map(|line| {
            let line = line.split_once("   ").unwrap();
            (
                line.0.parse::<u64>().unwrap(),
                line.1.parse::<u64>().unwrap(),
            )
        })
        .unzip()
}
