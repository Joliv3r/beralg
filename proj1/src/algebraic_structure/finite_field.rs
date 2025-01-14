// This module uses a lot of cloning of structures, which can be avoided by elements having
// reference to outer structure.


use crate::algebraic_structure::{HasMul, HasAdd, HasRepresentation, Element};
use number_theory::{NumberTheory, Mpz};
use std::sync::Arc;


use super::HasMulInv;

#[derive(Debug, Clone)]
pub struct FiniteField {
    // This struct will only consider finite fields isomorphic to Z_p for p prime.
    size: Mpz,
}


impl HasRepresentation for FiniteField {
    fn make_representation(&self, repr: Mpz) -> Mpz {
        repr.residue(self.mod_num())
    }
}


impl HasMul for FiniteField {
    fn mul(&self, a: &Element<FiniteField>, b: &Element<FiniteField>) -> Element<FiniteField> {
        Element::new(
            a.get_outer_structure(),
            a.representation.product_residue(&b.representation, self.mod_num())
        )
    }
}


impl HasAdd for FiniteField {
    fn add(&self, a: &Element<FiniteField>, b: &Element<FiniteField>) -> Element<FiniteField> {
        Element::new(
            a.get_outer_structure().clone(),
            (a.representation.ref_addition(&b.representation)).residue(self.mod_num())
        )
    }
}


impl FiniteField {
    pub fn new(size: Mpz) -> Option<FiniteField> {
        if size.is_prime() {
            Some(FiniteField {
                size,
            })
        } else {
            None
        }
    }

    pub fn get_size(&self) -> Mpz {
        self.size.clone()
    }

    fn mod_num(&self) -> &Mpz {
        &self.size
    }

    fn extended_euclidean_ordered(&self, a: &Mpz, b: &Mpz) -> (Mpz, Mpz, Mpz) {
        // There have been no attempt to optimize this function using cloning and references
        // efficiently.
        let mut a1: Mpz = a.clone();
        let mut b1: Mpz = b.clone();

        if b == &Mpz::from_u64(0) {
            return (a1.clone(), Mpz::one(), Mpz::zero())
        };

        let mut x: Mpz = Mpz::zero();
        let mut y: Mpz = Mpz::zero();
        
        let mut x2: Mpz = Mpz::one();
        let mut x1: Mpz = Mpz::zero();
        let mut y2: Mpz = Mpz::zero();
        let mut y1: Mpz = Mpz::one();

        let mut q: Mpz = Mpz::zero();
        let mut r: Mpz = Mpz::zero();

        while b.u_cmp(&Mpz::zero()).is_gt() {
            // q = a1/b1;
            (q, _) = a1.euclidean_div(&b1);
            // r = a1 - q*b1
            r = Mpz::ref_subtraction(&a1, &Mpz::ref_product(&q, &b1));
            // x = x2 - q*x1;
            x = Mpz::ref_subtraction(&x2, &Mpz::ref_product(&q, &x1));
            // y = y2 - q*y1;
            y = Mpz::ref_subtraction(&y2, &Mpz::ref_product(&q, &y1));
            a1 = b1;
            b1 = r;
            x2 = x1;
            x1 = x;
            y2 = y1;
            y1 = y;
        };

        x = x2;
        y = y2;

        (a1, x, y)
    }


