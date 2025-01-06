mod first_puzzle;
mod second_puzzle;

pub use crate::first_puzzle::first_puzzle;
pub use crate::second_puzzle::second_puzzle;

fn main() {
    first_puzzle();
    second_puzzle()
}
