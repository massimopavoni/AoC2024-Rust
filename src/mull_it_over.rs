use atoi::{atoi, FromRadix10SignedChecked};
use regex::bytes::Match;
use regex::bytes::{Captures, Regex};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn multiplications_sum(input: &str) -> u64 {
    // Just find all mul(x,y) and sum multiplications
    regex_captures_fold(input, r"mul\((\d{1,3}),(\d{1,3})\)", 0, |sum, captures| {
        sum + re_match_atoi::<u64>(captures.get(1)) * re_match_atoi::<u64>(captures.get(2))
    })
}

pub fn do_dont_multiplications_sum(input: &str) -> u64 {
    // Find all mul(x,y), do() and don't() and sum multiplications if doing is active
    regex_captures_fold(
        input,
        r"mul\((\d{1,3}),(\d{1,3})\)|(do(?:n't)?)\(\)",
        (0, true),
        |(sum, doing), captures| match captures.get(3).map(|m| m.as_bytes()) {
            None => (
                if doing {
                    sum + re_match_atoi::<u64>(captures.get(1))
                        * re_match_atoi::<u64>(captures.get(2))
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
pub fn re_match_atoi<N>(match_: Option<Match<'_>>) -> N
where
    N: FromRadix10SignedChecked,
{
    atoi::<N>(match_.expect("Expected capture").as_bytes()).expect("Expected valid integer")
}
