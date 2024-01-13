use super::*;
use crate::{integral::float::*, ZeroAndOne};

mod arithmetic;
mod logic;

pub use arithmetic::*;
pub use logic::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector<T: Sized, const N: usize>(pub [T; N]);

impl<T: Sized + Copy, const N: usize> Vector<T, N> {
    pub const fn new_explode(element: T) -> Self { Self([element; N]) }
    pub const fn new(elements: [T; N]) -> Self { Self(elements) }

    fn apply_unary<Fun: Fn(T) -> T>(mut self, fun: Fun) -> Self {
        for i in 0..N {
            self.0[i] = fun(self.0[i]);
        }

        self
    }

    fn apply_binary<Fun: Fn(T, T) -> T>(mut self, rhs: Self, fun: Fun) -> Self {
        for i in 0..N {
            self.0[i] = fun(self.0[i], rhs.0[i]);
        }

        self
    }

    fn apply_binary_scalar<Fun: Fn(T, T) -> T>(mut self, rhs: T, fun: Fun) -> Self {
        for i in 0..N {
            self.0[i] = fun(self.0[i], rhs);
        }

        self
    }

    fn apply_unary_generic<Res: Sized + Copy, Fun: Fn(T) -> Res>(mut self, default: Res, fun: Fun) -> Vector<Res, N> {
        let mut ret = Vector::new_explode(default);

        for i in 0..N {
            ret.0[i] = fun(self.0[i]);
        }

        ret
    }

    fn apply_binary_generic<Res: Sized + Copy, Fun: Fn(T, T) -> Res>(mut self, default: Res, rhs: Self, fun: Fun) -> Vector<Res, N> {
        let mut ret = Vector::new_explode(default);

        for i in 0..N {
            ret.0[i] = fun(self.0[i], rhs.0[i]);
        }

        ret
    }

    fn apply_binary_scalar_generic<Res: Sized + Copy, Fun: Fn(T, T) -> Res>(mut self, default: Res, rhs: T, fun: Fun) -> Vector<Res, N> {
        let mut ret = Vector::new_explode(default);

        for i in 0..N {
            ret.0[i] = fun(self.0[i], rhs);
        }

        ret
    }
}

impl<T, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.0[index] }
}
