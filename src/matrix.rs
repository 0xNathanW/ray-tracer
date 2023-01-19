use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Neg};
use std::convert::AsRef;
use std::convert::AsMut;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const R: usize, const C: usize>([[f64; C]; R]);

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
    fn default() -> Self {
        Self([[0.0; C]; R])
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(elements: [[f64; C]; R]) -> Self {
        Self(elements)
    }

    pub fn identity() -> Self {
        let mut result = Self::default();
        for i in 0..R {
            result[i][i] = 1.0;
        }
        result
    }

    pub fn scaling(n: f64) -> Self {
        let mut result = Self::default();
        for i in 0..R {
            result[i][i] = n;
        }
        result
    }
}

impl<const R: usize, const C: usize> AsRef<[[f64; C]; R]> for Matrix<R, C> {
    fn as_ref(&self) -> &[[f64; C]; R] {
        &self.0
    }
}

impl<const R: usize, const C: usize> AsMut<[[f64; C]; R]> for Matrix<R, C> {
    fn as_mut(&mut self) -> &mut [[f64; C]; R] {
        &mut self.0
    }
}

impl<const R: usize, const C: usize> Index<usize> for Matrix<R, C> {
    type Output = [f64; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const R: usize, const C: usize> IndexMut<usize> for Matrix<R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const R: usize, const C: usize> Index<(usize, usize)> for Matrix<R, C> {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<R, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl<const S: usize> Add for Matrix<S, S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..S {
            for j in 0..S {
                result[i][j] = self[i][j] + rhs[i][j];
            }
        }
        result
    }
}

impl<const S: usize> AddAssign for Matrix<S, S> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..S {
            for j in 0..S {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl<const S: usize> Sub for Matrix<S, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..S {
            for j in 0..S {
                result[i][j] = self[i][j] - rhs[i][j];
            }
        }
        result
    }
}

impl<const S: usize> SubAssign for Matrix<S, S> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..S {
            for j in 0..S {
                self[i][j] -= rhs[i][j];
            }
        }
    }
}

impl<const R: usize, const C: usize> Neg for Matrix<R, C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..R {
            for j in 0..C {
                result[i][j] = -self[i][j];
            }
        }
        result
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {

    // Naive algorithm.
    pub fn multiply<const K: usize>(&self, rhs: &Matrix<C, K>) -> Matrix<R, K> {
        let mut result = Matrix::<R, K>::default();
        for i in 0..R {
            for j in 0..K {
                for k in 0..C {
                    result[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        result
    }

    pub fn multiply_scaler(&mut self, scaler: f64) {
        for i in 0..R {
            for j in 0..C {
                self[i][j] *= scaler;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = Matrix::<3, 3>::new(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );
        assert_eq!(m[2][1], 8.0);
    
        let n = Matrix::<1, 3>::new(
            [
                [1.0, 2.0, 3.0],
            ]
        );
        assert_eq!(n[0][2], 3.0);
    }

    #[test]
    fn test_identity() {
        let m = Matrix::<3, 3>::identity();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[1][1], 1.0);
        assert_eq!(m[2][2], 1.0);
        assert_eq!(m[0][1], 0.0);
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[1][0], 0.0);
    }

    #[test]
    #[should_panic]
    fn test_bounds() {
        let m = Matrix::<1, 3>::new(
            [[1.0, 2.0, 3.0]]
        );
        m[2][2];
    }

    #[test]
    fn test_add() {
        let a = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let c = a + b;
        assert_eq!(c[0][0], 2.0);
        assert_eq!(c[0][1], 4.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        a += b;
        assert_eq!(a[0][0], 2.0);
        assert_eq!(a[0][1], 4.0);
    }

    #[test]
    fn test_sub() {
        let a = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let c = a - b;
        assert_eq!(c[0][0], 0.0);
        assert_eq!(c[0][1], 0.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        let b = Matrix::<2, 2>::new(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ]
        );
        a -= b;
        assert_eq!(a[0][0], 0.0);
        assert_eq!(a[0][1], 0.0);
    }

    #[test]
    fn test_multiply() {
        let a = Matrix::<3, 3>::new(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );

        let b = Matrix::<3, 3>::new(
            [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]
        );

        let c = a.multiply(&b);
        assert_eq!(c[0][0], 30.0);
        assert_eq!(c[1][0], 66.0);
        assert_eq!(c[1][1], 81.0);
        assert_eq!(c[2][0], 102.0);

        let d = Matrix::<3, 1>::new(
            [
                [4.0],
                [5.0],
                [6.0],
            ]
        );

        let e = Matrix::<1, 3>::new(
            [
                [7.0, 8.0, 9.0],
            ]
        );

        let f = d.multiply(&e);
        assert_eq!(f[0][0], 28.0);
        assert_eq!(f[0][2], 36.0);
        assert_eq!(f[1][1], 40.0);
    }

}