pub mod algebraic_structure;
use number_theory::{Mpz, NumberTheory};
use rug::ops::Pow;
use rug::Integer;
use std::{sync::Arc, time::Duration};
use std::time::Instant;
use std::thread::sleep;
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
    return product
}


pub fn pow(a: &Mpz, b: &Mpz, n: &Mpz) -> Mpz {
    let mut product = Mpz::one();
    let mut base = a.clone();
    let mut exponent = b.clone();
    let zero = Mpz::zero();

    loop {
        if exponent.check_bit(0) {
            product = product.product_residue(&base, n);
        }
        if exponent.is_zero() {
            break;
        }
        base = base.product_residue(&base, n);
        exponent.mut_shr(1);
    }
    product
}

fn main() {
    let p_mpz: Mpz = Mpz::from_i64(2).pow(127).ref_subtraction(&Mpz::one());
    let p: Integer = Integer::from(2).pow(127) - 1;
    // let p: Mpz = Mpz::from_u64(101);
    // let f = Arc::new(FiniteField::new(p.clone()).unwrap());

    let a_mpz = Mpz::from_u64(3);
    let a = Integer::from(3);
    let b_mpz = Mpz::from_u64(10).pow(500);
    let b = Integer::from(10).pow(500);

    // let a_elem = Element::new(f.clone(), a.clone());

    let now = Instant::now();
    // for _ in 0..10 {
    //     a_elem.pow(b.clone());
    // }
    let c = pow_rug(&a, &b, &p);
    let elapsed = now.elapsed();

    let now = Instant::now();
    let c_mpz = pow(&a_mpz, &b_mpz, &p_mpz);
    let elapsed_mpz = now.elapsed();

    println!("{} mod {}  in {} µs", c, p, elapsed.as_micros());
    println!("{} mod {}  in {} µs", c_mpz, p_mpz, elapsed_mpz.as_micros());

    // let mut a: Integer = Integer::from(2);
    // for _ in 1..500 {
    //     println!("{}", a);
    //     a.next_prime_mut();
    // }

    // let b = Mpz::from_u64(10).pow(10000);
    // let now = Instant::now();
    // for _ in 0..10 {
    //     a.exp_residue(&b.clone(), &p);
    // }
    // a.exp_residue(&b, &p);
    // let elapsed = now.elapsed();
    //
    // println!("{} ms", elapsed.as_millis());
}
