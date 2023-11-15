

use std::ops::{Add, AddAssign, Mul, Sub};

pub struct Matrix<T, const N: usize, const M: usize> {
    data: [[T; M]; N],
}

pub struct Vector<T, const N: usize>
{
    data: [T; N],
}


impl<T, const N: usize, const M: usize> Matrix<T, N, M>
    where T: Copy + Default + AddAssign + Mul<Output=T>
{
    pub fn zeros() -> Self {
        Matrix { data: [[Default::default(); M]; N] }
    }

    pub fn new(data: [[T; M]; N]) -> Self {
        Matrix { data }
    }

    pub fn at(&self, i: usize, j: usize) -> Option<T> {
        if i < N && j < M {
            Some(self.data[i][j])
        } else {
            None
        }
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.data[i][j] = value;
    }

    pub fn dot<const K: usize>(&self, other: &Matrix<T, M, K>) -> Matrix<T, N, K> {
        let mut result: Matrix<T, N, K> = Matrix::zeros();

        for l in 0..N {
            for k in 0..K {
                let mut sum: T = Default::default();

                for j in 0..M {
                    sum += self.data[l][j] * other.data[j][k];
                }

                result.set(l, k, sum);
            }
        }

        result
    }
}


impl<T, const N: usize> Vector<T, N>
    where
        T: Copy + Default + AddAssign + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + PartialOrd, f64: From<T>
{
    pub fn zeros() -> Self {
        Vector { data: [Default::default(); N] }
    }

    pub fn new(data: [T; N]) -> Self {
        Vector { data }
    }

    pub fn at(&self, i: usize) -> Option<T> {
        if i < N {
            Some(self.data[i])
        } else {
            None
        }
    }

    pub fn set(&mut self, i: usize, value: T) {
        self.data[i] = value;
    }

    pub fn size(&self) -> usize {
        N
    }

    pub fn normalize(&self) -> Vector<f64, N> {
        let mut result: Vector<f64, N> = Vector { data: [0.; N] };

        let product: f64 = (self.dot(&self)).try_into().expect("Cannot compute square root");
        let one_over_len = 1./product.sqrt();

        for (i, value) in self.data.iter().enumerate() {
            result.set(i, f64::from(*value) * one_over_len);
        }

        result
    }

    pub fn add(&self, other: &Vector<T, N>) -> Vector<T, N> {
        let mut result: Vector<T, N> = Vector::zeros();

        for (i, (x, y)) in self.data.iter().zip(other.data.iter()).enumerate() {
            result.set(i, *x + *y)
        }

        result
    }

    pub fn dot(&self, other: &Vector<T, N>) -> T {
        let mut result: T = Default::default();

        for (x, y) in self.data.iter().zip(other.data.iter()) {
            result += (*x) * (*y);
        }

        result
    }


    pub fn mdot<const M: usize>(&self, other: &Matrix<T, N, M>) -> Vector<T, M> {
        let mut result: Vector<T, M> = Vector::zeros();

        for j in 0..M {
            let mut sum: T = Default::default();

            for i in 0..N {
                sum += self.data[i] * other.data[i][j];
            }

            result.set(j, sum);
        }

        result
    }

    pub fn approx(&self, other: &Vector<T, N>, eps: T) -> bool{
        for (x, y) in self.data.iter().zip(other.data.iter()) {
            if *y - *x > eps {
                return false;
            }
        }

        true
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_at_case_index_within_bounds_then_return_some_value() {
        let v: Vector<i32, 3> = Vector::new([0, 1, 2]);

        assert_eq!(v.at(0).unwrap(), 0);
        assert_eq!(v.at(1).unwrap(), 1);
        assert_eq!(v.at(2).unwrap(), 2);
    }

    #[test]
    fn vector_at_case_index_outside_bounds_then_return_none() {
        let v: Vector<i32, 3> = Vector::new([0, 1, 2]);

        assert_eq!(v.at(3), None);
    }

    #[test]
    fn zeros_vector_at_case_index_within_bounds_then_return_value() {
        let v: Vector<i32, 3> = Vector::zeros();

        assert_eq!(v.at(0).unwrap(), 0);
        assert_eq!(v.at(1).unwrap(), 0);
        assert_eq!(v.at(2).unwrap(), 0);
    }


    #[test]
    fn zeros_vector_at_case_index_outside_bounds_then_return_none() {
        let v: Vector<i32, 3> = Vector::zeros();

        assert_eq!(v.at(3), None);
    }

    #[test]
    fn vector_dot_case_zeros_vectors_then_return_zero() {
        let v1: Vector<i32, 3> = Vector::zeros();
        let v2: Vector<i32, 3> = Vector::zeros();


        assert_eq!(v1.dot(&v2), 0);
    }

    #[test]
    fn vector_dot_case_one_vector_is_zero_then_return_zero() {
        let v1: Vector<i32, 3> = Vector::new([0, 1, 2]);
        let v2: Vector<i32, 3> = Vector::zeros();

        assert_eq!(v1.dot(&v2), 0);
    }

    #[test]
    fn vector_dot_case_non_zero_vectors_then_return_value() {
        let v1: Vector<i32, 3> = Vector::new([0, 1, 2]);
        let v2: Vector<i32, 3> = Vector::new([0, 1, 2]);

        assert_eq!(v1.dot(&v2), 5);
    }

    #[test]
    fn matrix_at_case_index_within_bounds_then_return_value() {
        let m: Matrix<i32, 2, 2> = Matrix::new([[0, 1], [2, 3]]);

        assert_eq!(m.at(0, 0).unwrap(), 0);
        assert_eq!(m.at(0, 1).unwrap(), 1);
        assert_eq!(m.at(1, 0).unwrap(), 2);
        assert_eq!(m.at(1, 1).unwrap(), 3);
    }

    #[test]
    fn matrix_at_case_index_outside_bounds_then_return_none() {
        let m: Matrix<i32, 2, 2> = Matrix::zeros();

        assert_eq!(m.at(2, 0), None);
        assert_eq!(m.at(0, 2), None);
        assert_eq!(m.at(2, 2), None);
    }

    #[test]
    fn vector_mdot_case_matrix_then_return_new_vector() {
        let v: Vector<i32, 3> = Vector::new([1, 2, 3]);
        let m: Matrix<i32, 3, 2> = Matrix::new([
            [1, 2],
            [3, 4],
            [5, 6],
        ]);

        let result: Vector<i32, 2> = v.mdot(&m);

        assert_eq!(result.at(0).unwrap(), v.dot(&Vector::new([1, 3, 5])));
        assert_eq!(result.at(1).unwrap(), v.dot(&Vector::new([2, 4, 6])));
    }

    #[test]
    fn matrix_dot_always_return_proper_dot_product() {
        let m1: Matrix<i32, 2, 3> = Matrix::new([
            [1, 2, 3],
            [4, 5, 6]
        ]);
        let m2: Matrix<i32, 3, 2> = Matrix::new([
            [1, 2],
            [3, 4],
            [5, 6]
        ]);

        let result: Matrix<i32, 2, 2> = m1.dot(&m2);

        assert_eq!(
            result.at(0, 0).unwrap(),
            Vector::new([1, 2, 3]).dot(&Vector::new([1, 3, 5]))
        );
        assert_eq!(
            result.at(0, 1).unwrap(),
            Vector::new([1, 2, 3]).dot(&Vector::new([2, 4, 6]))
        );
        assert_eq!(
            result.at(1, 0).unwrap(),
            Vector::new([4, 5, 6]).dot(&Vector::new([1, 3, 5]))
        );
        assert_eq!(
            result.at(1, 1).unwrap(),
            Vector::new([4, 5, 6]).dot(&Vector::new([2, 4, 6]))
        );
    }

    #[test]
    fn vector_add_always_return_proper_value() {
        let v1: Vector<i32, 3> = Vector::new([0, 1, 2]);
        let v2: Vector<i32, 3> = Vector::new([2, 1, 0]);
        let expected: Vector<i32, 3> = Vector::new([2, 2, 2]);

        assert!(v1.add(&v2).approx(&expected, 0));
    }
}


