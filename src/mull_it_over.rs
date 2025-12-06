use regex::bytes::Match;
use regex::bytes::{Captures, Regex};

use crate::random_utils::parse_number_bytes;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn multiplications_sum(input: &str) -> u32 {
    // Just find all mul(x,y) and sum multiplications
    regex_captures_fold(input, r"mul\((\d{1,3}),(\d{1,3})\)", 0, |sum, captures| {
        sum + regex_match_parse(captures.get(1)) * regex_match_parse(captures.get(2))
    })
}

pub fn do_dont_multiplications_sum(input: &str) -> u32 {
    // Find all mul(x,y), do() and don't() and sum multiplications if doing is active
    regex_captures_fold(
        input,
        r"mul\((\d{1,3}),(\d{1,3})\)|(do(?:n't)?)\(\)",
        (0, true),
        |(sum, doing), captures| match captures.get(3).map(|m| m.as_bytes()) {
            None => (
                if doing {
                    sum + regex_match_parse(captures.get(1)) * regex_match_parse(captures.get(2))
                } else {
                    sum
                },
                doing,
            ),
            Some(b"do") => (sum, true),
            Some(b"don't") => (sum, false),
            _ => unreachable!("Invalid regex capture"),
        },
    )
    .0
}

// ------------------------------------------------------------------------------------------------
// Functions

fn regex_captures_fold<I, Fold>(input: &str, regex: &str, init: I, fold: Fold) -> I
where
    Fold: FnMut(I, Captures<'_>) -> I,
{
    // Fold over captures applying the function starting with init
    Regex::new(regex)
        .expect("Invalid regex")
        .captures_iter(input.as_bytes())
        .fold(init, fold)
}

// ------------------------------------------------------------------------------------------------
// Parsers

#[inline]
pub fn regex_match_parse(match_: Option<Match<'_>>) -> u32 {
    parse_number_bytes(match_.expect("Expected caputure").as_bytes())
}
