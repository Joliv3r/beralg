pub mod algebraic_structure;
pub mod tests;
pub mod integer_computations;

use tests::*;

fn main() {
    check_timing_against_rug();
    check_timing_against_naive();
}
