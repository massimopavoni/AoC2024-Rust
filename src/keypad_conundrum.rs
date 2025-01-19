use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::iter::once;

use crate::random_utils::{parse_expect, pos::Pos, FxHashWithCapacity};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn codes_complexity_3_robots(input: &str) -> usize {
    // Calculate door codes complexity 3 robots deep
    codes_complexity::<3>(input)
}

pub fn codes_complexity_26_robots(input: &str) -> usize {
    // Calculate door codes complexity 26 robots deep
    codes_complexity::<26>(input)
}

// ------------------------------------------------------------------------------------------------
// Functions

#[allow(clippy::too_many_lines)]
fn codes_complexity<const ROBOTS_COUNT: usize>(input: &str) -> usize {
    // Normal direction priority
    const DIR_PRIORITIES: &str = "<v^>";
    // Direction priority when connecting between first column and last row
    const GAP_DIR_PRIORITIES: &str = "^>v<";

    fn key_position(key: char) -> Pos {
        Pos::from(match key {
            '1' => (2, 0_usize),
            '2' => (2, 1),
            '3' => (2, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '0' => (3, 1),
            'A' => (3, 2),
            _ => unreachable!("Invalid number"),
        })
    }

    // Best paths between any two directions (ignoring inefficient left-right/up-down movements)
    fn best_directional_path<'a>(start: char, end: char) -> &'a str {
        match (start, end) {
            ('v', 'v') | ('>', '>') | ('^', '^') | ('<', '<') | ('A', 'A') => "A",
            ('v', '>') | ('^', 'A') | ('<', 'v') => ">A",
            ('v', '<') | ('>', 'v') | ('A', '^') => "<A",
            ('v', 'A') => "^>A",
            ('>', '^') => "<^A",
            ('>', 'A') => "^A",
            ('^', '>') => "v>A",
            ('^', '<') => "v<A",
            ('<', '^') => ">^A",
            ('<', 'A') => ">>^A",
            ('A', 'v') => "<vA",
            ('A', '>') => "vA",
            ('A', '<') => "v<<A",
            _ => unreachable!("Invalid start/end"),
        }
    }

    // Recursive memoized path cost function
    fn path_cost<'a>(
        path: &'a str,
        robots_left: usize,
        cache: &mut FxHashMap<(&'a str, usize), usize>,
    ) -> usize {
        if let Some(&cost) = cache.get(&(path, robots_left)) {
            cost
        } else {
            let cost = once('A')
                .chain(path.chars())
                .tuple_windows()
                .map(|(start, end)| {
                    let path = if start == end {
                        "A"
                    } else {
                        best_directional_path(start, end)
                    };

                    if robots_left == 1 {
                        path.len()
                    } else {
                        path_cost(path, robots_left - 1, cache)
                    }
                })
                .sum();

            cache.insert((path, robots_left), cost);
            cost
        }
    }

    // Door codes complexity calculation
    input
        .lines()
        .map(|code| {
            parse_expect::<usize>(&code[0..3])
                * path_cost(
                    // Initial door code best path search
                    &once('A')
                        .chain(code.chars())
                        .tuple_windows()
                        .map(|(start, end)| {
                            if start == end {
                                return "A".to_string();
                            };

                            let priorities = if "0A".contains(start) && "147".contains(end)
                                || "147".contains(start) && "0A".contains(end)
                            {
                                GAP_DIR_PRIORITIES
                            } else {
                                DIR_PRIORITIES
                            };

                            let (start, end) = (key_position(start), key_position(end));
                            let dpos = end - start;
                            let mut movement = String::new();

                            for key in priorities.chars() {
                                #[allow(clippy::cast_sign_loss)]
                                match key {
                                    '<' if dpos.y < 0 => movement += &"<".repeat(-dpos.y as usize),
                                    '>' if dpos.y > 0 => movement += &">".repeat(dpos.y as usize),
                                    '^' if dpos.x < 0 => movement += &"^".repeat(-dpos.x as usize),
                                    'v' if dpos.x > 0 => movement += &"v".repeat(dpos.x as usize),
                                    _ => {}
                                }
                            }

                            movement + "A"
                        })
                        .collect::<String>(),
                    ROBOTS_COUNT - 1,
                    &mut FxHashMap::with_capacity(ROBOTS_COUNT * ROBOTS_COUNT / 2),
                )
        })
        .sum()
}
