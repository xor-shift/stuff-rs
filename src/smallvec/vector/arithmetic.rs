use super::*;

impl<T: Number, const N: usize> Vector<T, N> {
    pub fn dot(self, rhs: Self) -> T {
        self.apply_binary(rhs, std::ops::Mul::mul) //
            .0
            .iter()
            .map(|&v| v)
            .fold(T::zero(), std::ops::Add::add)
    }

    pub fn abs(mut self) -> Self {
        for i in 0..N {
            self.0[i] = self.0[i].abs();
        }

        self
    }

    pub fn clamp(mut self, lo: T, hi: T) -> Self {
        for i in 0..N {
            self.0[i] = self.0[i].clamp(lo, hi);
        }

        self
    }

    pub fn reciprocal(self) -> Self { self.apply_binary_scalar(T::one(), |v, _| T::one() / v) }
}

impl<T: Number + FloatingPoint, const N: usize> Vector<T, N> {
    pub fn length(&self) -> T { self.0.iter().map(|&v| v * v).fold(<T as ZeroAndOne>::zero(), std::ops::Add::add).sqrt() }

    pub fn normalized(self) -> Self { self / self.length() }

    pub fn powf_scalar(self, pow: T) -> Self { self.apply_binary_scalar(pow, T::powf) }
    pub fn powf_vector(self, pow: Self) -> Self { self.apply_binary(pow, T::powf) }
}

impl<T: Number + FloatingPoint> Vector<T, 3> {
    pub fn cross(self, other: Self) -> Self {
        let [a1, a2, a3] = self.0;
        let [b1, b2, b3] = other.0;

        let s1 = a2 * b3 - a3 * b2;
        let s2 = a3 * b1 - a1 * b3;
        let s3 = a1 * b2 - a2 * b1;

        Self([s1, s2, s3])
    }
}

impl<T: Number, const N: usize> std::ops::Add<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    /// ```
    /// use stuff::smallvec::Vector;
    ///
    /// let lhs = Vector::new_explode(1f32);
    /// let rhs = Vector([1f32, 2f32, 3f32]);
    /// assert_eq!(lhs + rhs, Vector([2f32, 3f32, 4f32]));
    /// ```
    fn add(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Add::add) }
}

impl<T: Number, const N: usize> std::ops::Mul<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Mul::mul) }
}

impl<T: Number, const N: usize> std::ops::Sub<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn sub(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Sub::sub) }
}

impl<T: Number, const N: usize> std::ops::Div<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, rhs: Vector<T, N>) -> Self::Output { self.apply_binary(rhs, std::ops::Div::div) }
}

impl<T: Number, const N: usize> std::ops::Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn mul(self, rhs: T) -> Self::Output { self.apply_binary_scalar(rhs, std::ops::Mul::mul) }
}

impl<T: Number, const N: usize> std::ops::Div<T> for Vector<T, N> {
    type Output = Vector<T, N>;
    fn div(self, rhs: T) -> Self::Output { self.apply_binary_scalar(rhs, std::ops::Div::div) }
}

impl<T: Number, const N: usize> std::ops::Neg for Vector<T, N> {
    type Output = Vector<T, N>;
    fn neg(mut self) -> Self::Output {
        for i in 0..N {
            self.0[i] = -self.0[i];
        }

        self
    }
}
