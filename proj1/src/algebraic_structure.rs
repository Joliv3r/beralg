// This is in addition to computational algebra a personal learning experience with rust.

use std::ops;
pub mod finite_field;


pub trait HasRepresentation {
    fn make_representation(&self, repr: u32) -> u32;
}


#[derive(Debug, Clone, Copy)]
pub struct Element<T: HasRepresentation + Copy> {
    outer_structure: T,
    representation: u32,
}


pub trait HasMul<T: HasRepresentation + Copy = Self> {
    fn mul(&self, a: Element<T>, b: Element<T>) -> Element<T>;
}


pub trait HasAdd<T: HasRepresentation + Copy = Self> {
    fn add(&self, a: Element<T>, b: Element<T>) -> Element<T>;
}


pub trait HasMulInv<T: HasRepresentation + HasMul + Copy = Self> {
    fn inv(&self, a: Element<T>) -> Element<T>;
}


impl<T: HasRepresentation + Copy> Element<T> {
    pub fn new(outer_structure: T, repr: u32) -> Element<T> {
        let representation: u32 = outer_structure.make_representation(repr);
        Element { outer_structure, representation }
    }
}


impl<T: HasMul + HasRepresentation + Copy> ops::Mul<Element<T>> for Element<T> {
    type Output = Element<T>;

    fn mul(self, _rhs: Element<T>) -> Element<T> {
        self.outer_structure.mul(self, _rhs)
    }
}


impl<T: HasMul + HasRepresentation + Copy> Element<T> {
    pub fn pow(a: i32) -> Element<T> {
        todo!()
    }
}


impl<T: HasAdd + HasRepresentation + Copy> ops::Add<Element<T>> for Element<T> {
    // Addition implemented even in structures without addition.
    type Output = Element<T>;

    fn add(self, _rhs: Element<T>) -> Element<T> {
        self.outer_structure.add(self, _rhs)
    }
}


impl<T: HasMulInv + HasMul + HasRepresentation + Copy> Element<T> {
    fn inv(self) -> Element<T> {
        self.outer_structure.inv(self)
    }
}
