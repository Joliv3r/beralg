use std::{fs::OpenOptions, io::Write};
use rug::Integer;

pub fn generate_primes(n: u32) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("primes")
        .unwrap();
    let mut b = Integer::from(2);
    for _ in 0..n {
        let string = b.to_string();
        writeln!(file, "{}", string).unwrap();
        b.next_prime_mut();
    }
}
