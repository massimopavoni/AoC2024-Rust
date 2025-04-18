use std::{array::from_fn, convert::identity, sync::Arc};

use pathfinding::directed::count_paths::count_paths;
use rayon::{iter::ParallelIterator, str::ParallelString};
use rustc_hash::FxHashSet;

use crate::random_utils::FxHashWithCapacity;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn possible_designs_count(input: &str) -> usize {
    // Find possible designs by checking if at least one path generates the design
    possible_designs_function(input, |count| count.clamp(0, 1))
}

pub fn possible_designs_possible_ways_count(input: &str) -> usize {
    // Count all possible designs possible paths
    possible_designs_function(input, identity)
}

// ------------------------------------------------------------------------------------------------
// Functions

fn possible_designs_function<Trans>(input: &str, solution_transform: Trans) -> usize
where
    Trans: Fn(usize) -> usize + Sync,
{
    // Convert color byte to color
    fn byte_to_color(b: u8) -> usize {
        match b {
            b'w' => 0,
            b'b' => 1,
            b'g' => 2,
            b'r' => 3,
            b'u' => 4,
            _ => unreachable!("Invalid color byte"),
        }
    }

    let (patterns_str, designs_str) = input.split_once("\n\n").expect("Expected two sections");

    let mut patterns: [FxHashSet<&str>; 5] = from_fn(|_| FxHashSet::with_capacity(128));
    let mut longest_patterns: [usize; 5] = [0; 5];

    // Store patterns by first color in hash sets
    for pattern in patterns_str.split(", ") {
        let first_color = byte_to_color(pattern.as_bytes()[0]);

        if pattern.len() > longest_patterns[first_color] {
            longest_patterns[first_color] = pattern.len();
        }

        patterns[first_color].insert(pattern);
    }

    let patterns = Arc::new(patterns);

    // Count paths for each design
    designs_str
        .par_lines()
        .map(|design| {
            let patterns = Arc::clone(&patterns);

            solution_transform(count_paths(
                0,
                move |&index| {
                    let mut successors = Vec::with_capacity(4);
                    let first_color = byte_to_color(design.as_bytes()[index]);

                    for pattern_size in 1..=longest_patterns[first_color] {
                        if index + pattern_size > design.len() {
                            break;
                        }

                        let sub_design = &design[index..index + pattern_size];

                        if patterns[first_color].contains(sub_design) {
                            successors.push(index + pattern_size);
                        }
                    }

                    successors
                },
                |&index| index == design.len(),
            ))
        })
        .sum()
}
