use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix<T: Number, const ROWS: usize, const COLS: usize>(pub [T; ROWS * COLS])
where
    [T; ROWS * COLS]:;

impl<T: Number, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    [T; ROWS * COLS]:,
{
    pub fn new(elements: [T; ROWS * COLS]) -> Self { Self(elements) }

    const fn index(row: usize, col: usize) -> usize { row * COLS + col }

    /// ```
    /// use stuff::smallvec::Matrix;
    ///
    /// let mat = Matrix::<f32, 3, 3>([1., 0., 0., 0., 1., 0., 0., 0., 1.]);
    /// let mat = mat.transpose();
    /// assert_eq!(mat, Matrix::<f32, 3, 3>([1., 0., 0., 0., 1., 0., 0., 0., 1.]));
    ///
    /// let mat = Matrix::<f32, 3, 3>([1., 2., 3., 4., 5., 6., 7., 8., 9.]);
    /// let mat = mat.transpose();
    /// assert_eq!(mat, Matrix::<f32, 3, 3>([1., 4., 7., 2., 5., 8., 3., 6., 9.]));
    /// ```
    pub fn transpose(mut self) -> Self {
        for i in 0..ROWS {
            for j in i..COLS {
                self.0.swap(Self::index(i, j), Self::index(j, i));
            }
        }

        self
    }

    fn apply_binary<Fun: Fn(T, T) -> T>(mut self, rhs: Self, fun: Fun) -> Self {
        for i in 0..(ROWS * COLS) {
            self.0[i] = fun(self.0[i], rhs.0[i]);
        }

        self
    }

    fn apply_binary_scalar<Fun: Fn(T, T) -> T>(mut self, rhs: T, fun: Fun) -> Self {
        for i in 0..(ROWS * COLS) {
            self.0[i] = fun(self.0[i], rhs);
        }

        self
    }
}

impl<T: Number, const ROWS: usize, const COLS: usize> std::ops::Add<Matrix<T, ROWS, COLS>> for Matrix<T, ROWS, COLS>
where
    [T; ROWS * COLS]:,
{
    type Output = Matrix<T, ROWS, COLS>;
    fn add(self, rhs: Matrix<T, ROWS, COLS>) -> Self::Output { self.apply_binary(rhs, std::ops::Add::add) }
}

impl<T: Number, const ROWS: usize, const COLS: usize> std::ops::Sub<Matrix<T, ROWS, COLS>> for Matrix<T, ROWS, COLS>
where
    [T; ROWS * COLS]:,
{
    type Output = Matrix<T, ROWS, COLS>;
    fn sub(self, rhs: Matrix<T, ROWS, COLS>) -> Self::Output { self.apply_binary(rhs, std::ops::Sub::sub) }
}

impl<'a, T: Number, const ROWS: usize, const COLS: usize> std::ops::Index<usize> for &'a Matrix<T, ROWS, COLS>
where
    [T; ROWS * COLS]:,
{
    type Output = [T; COLS];

    /// ```
    /// use stuff::smallvec::Matrix;
    ///
    /// let mat = Matrix::<f32, 3, 3>([1., 2., 3., 4., 5., 6., 7., 8., 9.]);
    /// assert_eq!(mat[0][0], 1.);
    /// assert_eq!(mat[0][1], 2.);
    /// assert_eq!(mat[1][0], 4.);
    /// assert_eq!(mat[1][1], 5.);
    /// ```
    fn index(&self, index: usize) -> &'a Self::Output { (&self.0[index * COLS..(index + 1) * COLS]).try_into().unwrap() }
}

use crate::smallvec::Vector;

impl<T: Number, const ROWS: usize, const COLS: usize> std::ops::Mul<Vector<T, COLS>> for Matrix<T, ROWS, COLS>
where
    [(); ROWS * COLS]:,
{
    type Output = Vector<T, ROWS>;

    fn mul(self, rhs: Vector<T, COLS>) -> Self::Output {
        let mut ret = Vector::new_explode(T::zero());

        for i in 0..ROWS {
            let mut v = T::zero();

            for j in 0..COLS {
                v = v + rhs[j] * (&self)[i][j];
            }

            ret[i] = v;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    /// this function fails to compile for some reason, within a doctest
    ///
    /// something about generic_const_exprs and \[T; ROWS * COLS] failing to evaluate
    #[test]
    pub fn test_cant_doctest() {
        use crate::smallvec::Matrix;
        use crate::smallvec::Vector;

        let identity = Matrix::<f32, 3, 3>([1., 0., 0., 0., 1., 0., 0., 0., 1.]);
        let scale = Matrix::<f32, 3, 3>([2., 0., 0., 0., 2., 0., 0., 0., 2.]);
        let project = Matrix::<f32, 2, 3>([0., 1., 0., 0., 0., 1.]);
        let weird = Matrix::<f32, 3, 3>([1., 2., 3., 4., 5., 6., 7., 8., 9.]);

        let vec = Vector([1., 2., 3.]);

        assert_eq!(identity * vec, vec);
        assert_eq!(scale * vec, vec * 2.);
        assert_eq!(project * vec, Vector([2., 3.]));
        assert_eq!(weird * vec, Vector([14., 32., 50.]));
    }
}
