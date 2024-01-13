use super::*;

impl<T: Sized + Copy + std::cmp::PartialOrd, const N: usize> Vector<T, N> {
    /// ```
    /// use stuff::smallvec::*;
    ///
    /// let vec1 = Vector([0, 1, 2]);
    /// let vec2 = Vector([3, 1, 1]);
    ///
    /// assert_eq!(Vector::lt(&vec1, &vec2), Vector([true, false, false]));
    /// assert_eq!(Vector::eq(&vec1, &vec2), Vector([false, true, false]));
    /// assert_eq!(Vector::gt(&vec1, &vec2), Vector([false, false, true]));
    /// ```
    pub fn partial_cmp(&self, other: &Vector<T, N>) -> Vector<Option<std::cmp::Ordering>, N> {
        let mut ret = Vector::new_explode(None);

        for i in 0..N {
            ret[i] = self[i].partial_cmp(&other[i])
        }

        ret
    }

    pub fn lt(&self, other: &Vector<T, N>) -> Vector<bool, N> { self.partial_cmp(other).apply_unary_generic(false, |lhs| matches!(lhs, Some(std::cmp::Ordering::Less))) }
    pub fn eq(&self, other: &Vector<T, N>) -> Vector<bool, N> { self.partial_cmp(other).apply_unary_generic(false, |lhs| matches!(lhs, Some(std::cmp::Ordering::Equal))) }
    pub fn gt(&self, other: &Vector<T, N>) -> Vector<bool, N> { self.partial_cmp(other).apply_unary_generic(false, |lhs| matches!(lhs, Some(std::cmp::Ordering::Greater))) }
}

impl<T: Sized + Copy + ZeroAndOne, const N: usize> std::ops::Mul<Vector<bool, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn mul(mut self, rhs: Vector<bool, N>) -> Self::Output {
        for i in 0..N {
            self[i] = if rhs[i] { self[i] } else { T::zero() };
        }

        self
    }
}

impl<const N: usize> std::ops::BitAnd<Vector<bool, N>> for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn bitand(self, rhs: Vector<bool, N>) -> Self::Output { self.apply_binary(rhs, std::ops::BitAnd::bitand) }
}

impl<const N: usize> std::ops::BitOr<Vector<bool, N>> for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn bitor(self, rhs: Vector<bool, N>) -> Self::Output { self.apply_binary(rhs, std::ops::BitOr::bitor) }
}

impl<const N: usize> std::ops::BitXor<Vector<bool, N>> for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn bitxor(self, rhs: Vector<bool, N>) -> Self::Output { self.apply_binary(rhs, std::ops::BitXor::bitxor) }
}

impl<const N: usize> std::ops::Not for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn not(self) -> Self::Output { self.apply_unary(std::ops::Not::not) }
}
