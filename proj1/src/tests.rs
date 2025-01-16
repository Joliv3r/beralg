use rug::ops::Pow;
use rug::Integer;
use std::sync::Arc;
use std::time::Instant;
use crate::algebraic_structure::{Element, finite_field::FiniteField};


pub fn check_timing() {
    let p: Integer = Integer::from(2).pow(127) - 1;
    let a = Integer::from(3);
    let b = Integer::from(10).pow(10000);
    
    let f = Arc::new(FiniteField::new(p.clone()).unwrap());
    let a_elem = Element::new(f.clone(), a.clone());
    let b_exp = b.clone();

    let now = Instant::now();
    a_elem.pow(&b_exp);
    let elapsed = now.elapsed();
    println!("Self-implentation in {} µs", elapsed.as_micros());

    let now = Instant::now();
    a.pow_mod(&b, &p).unwrap();
    let elapsed = now.elapsed();
    println!("rug::Integer::pow_mod in {} µs", elapsed.as_micros());
}
