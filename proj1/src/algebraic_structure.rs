// This is in addition to computational algebra a personal learning experience with rust.

use std::ops;
use rug::Integer;
use std::sync::Arc;
pub mod finite_field;


pub trait HasRepresentation {
    fn make_representation(&self, repr: Integer) -> Integer;
}


#[derive(Debug, Clone)]
pub struct Element<T: HasRepresentation + Clone> {
    outer_structure: Arc<T>,
    representation: Integer,
}


pub trait HasMul<T: HasRepresentation + Clone = Self> {
    fn mul(&self, a: &Element<T>, b: &Element<T>) -> Element<T>;
}


pub trait HasAdd<T: HasRepresentation + Clone = Self> {
    fn add(&self, a: &Element<T>, b: &Element<T>) -> Element<T>;
}


pub trait HasMulInv<T: HasRepresentation + HasMul + Clone = Self> {
    fn inv(&self, a: &Element<T>) -> Element<T>;
}


impl<T: HasRepresentation + Clone> Element<T> {
    pub fn new(outer_structure: Arc<T>, repr: Integer) -> Element<T> {
        let representation: Integer = outer_structure.make_representation(repr);
        Element { outer_structure, representation }
    }


    pub fn get_outer_structure(&self) -> Arc<T> {
        self.outer_structure.clone()
    }


    pub fn get_rep(&self) -> &Integer {
        &self.representation
    }
}


impl<T: HasMul + HasRepresentation + Clone> Element<T> {
    fn mul_ref(&self, _rhs: &Element<T>) -> Element<T> {
        self.outer_structure.mul(&self, _rhs)
    }
}


impl<T: HasMul + HasRepresentation + Clone> Element<T> {
    pub fn pow(&self, mut a: Integer) -> Element<T> {
        let mut product = Element::new(self.get_outer_structure(), Integer::ONE.clone());
        let mut base = self.clone();

        while !a.is_zero() {
            if a.get_bit(0) {
                product = product.mul_ref(&base);
            }
            base = base.mul_ref(&base);

            a = a >> 1;
        }

        product
    }
}


impl<T: HasAdd + HasRepresentation + Clone> Element<T> {
    fn add_ref(&self, _rhs: &Element<T>) -> Element<T> {
        self.outer_structure.add(&self, _rhs)
    }
}


impl<T: HasMulInv + HasMul + HasRepresentation + Clone> Element<T> {
    fn inv(&self) -> Element<T> {
        self.outer_structure.inv(&self)
    }
}
