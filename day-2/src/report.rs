use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Report(pub Vec<u8>);

#[derive(Debug, PartialEq)]
pub enum ReportStatus {
    Safe,
    Unsafe,
}

pub fn evaluate_status(report: &Report) -> ReportStatus {
    let levels = &report.0;
    let is_increasing = levels.windows(2).all(|window| window[0] < window[1]);
    let is_decreasing = levels.windows(2).all(|window| window[0] > window[1]);
    let is_within_1_and_3_change = levels.windows(2).all(|window| {
        let diff = window[0].abs_diff(window[1]);
        diff >= 1 && diff <= 3
    });

    if is_within_1_and_3_change && (is_increasing || is_decreasing) {
        return ReportStatus::Safe;
    }
    ReportStatus::Unsafe
}

pub fn parse_input() -> Result<Vec<Report>> {
    let mut reports: Vec<Report> = vec![];

    let file = File::open(Path::new("./day-2/inputs/input.txt"))?;
    for line in io::BufReader::new(file).lines().flatten() {
        let preprocessed_line = "[".to_owned() + &line.replace(" ", ",") + "]";
        let deserialized_report: Report = serde_json::from_str(&preprocessed_line).unwrap();
        reports.push(deserialized_report);
    }
    Ok(reports)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_decreasing_windows_by_1_or_2_then_return_safe() {
        assert_eq!(
            evaluate_status(&Report(vec![7, 6, 4, 2, 1])),
            ReportStatus::Safe
        );
    }

    #[test]
    fn test_when_increasing_windows_by_1_or_2_or_3_then_return_safe() {
        assert_eq!(
            evaluate_status(&Report(vec![1, 3, 6, 7, 9])),
            ReportStatus::Safe
        );
    }

    #[test]
    fn test_when_increase_of_5_then_return_unsafe() {
        assert_eq!(
            evaluate_status(&Report(vec![1, 2, 7, 8, 9])),
            ReportStatus::Unsafe
        );
    }

    #[test]
    fn test_when_decrease_of_4_then_return_unsafe() {
        assert_eq!(
            evaluate_status(&Report(vec![9, 7, 6, 2, 1])),
            ReportStatus::Unsafe
        );
    }

    #[test]
    fn test_when_mix_increase_and_decrease_then_return_unsafe() {
        assert_eq!(
            evaluate_status(&Report(vec![1, 3, 2, 4, 5])),
            ReportStatus::Unsafe
        );
    }

    #[test]
    fn test_when_contains_equal_neighbours_then_return_unsafe() {
        assert_eq!(
            evaluate_status(&Report(vec![8, 6, 4, 4, 1])),
            ReportStatus::Unsafe
        );
    }
}
