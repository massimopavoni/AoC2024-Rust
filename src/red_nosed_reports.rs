use itertools::Itertools;

use crate::random_utils::parse_expect;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn safe_reports_count(input: &str) -> u64 {
    // Simply count safe reports
    filter_count_reports(input, |report| is_safe_report(report.iter().collect_vec()))
}

pub fn problem_dampener_safe_reports_count(input: &str) -> u64 {
    // Count reports which have a safe subset
    filter_count_reports(input, |report| {
        report
            .iter()
            .combinations(report.len() - 1)
            .any(is_safe_report)
    })
}

// ------------------------------------------------------------------------------------------------
// Functions

fn filter_count_reports<Filter>(input: &str, filter: Filter) -> u64
where
    Filter: Fn(&Vec<u64>) -> bool,
{
    // Parse, filter, count
    report_list(input).into_iter().filter(filter).count() as u64
}

fn is_safe_report(report: Vec<&u64>) -> bool {
    let (mut asc, mut desc) = (true, true);

    // A safe report is sorted and has differences of 1, 2 or 3
    report.into_iter().tuple_windows().all(|(&a, &b)| {
        let diff = a.abs_diff(b);

        if (1..4).contains(&diff) {
            asc &= a < b;
            desc &= a > b;
            asc || desc
        } else {
            false
        }
    })
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn report_list(input: &str) -> Vec<Vec<u64>> {
    // Split lines and get number vectors
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(parse_expect).collect())
        .collect()
}
