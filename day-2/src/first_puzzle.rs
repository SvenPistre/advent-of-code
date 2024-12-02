pub use crate::report::{evaluate_status, Report, ReportStatus};

pub fn first_puzzle(reports: &Vec<Report>) -> () {
    let number_of_safe_reports = reports.iter().fold(0, |acc, report| {
        acc + match evaluate_status(report) {
            ReportStatus::Safe => 1,
            ReportStatus::Unsafe => 0,
        }
    });
    println!("The number of safe reports is {}", number_of_safe_reports)
}
