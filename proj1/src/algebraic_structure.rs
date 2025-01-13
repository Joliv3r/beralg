use std::ops;

pub mod finite_field;

pub trait ModIso {
    // For structures where modulo makes sense.
    fn mod_num(&self) -> u32;
}


#[derive(Debug, Clone, Copy)]
pub struct Element<T: ModIso> {
    outer_structure: T,
    representation: u32,
}


impl<T: ModIso> Element<T> {
    pub fn new(outer_structure: T, repr: u32) -> Element<T> {
        let representation: u32 = repr % outer_structure.mod_num();
        Element { outer_structure, representation }
    }
}


impl<T: ModIso> ops::Mul<Element<T>> for Element<T> {
    type Output = Element<T>;

    fn mul(self, _rhs: Element<T>) -> Element<T> {
        let rep: u32 = (self.representation * _rhs.representation) % self.outer_structure.mod_num();
        Element { outer_structure: self.outer_structure, representation: rep }
    }
}


impl<T: ModIso> ops::Add<Element<T>> for Element<T> {
    type Output = Element<T>;

    fn add(self, _rhs: Element<T>) -> Element<T> {
        let rep: u32 = (self.representation + _rhs.representation) % self.outer_structure.mod_num();
        Element { outer_structure: self.outer_structure, representation: rep }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebraic_structure::finite_field::{FiniteField, MultiplicativeGroup};

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
