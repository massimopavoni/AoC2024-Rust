use atoi::atoi;
use regex::bytes::{Captures, Match, Regex};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn multiplications_sum(input: &str) -> u64 {
    // Just find all mul(x,y) and sum multiplications
    regex_captures_fold(input, r"mul\((\d{1,3}),(\d{1,3})\)", 0, |acc, capture| {
        acc + match_to_u64(capture.get(1)) * match_to_u64(capture.get(2))
    })
}

pub fn do_dont_multiplications_sum(input: &str) -> u64 {
    // Find all mul(x,y), do() and don't() and sum multiplications if doing is active
    regex_captures_fold(
        input,
        r"mul\((\d{1,3}),(\d{1,3})\)|(do(?:n't)?)\(\)",
        (0, true),
        |(acc, doing), capture| match capture.get(3).map(|m| m.as_bytes()) {
            None => (
                if doing {
                    acc + match_to_u64(capture.get(1)) * match_to_u64(capture.get(2))
                } else {
                    acc
                },
                doing,
            ),
            Some(b"do") => (acc, true),
            Some(b"don't") => (acc, false),
            _ => unreachable!(),
        },
    )
    .0
}

// ------------------------------------------------------------------------------------------------
// Functions

fn match_to_u64(match_: Option<Match<'_>>) -> u64 {
    atoi::<u64>(match_.expect("Expected capture").as_bytes()).expect("Invalid integer")
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn regex_captures_fold<I, Func>(input: &str, regex: &str, init: I, function: Func) -> I
where
    Func: FnMut(I, Captures<'_>) -> I,
{
    // Fold over captures applying the function starting with init
    Regex::new(regex)
        .expect("Invalid regex")
        .captures_iter(input.as_bytes())
        .fold(init, function)
}
