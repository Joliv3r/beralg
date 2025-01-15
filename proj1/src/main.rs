pub mod algebraic_structure;
use rug::ops::Pow;
use rug::Integer;
use std::sync::Arc;
use std::time::Instant;
use crate::algebraic_structure::{Element, finite_field::FiniteField};

pub fn pow_rug(a: &Integer, b: &Integer, n: &Integer) -> Integer {
    let mut product: Integer = Integer::ONE.clone();
    let mut base = a.clone() % n;
    let mut exponent = b.clone();

    while exponent != 0 {
        if exponent.get_bit(0) {
            product = (product * &base).modulo(n);
        }
        base = base.square().modulo(n);
        exponent = exponent >> 1;
    }
    product
}


fn check_timing() {
    let p: Integer = Integer::from(2).pow(127) - 1;
    let a = Integer::from(3);
    let b = Integer::from(10).pow(100);
    
    let f = Arc::new(FiniteField::new(p.clone()).unwrap());
    let a_elem = Element::new(f.clone(), a.clone());
    let b_exp = b.clone();

    let now = Instant::now();
    let c_elem = a_elem.pow(b_exp);
    let elapsed = now.elapsed();
    println!("{} mod {}  in {} µs", c_elem.get_rep(), p, elapsed.as_micros());

    let now = Instant::now();
    let c = pow_rug(&a, &b, &p);
    let elapsed = now.elapsed();
    println!("{} mod {}  in {} µs", c, p, elapsed.as_micros());

    let now = Instant::now();
    let c_in = a.pow_mod(&b, &p).unwrap();
    let elapsed = now.elapsed();
    println!("{} mod {}  in {} µs", c_in, p, elapsed.as_micros());
}

fn main() {
    check_timing();
}
