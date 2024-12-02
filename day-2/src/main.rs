mod first_puzzle;
mod report;

pub use crate::first_puzzle::first_puzzle;
pub use crate::report::parse_input;

fn main() {
    let reports = parse_input().unwrap();

    first_puzzle(&reports);
}
