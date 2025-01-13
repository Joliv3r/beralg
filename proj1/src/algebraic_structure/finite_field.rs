use crate::algebraic_structure::ModIso;
use number_theory::NumberTheory;

#[derive(Debug, Clone, Copy)]
pub struct FiniteField {
    // This struct will only consider finite fields isomorphic to Z_p for p prime.
    size: u32,
}


impl ModIso for FiniteField {
    fn mod_num(&self) -> u32 {
        self.size
    }
}


impl FiniteField {
    pub fn new(size: u32) -> Option<FiniteField> {
        if size.is_prime() {
            Some(FiniteField {
                size,
                // representation: Vec::from_iter(0..size),
            })
        } else {
            None
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    // fn extended_euclidean_ordered(&self, a: u32, b: u32) -> (u32, u32, u32) {
    //     if b = 0 {
    //         let d: u32 = a;
    //         let x: u32 = 1;
    //         let y: u32 = 0;
    //         return (d, x, y)
    //     };
    //     
    //     let x2: u32 = 1;
    //     let x1: u32 = 0;
    //     let y2: u32 = 0;
    //     let y1: u32 = 1;
    //
    //     while b > 0 {
    //         let q: u32 = 
    //     }
    // }

    // pub fn extended_euclidean(&self, a: Element<FiniteField>, b: Element<FiniteField>) -> (Element<FiniteField>, Element<FiniteField>, Element<FiniteField>) {
    //     if a > b {
    //         self.extended_euclidean_ordered(a, b)
    //     } else {
    //         self.extended_euclidean_ordered(b, a)
    //     }
    // }
}


#[derive(Debug, Clone, Copy)]
pub struct MultiplicativeGroup {
    size: u32,
}


impl ModIso for MultiplicativeGroup {
    fn mod_num(&self) -> u32 {
        self.size+1
    }
}


impl MultiplicativeGroup {
    pub fn new(size: u32) -> MultiplicativeGroup {
        MultiplicativeGroup {
            size,
            // representation: Vec::from_iter(1..size+2),
        }
    }


    pub fn from_finite_field(finite_field: &FiniteField) -> MultiplicativeGroup {
        let size = finite_field.get_size()-1;
        MultiplicativeGroup {
            size,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic_structure::Element;

    #[test]
    fn test_algebraic_structure_arithmetic() {
        let f: FiniteField = FiniteField::new(17).unwrap();
        let a = Element::new(f, 234);
        let b = Element::new(f, 23);

        assert_eq!(a.representation, 234%17);
        assert_eq!(b.representation, 6);

        let added = (a + b).representation;
        let multiplied = (a * b).representation;

        assert_eq!(added, (234+23)%17);
        assert_eq!(multiplied, (234*23)%17);

        let g: MultiplicativeGroup = MultiplicativeGroup::from_finite_field(&f);
        let a = Element::new(g, 72);
        let b = Element::new(g, 186);

        assert_eq!(a.representation, 4);
        assert_eq!(b.representation, 16);

        let multiplied = (a * b).representation;
        assert_eq!(multiplied, 13);
    }
}
