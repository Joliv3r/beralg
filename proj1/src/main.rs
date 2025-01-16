use primes::generate_primes;

pub mod algebraic_structure;
pub mod tests;
pub mod integer_computations;
pub mod primes;


fn main() {
    // check_timing_against_rug();
    // check_timing_against_naive();
    generate_primes(1000000);
}
