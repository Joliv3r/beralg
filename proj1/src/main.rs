pub mod algebraic_structure;
use number_theory::{Mpz, NumberTheory};
use std::{sync::Arc, time::Duration};
use std::time::Instant;
use std::thread::sleep;
use crate::algebraic_structure::{Element, finite_field::FiniteField};

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
    let p: Mpz = Mpz::from_i64(2).pow(127).ref_subtraction(&Mpz::one());
    // let p: Mpz = Mpz::from_u64(101);
    let f = Arc::new(FiniteField::new(p.clone()).unwrap());

    let a = Mpz::from_u64(3);
    let b = Mpz::from_u64(10).pow(10000);

    let a_elem = Element::new(f.clone(), a.clone());

    let now = Instant::now();
    // for _ in 0..10 {
    //     a_elem.pow(b.clone());
    // }
    let c = a_elem.pow(b);
    let elapsed = now.elapsed();

    println!("{} mod {}  in {} ms", c.get_rep(), p, elapsed.as_millis());

    let b = Mpz::from_u64(10).pow(10000);
    let now = Instant::now();
    // for _ in 0..10 {
    //     a.exp_residue(&b.clone(), &p);
    // }
    a.exp_residue(&b, &p);
    let elapsed = now.elapsed();

    println!("{} ms", elapsed.as_millis());

    // let mut p: Mpz = Mpz::one();
    // p.mut_shr(1);
    // println!("{}", p);
    // p.check_bit(0);

    // let b = b.ref_subtraction(&b);

    // let limbs = b.len();

    // loop {
    //     b.mut_shr(1);
    //     sleep(Duration::from_millis(100));
    //     let limbs = b.len();
    //     println!("{:#?}", b);
    //     println!("{}", limbs);
    //     println!("{}", b.is_zero());
    // }

    // println!("{:#?}", b);
    // println!("{:#?}", limbs);
}
