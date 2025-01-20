// This is in addition to computational algebra a personal learning experience with rust.

pub mod algebraic_structure;
pub mod tests;
pub mod integer_computations;


fn main() {
    let n: u32 = 500;
    let m: u32 = 75;
    tests::check_timings_sep(n).expect("Should not fail");
    tests::check_timings(m).expect("Should not fail");
}