    fn extended_euclidean_to_integers(&self, a: &Element<FiniteField>, b: &Element<FiniteField>) -> (Mpz, Mpz, Mpz) {
        let a_rep: &Mpz = a.get_rep();
        let b_rep: &Mpz = b.get_rep();
        if a_rep.u_cmp(b_rep).is_gt() {
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
        let p = &Element::new(
            a.get_outer_structure(),
            self.mod_num().clone(),
        );
        let (_, x, _) = self.extended_euclidean(a, p);
        x 
    }
}


#[derive(Debug, Clone)]
pub struct MultiplicativeGroup {
    mod_num: Mpz,
}


impl HasRepresentation for MultiplicativeGroup {
    fn make_representation(&self, repr: Mpz) -> Mpz {
        repr.residue(self.mod_num())
    }
}


impl HasMul for MultiplicativeGroup {
    fn mul(&self, a: &Element<MultiplicativeGroup>, b: &Element<MultiplicativeGroup>) -> Element<MultiplicativeGroup> {
        Element::new(
            a.get_outer_structure(),
            a.representation.ref_product(&b.representation).residue(self.mod_num())
        )
    }
}


impl MultiplicativeGroup {
    pub fn new(mod_num: Mpz) -> MultiplicativeGroup {
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


    pub fn mod_num(&self) -> &Mpz {
        &self.mod_num
    }


    pub fn get_size(&self) -> Mpz {
        self.mod_num.ref_addition(&Mpz::one())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic_structure::Element;
    use rand::{self, RngCore};

    #[test]
    fn test_algebraic_structure_arithmetic() {
        let n: u32 = 200;
        let rand_len = 12;

        for i in 2..n {
            if i.is_prime() {
                let p = &Mpz::from_u64(i as u64);
                let f: Arc<FiniteField> = Arc::new(FiniteField::new(p.clone()).unwrap());
                let a_rand: Mpz = Mpz::rand(rand_len);
                let b_rand: Mpz = Mpz::rand(rand_len);
                let a = Element::new(f.clone(), a_rand.clone());
                let b = Element::new(f.clone(), b_rand.clone());

                assert_eq!(a.get_rep(), &a_rand.residue(p));
                assert_eq!(b.get_rep(), &b_rand.residue(p));

                let added = (a.add_ref(&b)).get_rep().clone();
                let multiplied = (a.mul_ref(&b)).get_rep().clone();

                let added_check: Mpz = a_rand.residue(p).ref_addition(&b_rand.residue(p)).residue(p);
                let mul_check: Mpz = a_rand.residue(p).ref_product(&b_rand.residue(p)).residue(p);

                assert_eq!(added, added_check, "Failed for: {} + {}, and got {} instead of {}", a.get_rep(), b.get_rep(), added, added_check);
                assert_eq!(multiplied, mul_check, "Failed for: {} * {}, and got {} instead of {}", a.get_rep(), b.get_rep(), multiplied, mul_check);

                let g: Arc<MultiplicativeGroup> = Arc::new(MultiplicativeGroup::from_finite_field(&f));
                let a_rand: Mpz = Mpz::rand(rand_len);
                let b_rand: Mpz = Mpz::rand(rand_len);
                let a = Element::new(g.clone(), a_rand.clone());
                let b = Element::new(g.clone(), b_rand.clone());

                assert_eq!(a.get_rep(), &a_rand.residue(p), "Failed representation of {} in Z_{},  got {} instead of {}", a_rand, p, a.get_rep(), a_rand.residue(p));
                assert_eq!(b.get_rep(), &b_rand.residue(p), "Failed representation of {} in Z_{},  got {} instead of {}", b_rand, p, b.get_rep(), b_rand.residue(p));

                let multiplied = (a.mul_ref(&b)).representation;
                let mul_check: Mpz = a_rand.residue(p).ref_product(&b_rand.residue(p)).residue(p);
                assert_eq!(multiplied, mul_check);
            }
        }
    }


    #[test]
    fn test_extended_euclidean() {
        let n: u32 = 200;
        let rand_len = 12;

        for i in 2..n {
            if i.is_prime() {
                let p = Mpz::from_u64(i as u64);
                let f = Arc::new(FiniteField::new(p.clone()).unwrap());
                let mut a_rand = Mpz::rand(rand_len);
                let mut a = Element::new(f.clone(), a_rand);
                // while a_rand.residue(&p).u_cmp(&Mpz::zero()).is_ne()  {
                //     a_rand = Mpz::rand(rand_len);
                //     a = Element::new(f.clone(), a_rand);
                // }

                let a_inv = a.inv();
                assert_eq!(&Mpz::one(), a_inv.get_rep(), "We have a: {}, a_inv: {}, p: {}", a.representation, a_inv.representation, p);

            }
        }
    }


    #[test]
    fn test_exponentiation() {
        let p: Mpz = Mpz::from_u64(10).pow(10).ref_addition(&Mpz::from_u64(19));
        let rand_len = 500;
        let f = Arc::new(FiniteField::new(p.clone()).unwrap());

        let a_rand = Mpz::rand(rand_len);
        // let a_rand = Mpz::from_u64(4);
        let a = Element::new(f.clone(), a_rand.clone());

        let mut x_rand = Mpz::rand(rand_len);
        // let x_rand = Mpz::from_u64(2);
        // let x = Element::new(f.clone(), x_rand.clone());

        let a_exp_check = a_rand.exp_residue(&x_rand, &p);
        let a_exp = a.pow(x_rand);

        assert_eq!(a_exp.get_rep(), &a_exp_check);
    }
}
