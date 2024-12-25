use pathfinding::directed::count_paths::count_paths;
use regex::bytes::RegexBuilder;
use rustc_hash::FxHashSet;
use std::array::from_fn;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn possible_designs_count(input: &str) -> usize {
    let (patterns, designs) = input.split_once("\n\n").expect("Expected two sections");

    // Very simply build regex and check how many designs match
    RegexBuilder::new(&format!("^({})+$", patterns.replace(", ", "|")))
        .multi_line(true)
        .build()
        .expect("Expected valid regex")
        .captures_iter(designs.as_bytes())
        .count()
}

pub fn possible_designs_possible_ways_count(input: &str) -> usize {
    // Convert color byte to color
    let byte_to_color = |b| {
        b"wbgru"
            .iter()
            .position(|&c| c == b)
            .expect("Expected valid color")
    };

    let (patterns_str, designs_str) = input.split_once("\n\n").expect("Expected two sections");

    let mut patterns: [FxHashSet<&str>; 5] = from_fn(|_| FxHashSet::default());
    let mut longest_patterns: [usize; 5] = [0; 5];

    // Store patterns by first color in hash sets
    for pattern in patterns_str.split(", ") {
        let first_color = byte_to_color(pattern.as_bytes()[0]);

        if pattern.len() > longest_patterns[first_color] {
            longest_patterns[first_color] = pattern.len();
        }

        patterns[first_color].insert(pattern);
    }

    // Count paths for each design
    designs_str
        .lines()
        .map(|design| {
            count_paths(
                0,
                |&index| {
                    let mut successors = vec![];
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
            )
        })
        .sum()
}
