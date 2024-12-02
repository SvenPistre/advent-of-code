pub use crate::report::{evaluate_status, Report, ReportStatus};

pub fn second_puzzle(reports: &Vec<Report>) -> () {
    let number_of_safe_reports = reports.iter().fold(0, |acc, report| {
        acc + match dampen_report_status_by_one_level(report) {
            ReportStatus::Safe => 1,
            ReportStatus::Unsafe => 0,
        }
    });
    println!(
        "The number of dampened safe reports is {}",
        number_of_safe_reports
    )
}

fn dampen_report_status_by_one_level(report: &Report) -> ReportStatus {
    if let ReportStatus::Safe = evaluate_status(report) {
        return ReportStatus::Safe;
    }

    let levels = &report.0;
    let subreports = (0..levels.len())
        .map(|mid| levels.split_at(mid))
        .map(|(left, right)| Report([left, &right[1..]].concat()))
        .collect::<Vec<_>>();
    let number_of_safe_subreports = subreports
        .iter()
        .filter(|subreport| evaluate_status(subreport) == ReportStatus::Safe)
        .collect::<Vec<_>>()
        .len();

    if number_of_safe_subreports >= 1 {
        return ReportStatus::Safe;
    }

    ReportStatus::Unsafe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_decreasing_windows_by_1_or_2_or_3_then_return_safe() {
        assert_eq!(
            dampen_report_status_by_one_level(&Report(vec![7, 6, 4, 2, 1])),
            ReportStatus::Safe
        );
    }

    #[test]
    fn test_when_one_fault_present_otherwise_increasing_windows_by_1_or_2_or_3_then_return_safe() {
        assert_eq!(
            dampen_report_status_by_one_level(&Report(vec![1, 3, 2, 4, 5])),
            ReportStatus::Safe
        );
    }

    #[test]
    fn test_when_one_duplicate_number_otherwise_decreasing_windows_by_1_or_2_or_3_then_return_safe()
    {
        assert_eq!(
            dampen_report_status_by_one_level(&Report(vec![8, 6, 4, 4, 1])),
            ReportStatus::Safe
        );
    }

    #[test]
    fn test_when_increasing_windows_by_1_or_2_or_3_then_return_safe() {
        assert_eq!(
            dampen_report_status_by_one_level(&Report(vec![1, 3, 6, 7, 9])),
            ReportStatus::Safe
        );
    }

    #[test]
    fn test_when_removing_any_level_still_leaves_increase_of_4_then_return_unsafe() {
        assert_eq!(
            dampen_report_status_by_one_level(&Report(vec![1, 2, 7, 8, 9])),
            ReportStatus::Unsafe
        );
    }

    #[test]
    fn test_when_removing_any_level_still_leaves_decrease_of_4_then_return_unsafe() {
        assert_eq!(
            dampen_report_status_by_one_level(&Report(vec![9, 7, 6, 2, 1])),
            ReportStatus::Unsafe
        );
    }
}
