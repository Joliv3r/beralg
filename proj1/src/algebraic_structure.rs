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
