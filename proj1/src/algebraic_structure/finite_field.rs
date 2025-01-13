use crate::algebraic_structure::{HasMul, HasAdd, HasRepresentation, Element};
use number_theory::NumberTheory;

use super::HasMulInv;

#[derive(Debug, Clone, Copy)]
pub struct FiniteField {
    // This struct will only consider finite fields isomorphic to Z_p for p prime.
    size: u32,
}


impl HasRepresentation for FiniteField {
    fn make_representation(&self, repr: u32) -> u32 {
        repr % self.mod_num()
    }
}


impl HasMul for FiniteField {
    fn mul(&self, a: Element<FiniteField>, b: Element<FiniteField>) -> Element<FiniteField> {
        Element::new(
            *self,
            (a.representation * b.representation) % self.mod_num()
        )
    }
}


impl HasAdd for FiniteField {
    fn add(&self, a: Element<FiniteField>, b: Element<FiniteField>) -> Element<FiniteField> {
        Element::new(
            *self,
            (a.representation + b.representation) % self.mod_num()
        )
    }
}


impl FiniteField {
    pub fn new(size: u32) -> Option<FiniteField> {
        if size.is_prime() {
            Some(FiniteField {
                size,
            })
        } else {
            None
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    fn mod_num(&self) -> u32 {
        self.size
    }

    fn extended_euclidean_ordered(&self, a: u32, b: u32) -> (u32, u32, u32) {
        let mut a1: u32 = a;
        let mut b1: u32 = b;

        if b == 0 {
            return (a1, 1, 0)
        };

        let mut d: u32 = 0;
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        
        let mut x2: u32 = 1;
        let mut x1: u32 = 0;
        let mut y2: u32 = 0;
        let mut y1: u32 = 1;

        let mut q: u32 = 0;
        let mut r: u32 = 0;

        while b > 0 {
            q = a1/b1;
            r = a1 - q*b1;
            x = x2 - q*x1;
            y = y2 - q*y1;
            a1 = b1;
            b1 = r;
            x2 = x1;
            x1 = x;
            y2 = y1;
            y1 = y;
        };

        d = a;
        x = x2;
        y = y2;

        (d, x, y)
    }


    fn extended_euclidean_to_integers(&self, a: Element<FiniteField>, b: Element<FiniteField>) -> (u32, u32, u32) {
        let a_rep: u32 = a.representation;
        let b_rep: u32 = b.representation;
        if a_rep > b_rep {
            self.extended_euclidean_ordered(a_rep, b_rep)
        } else {
            self.extended_euclidean_ordered(b_rep, a_rep)
        }
    }


    pub fn extended_euclidean(&self, a: Element<FiniteField>, b: Element<FiniteField>) -> (Element<FiniteField>, Element<FiniteField>, Element<FiniteField>) {
        let (d, x, y) = self.extended_euclidean_to_integers(a, b);
        (
            Element::new(
                *self,
                d
            ),
            Element::new(
                *self,
                x
            ),
            Element::new(
                *self,
                y
            ),
        )
    }
}


impl HasMulInv for FiniteField {
    fn inv(&self, a: Element<Self>) -> Element<Self> {
        let p = Element::new(
            *self,
            self.mod_num(),
        );
        let (_, x, _) = self.extended_euclidean(a, p);
        x 
    }
}


#[derive(Debug, Clone, Copy)]
pub struct MultiplicativeGroup {
    size: u32,
}


impl HasRepresentation for MultiplicativeGroup {
    fn make_representation(&self, repr: u32) -> u32 {
        repr % self.mod_num()
    }
}


impl HasMul for MultiplicativeGroup {
    fn mul(&self, a: Element<MultiplicativeGroup>, b: Element<MultiplicativeGroup>) -> Element<MultiplicativeGroup> {
        Element::new(
            *self,
            (a.representation * b.representation) % self.mod_num()
        )
    }
}


impl MultiplicativeGroup {
    pub fn new(size: u32) -> MultiplicativeGroup {
        MultiplicativeGroup {
            size,
        }
    }


    pub fn from_finite_field(finite_field: &FiniteField) -> MultiplicativeGroup {
        let size = finite_field.get_size()-1;
        MultiplicativeGroup {
            size,
        }
    }


    pub fn mod_num(&self) -> u32 {
        self.size+1
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
        let mut rng = rand::thread_rng();

        for p in 2..n {
            if p.is_prime() {
                let f: FiniteField = FiniteField::new(p).unwrap();
                let a_rand: u32 = rng.next_u32();
                let b_rand: u32 = rng.next_u32();
                let a = Element::new(f, a_rand);
                let b = Element::new(f, b_rand);

                assert_eq!(a.representation, a_rand%p);
                assert_eq!(b.representation, b_rand%p);

                let added = (a + b).representation;
                let multiplied = (a * b).representation;

                assert_eq!(added, ((a_rand%p)+(b_rand%p))%p);
                assert_eq!(multiplied, ((a_rand%p)*(b_rand%p))%p);

                let g: MultiplicativeGroup = MultiplicativeGroup::from_finite_field(&f);
                let a_rand = rng.next_u32();
                let b_rand = rng.next_u32();
                let a = Element::new(g, a_rand);
                let b = Element::new(g, b_rand);

                assert_eq!(a.representation, a_rand%p);
                assert_eq!(b.representation, b_rand%p);

                let multiplied = (a * b).representation;
                assert_eq!(multiplied, ((a_rand%p)*(b_rand%p))%p);
            }
        }
    }


    #[test]
    fn test_extended_euclidean() {
        let n: u32 = 200;
        let mut rng = rand::thread_rng();

        for p in 2..n {
            if p.is_prime() {
                let f = FiniteField::new(p).unwrap();
                let mut a_rand: u32 = rng.next_u32();
                let mut a = Element::new(f, a_rand);
                while a_rand%p == 0 {
                    a_rand = rng.next_u32();
                    a = Element::new(f, a_rand);
                }

                let a_inv = a.inv();
                assert_eq!(1, a_inv.representation, "We have a: {}, a_inv: {}, p: {}", a.representation, a_inv.representation, p);

            }
        }
    }
}
