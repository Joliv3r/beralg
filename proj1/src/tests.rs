use rug::ops::Pow;
use std::sync::Arc;
use std::time::Instant;
use crate::{algebraic_structure::{finite_field::FiniteField, Element}, integer_computations::naive_pow};
use rug::Integer;


pub fn check_timing_against_rug() {
    let p: Integer = Integer::from(2).pow(127) - 1;
    let a = Integer::from(3);
    let exponent = 10000;
    let base = 10;
    let b = Integer::from(base).pow(exponent);
    
    let f = Arc::new(FiniteField::new(p.clone()).unwrap());
    let a_elem = Element::new(f.clone(), a.clone());
    let b_exp = b.clone();

    println!("Calculating: {}^{}^{}  mod {}", a, base, exponent, p);

    let now = Instant::now();
    a_elem.pow(&b_exp);
    let elapsed = now.elapsed();
    println!("Self-implentation in {} µs", elapsed.as_micros());

    let now = Instant::now();
    a.pow_mod(&b, &p).unwrap();
    let elapsed = now.elapsed();
    println!("rug::Integer::pow_mod in {} µs\n", elapsed.as_micros());
}


pub fn check_timing_against_naive() {
    let p: Integer = Integer::from(2).pow(127) - 1;
    let a = Integer::from(3);
    let exponent = 4;
    let base = 10;
    let b = Integer::from(base).pow(exponent);
    
    let f = Arc::new(FiniteField::new(p.clone()).unwrap());
    let a_elem = Element::new(f.clone(), a.clone());
    let b_exp = b.clone();

    println!("Calculating: {}^{}^{}  mod {}", a, base, exponent, p);

    let now = Instant::now();
    a_elem.pow(&b_exp);
    let elapsed = now.elapsed();
    println!("Self-implentation in {} µs", elapsed.as_micros());

    let now = Instant::now();
    naive_pow(&a, &b, &p);
    let elapsed = now.elapsed();
    println!("Naive implementation in {} µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic_structure::Element;
    use crate::algebraic_structure::finite_field::MultiplicativeGroup;
    use rug::{Complete, rand::RandState};
    use std::sync::Arc;

    #[test]
    fn test_algebraic_structure_arithmetic() {
        let n: u32 = 200;
        let mut prime: Integer = Integer::from(2);
        let mut rng = RandState::new();

        for _ in 2..n {
            let p = prime.clone();
            let f: Arc<FiniteField> = Arc::new(FiniteField::new(p.clone()).unwrap());
            let a_rand: Integer = Integer::from(rng.bits(32));
            let b_rand: Integer = Integer::from(rng.bits(32));
            let a = Element::new(f.clone(), a_rand.clone());
            let b = Element::new(f.clone(), b_rand.clone());

            assert_eq!(a.get_rep(), &(&a_rand % &p).complete());
            assert_eq!(b.get_rep(), &(&b_rand % &p).complete());

            let added = (a.add_ref(&b)).get_rep().clone();
            let multiplied = (a.mul_ref(&b)).get_rep().clone();

            let added_check: Integer = ((&a_rand % &p).complete() + (&b_rand % &p).complete()) % &p;
            let mul_check: Integer = ((a_rand % &p) * (b_rand % &p)) % &p;

            assert_eq!(added, added_check, "Failed for: {} + {}, and got {} instead of {}", a.get_rep(), b.get_rep(), added, added_check);
            assert_eq!(multiplied, mul_check, "Failed for: {} * {}, and got {} instead of {}", a.get_rep(), b.get_rep(), multiplied, mul_check);

            let g: Arc<MultiplicativeGroup> = Arc::new(MultiplicativeGroup::from_finite_field(&f));

            // To not create zero
            let a_rand: Integer = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1;
            let b_rand: Integer = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1;

            let a = Element::new(g.clone(), a_rand.clone());
            let b = Element::new(g.clone(), b_rand.clone());

            assert_eq!(a.get_rep(), &(&a_rand % &p).complete(), "Failed representation of {} in Z_{}*,  got {} instead of {}", &a_rand, &p, a.get_rep(), (&a_rand % &p).complete());
            assert_eq!(b.get_rep(), &(&b_rand % &p).complete(), "Failed representation of {} in Z_{}*,  got {} instead of {}", &b_rand, &p, b.get_rep(), (&b_rand % &p).complete());

            let multiplied = a.mul_ref(&b);
            let mul_check: Integer = ((a_rand % &p) * (b_rand % &p)) % &p;
            assert_eq!(multiplied.get_rep(), &mul_check);

            prime.next_prime_mut();
        }
    }

    #[test]
    fn test_extended_euclidean() {
        let n: u32 = 200;
        let mut prime: Integer = Integer::from(2);
        let mut rng = RandState::new();

        for _ in 2..n {
            let p = prime.clone();
            let f = Arc::new(FiniteField::new(p.clone()).unwrap());
            let a_rand = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1 ;
            let a = Element::new(f.clone(), a_rand);

            let a_inv = a.inv();
            assert_eq!(Integer::ONE, a_inv.mul_ref(&a).get_rep(), "We have a: {}, a_inv: {}, p: {}", a.get_rep(), a_inv.get_rep(), p);

            let g = Arc::new(MultiplicativeGroup::from_finite_field(&f));
            let a_rand = (Integer::from(rng.bits(32)).modulo(&(&p-&Integer::ONE.clone()).complete())) + 1 ;
            let a = Element::new(g.clone(), a_rand);
            let a_inv = a.inv();
            assert_eq!(Integer::ONE, a_inv.mul_ref(&a).get_rep(), "We have a: {}, a_inv: {}, p: {}", a.get_rep(), a_inv.get_rep(), p);


            prime.next_prime_mut();

        }
    }


    #[test]
    fn test_exponentiation() {
        let n: u32 = 200;
        let mut rng = RandState::new();
        let mut prime: Integer = Integer::from(2);


        for _ in 2..n {
            let p = prime.clone();
            let f = Arc::new(FiniteField::new(p.clone()).unwrap());

            let a_rand = Integer::from(rng.bits(32));
            let a = Element::new(f.clone(), a_rand.clone());

            let x_rand = Integer::from(rng.bits(32));

            // let a_exp_check = a_rand.exp_residue(&x_rand, &p);
            let a_exp_check = a_rand.pow_mod_ref(&x_rand, &p).unwrap().complete();
            let a_exp = a.pow(&x_rand);

            assert_eq!(a_exp.get_rep(), &a_exp_check);

            prime.next_prime_mut();
        }
    }
}
