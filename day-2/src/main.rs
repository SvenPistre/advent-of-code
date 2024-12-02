mod first_puzzle;
mod report;
mod second_puzzle;

pub use crate::first_puzzle::first_puzzle;
pub use crate::report::parse_input;
pub use crate::second_puzzle::second_puzzle;

fn main() {
    let reports = parse_input().unwrap();

    first_puzzle(&reports);
    second_puzzle(&reports);
}
