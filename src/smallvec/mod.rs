use crate::{integral::float::*, ZeroAndOne};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector<T: FloatingPoint, const N: usize> {
    data: [T; N],
}

impl<T: FloatingPoint, const N: usize> Vector<T, N> {
    pub const fn new_arr(data: [T; N]) -> Self { Self { data } }
    pub const fn new_explode(element: T) -> Self { Self { data: [element; N] } }

    fn apply_binary<Fun: Fn(T, T) -> T>(mut self, rhs: Self, fun: Fun) -> Self {
        for i in 0..N {
            self.data[i] = fun(self.data[i], rhs.data[i]);
        }

        self
    }

    fn apply_binary_scalar<Fun: Fn(T, T) -> T>(mut self, rhs: T, fun: Fun) -> Self {
        for i in 0..N {
            self.data[i] = fun(self.data[i], rhs);
        }

        self
    }

    pub fn length(&self) -> T { self.data.iter().map(|&v| v * v).fold(<T as ZeroAndOne>::zero(), std::ops::Add::add).sqrt() }

    pub fn normalized(self) -> Self { self / self.length() }

    pub fn dot(self, rhs: Self) -> T {
        self.apply_binary(rhs, std::ops::Mul::mul) //
            .data
            .iter()
            .map(|&v| v)
            .fold(<T as ZeroAndOne>::zero(), std::ops::Add::add)
    }

    pub fn abs(mut self) -> Self {
        for i in 0..N {
            self.data[i] = self.data[i].abs();
        }

        self
    }
}

impl<T: FloatingPoint, const N: usize> std::ops::Add<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn add(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Add::add) }
}

impl<T: FloatingPoint, const N: usize> std::ops::Mul<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Mul::mul) }
}

impl<T: FloatingPoint, const N: usize> std::ops::Sub<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn sub(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Sub::sub) }
}

impl<T: FloatingPoint, const N: usize> std::ops::Div<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Div::div) }
}

impl<T: FloatingPoint, const N: usize> std::ops::Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: T) -> Self::Output { self.apply_binary_scalar(rhs, std::ops::Mul::mul) }
}

impl<T: FloatingPoint, const N: usize> std::ops::Div<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, rhs: T) -> Self::Output { self.apply_binary_scalar(rhs, std::ops::Div::div) }
}

impl<T: FloatingPoint, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.data[index] }
}

impl<T: FloatingPoint, const N: usize> std::ops::IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.data[index] }
}

impl<T: FloatingPoint, const N: usize> std::ops::Neg for Vector<T, N> {
    type Output = Vector<T, N>;
    fn neg(mut self) -> Self::Output {
        for i in 0..N {
            self.data[i] = -self.data[i];
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let lhs = Vector::new_explode(1f32);
        let rhs = Vector::new_arr([1f32, 2f32, 3f32]);
        assert_eq!(lhs + rhs, Vector::new_arr([2f32, 3f32, 4f32]));
    }
}
