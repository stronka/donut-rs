use std::ops::{AddAssign, Mul};

pub struct Vector<T, const N: usize>
{
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N>
    where
        T: Copy + Default + AddAssign + Mul<Output=T>
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

    pub fn size(&self) -> usize {
        N
    }

    pub fn dot(&self, other: &Vector<T, N>) -> T {
        let mut result: T = Default::default();

        for (x, y) in self.data.iter().zip(other.data.iter()) {
            result += (*x) * (*y);
        }

        result
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
    fn dot_case_zeros_vectors_then_return_zero() {
        let v1: Vector<i32, 3> = Vector::zeros();
        let v2: Vector<i32, 3> = Vector::zeros();


        assert_eq!(v1.dot(&v2), 0);
    }

    #[test]
    fn dot_case_one_vector_is_zero_then_return_zero() {
        let v1: Vector<i32, 3> = Vector::new([0, 1, 2]);
        let v2: Vector<i32, 3> = Vector::zeros();

        assert_eq!(v1.dot(&v2), 0);
    }

    #[test]
    fn dot_case_non_zero_vectors_then_return_value() {
        let v1: Vector<i32, 3> = Vector::new([0, 1, 2]);
        let v2: Vector<i32, 3> = Vector::new([0, 1, 2]);

        assert_eq!(v1.dot(&v2), 5);
    }
}


