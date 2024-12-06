use std::{any::type_name, fmt::Debug, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Functions

pub fn parse_expect<N>(string: &str) -> N
where
    N: FromStr,
    <N as FromStr>::Err: Debug,
{
    string
        .parse::<N>()
        .unwrap_or_else(|_| panic!("Expected {}", type_name::<N>()))
}
