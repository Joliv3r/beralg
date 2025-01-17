// This is in addition to computational algebra a personal learning experience with rust.

pub mod algebraic_structure;
pub mod tests;
pub mod integer_computations;


fn main() {
    let n: u32 = 100;
    tests::check_timings(n).expect("Should not fail");
}
