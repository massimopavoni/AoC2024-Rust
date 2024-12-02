use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn safe_reports_count(input: &str) -> u64 {
    let reports = report_list(input);

    // Filter and count safe reports
    reports
        .into_iter()
        .filter(|report| is_safe_report(report.iter().collect::<Vec<_>>()))
        .count() as u64
}

pub fn problem_dampener_safe_reports_count(input: &str) -> u64 {
    let reports = report_list(input);

    // Filter and count reports that can be considered safe after removing a single level
    reports
        .into_iter()
        .filter(|report| {
            report
                .iter()
                .combinations(report.len() - 1)
                .any(is_safe_report)
        })
        .count() as u64
}

// ------------------------------------------------------------------------------------------------
// Functions

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
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .expect("Expected some unsigned integers")
        })
        .collect()
}
