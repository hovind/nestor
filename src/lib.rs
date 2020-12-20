#![allow(incomplete_features)]
#![feature(array_map, const_generics)]

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let u = Vector::<{Variance::Contra}, f64, 2>::zero();
        let v = Vector::<{Variance::Contra}, f64, 4>::zero();
        let q = u * v;
        println!("{:?}", q);

        /*
        let x = TwoForm::<f64, 10>::zero();
        let y = Vector::<{Variance::Contra}, f64, 10>::zero();
        let z = x * y * y;
        println!("{:?}", z);
        assert_eq!(2 + 2, 4);
        */
    }
}

use core::ops::{Add, Mul};

#[derive(Eq, PartialEq)]
pub enum Variance {
    Co,
    Contra,
}

#[derive(Clone, Copy, Debug)]
pub struct Vector<const V: Variance, T, const N: usize>([T; N]);


pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0f64
    }
}

impl<const V: Variance, T, const N: usize> Zero for Vector<V, T, N> where
T: Copy + Zero,
{
    fn zero() -> Self {
        Self([T::zero(); N])
    }
}

impl<const V: Variance, const N: usize, T> Add<Self> for Vector<V, T, N> where
T: Clone + Add<T, Output = T> + Zero,
Self: Zero,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut x = Self::zero();
        for i in 0..N {
            x.0[i] = self.0[i].clone() + rhs.0[i].clone();
        }
        x
    }
}



impl<const V: Variance, S, T, U, const N: usize> Mul<T> for Vector<V, S, N> where
T: Clone + Mul<S, Output = U>,
S: Clone,
Vector<V, U, N>: Zero,
{
    type Output = Vector<V, U, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut x = Vector::<V, U, N>::zero();
        for i in 1..N {
            x.0[i] = rhs.clone() * self.0[i].clone();
        }
        x
    }
}

/*impl<const N: usize, S, T, U> Mul<Vector<{Variance::Contra}, T, N>> for Vector<{Variance::Co}, S, N> where
S: Clone + Mul<T, Output = U>,
T: Clone,
U: Add<U, Output = U> + Zero,
{
    type Output = U;

    fn mul(self, rhs: Vector<{Variance::Contra}, T, N>) -> Self::Output {
        let mut x = U::zero();
        for i in 0..N {
            x = x + self.0[i].clone() * rhs.0[i].clone();
        }
        x
    }
}*/

pub type Matrix<T, const N: usize, const M: usize> = Vector<{Variance::Co}, Vector<{Variance::Contra}, T, M>, N>;

pub type TwoForm<T, const N: usize> = Vector<{Variance::Co}, Vector<{Variance::Co}, T, N>, N>;
