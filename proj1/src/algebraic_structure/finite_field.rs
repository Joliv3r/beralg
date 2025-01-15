// This module uses a lot of cloning of structures, which can be avoided by elements having
// reference to outer structure.

use crate::algebraic_structure::{HasMul, HasAdd, HasRepresentation, Element};
use rug::{integer::IsPrime, Complete, Integer};


use super::HasMulInv;

#[derive(Debug, Clone)]
pub struct FiniteField {
    // This struct will only consider finite fields isomorphic to Z_p for p prime.
    size: Integer,
}


impl HasRepresentation for FiniteField {
    fn make_representation(&self, repr: Integer) -> Integer {
        repr.modulo(self.mod_num())
    }
}


impl HasMul for FiniteField {
    fn mul(&self, a: &Element<FiniteField>, b: &Element<FiniteField>) -> Element<FiniteField> {
        Element::new(
            a.get_outer_structure(),
            (a.get_rep() * b.get_rep()).complete() % self.mod_num()
        )
    }
}


impl HasAdd for FiniteField {
    fn add(&self, a: &Element<FiniteField>, b: &Element<FiniteField>) -> Element<FiniteField> {
        Element::new(
            a.get_outer_structure().clone(),
            (a.get_rep() + b.get_rep()).complete() % self.mod_num()
        )
    }
}


impl FiniteField {
    pub fn new(size: Integer) -> Option<FiniteField> {
        if size.is_probably_prime(30) != IsPrime::No {
            Some(FiniteField {
                size,
            })
        } else {
            None
        }
    }

    pub fn get_size(&self) -> Integer {
        self.size.clone()
    }

    fn mod_num(&self) -> &Integer {
        &self.size
    }

    fn extended_euclidean_ordered(&self, a: &Integer, b: &Integer) -> (Integer, Integer, Integer) {
        // There have been no attempt to optimize this function using cloning and references
        // efficiently.
        let mut a1: Integer = a.clone();
        let mut b1: Integer = b.clone();

        if b.is_zero() {
            return (a1, Integer::ONE.clone(), Integer::ZERO.clone())
        };

        let mut x: Integer = Integer::ZERO.clone();
        let mut y: Integer = Integer::ZERO.clone();
        
        let mut x2: Integer = Integer::ONE.clone();
        let mut x1: Integer = Integer::ZERO.clone();
        let mut y2: Integer = Integer::ZERO.clone();
        let mut y1: Integer = Integer::ONE.clone();

        let mut q: Integer = Integer::ZERO.clone();
        let mut r: Integer = Integer::ZERO.clone();


        while b1 > Integer::ZERO {
            // q = a1/b1;
            (q, _) = (a1.div_rem_floor_ref(&b1)).complete();
            r = a1 - &q*&b1;
            x = x2 - &q*&x1;
            y = y2 - &q*&y1;
            a1 = b1;
            b1 = r;
            x2 = x1;
            x1 = x;
            y2 = y1;
            y1 = y;
        };

        (a1, x2, y2)
    }


    fn extended_euclidean_to_integers(&self, a: &Element<FiniteField>, b: &Element<FiniteField>) -> (Integer, Integer, Integer) {
        let a_rep: &Integer = a.get_rep();
        let b_rep: &Integer = b.get_rep();
        if a_rep > b_rep {
            self.extended_euclidean_ordered(a_rep, b_rep)
        } else {
            self.extended_euclidean_ordered(b_rep, a_rep)
        }
    }


    pub fn extended_euclidean(&self, a: &Element<FiniteField>, b: &Element<FiniteField>) -> (Element<FiniteField>, Element<FiniteField>, Element<FiniteField>) {
        let (d, x, y) = self.extended_euclidean_to_integers(a, b);
        (
            Element::new(
                a.get_outer_structure(),
                d
            ),
            Element::new(
                a.get_outer_structure(),
                x
            ),
            Element::new(
                a.get_outer_structure(),
                y
            ),
        )
    }
}


impl HasMulInv for FiniteField {
    fn inv(&self, a: &Element<Self>) -> Element<Self> {
        let (_, _, y) = self.extended_euclidean_ordered(self.mod_num(), a.get_rep());
        Element::new(
            a.get_outer_structure(),
            y
        )
    }
}


#[derive(Debug, Clone)]
pub struct MultiplicativeGroup {
    mod_num: Integer,
}


impl HasRepresentation for MultiplicativeGroup {
    fn make_representation(&self, repr: Integer) -> Integer {
        repr.modulo(self.mod_num())
    }
}


impl HasMul for MultiplicativeGroup {
    fn mul(&self, a: &Element<MultiplicativeGroup>, b: &Element<MultiplicativeGroup>) -> Element<MultiplicativeGroup> {
        Element::new(
            a.get_outer_structure(),
            (a.get_rep() * b.get_rep()).complete() % self.mod_num()
        )
    }
}


impl MultiplicativeGroup {
    pub fn new(mod_num: Integer) -> MultiplicativeGroup {
        MultiplicativeGroup {
            mod_num,
        }
    }


    pub fn from_finite_field(finite_field: &FiniteField) -> MultiplicativeGroup {
        let mod_num = finite_field.mod_num().clone();
        MultiplicativeGroup {
            mod_num,
        }
    }


    pub fn mod_num(&self) -> &Integer {
        &self.mod_num
    }


    pub fn get_size(&self) -> Integer {
        (self.mod_num() + Integer::ONE).complete()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic_structure::Element;
    use rug::rand::RandState;
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
            let a_rand: Integer = Integer::from(rng.bits(32));
            let b_rand: Integer = Integer::from(rng.bits(32));
            let a = Element::new(g.clone(), a_rand.clone());
            let b = Element::new(g.clone(), b_rand.clone());

            assert_eq!(a.get_rep(), &(&a_rand % &p).complete(), "Failed representation of {} in Z_{},  got {} instead of {}", &a_rand, &p, a.get_rep(), (&a_rand % &p).complete());
            assert_eq!(b.get_rep(), &(&b_rand % &p).complete(), "Failed representation of {} in Z_{},  got {} instead of {}", &b_rand, &p, b.get_rep(), (&b_rand % &p).complete());

            let multiplied = (a.mul_ref(&b)).representation;
            let mul_check: Integer = ((a_rand % &p) * (b_rand % &p)) % &p;
            assert_eq!(multiplied, mul_check);

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
            let a_exp = a.pow(x_rand);

            assert_eq!(a_exp.get_rep(), &a_exp_check);

            prime.next_prime_mut();
        }
    }
}
