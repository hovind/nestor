#![allow(incomplete_features)]
#![feature(array_map, auto_traits, const_generics, negative_impls)]

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let u = Vector::<{Variance::Contra}, f64, 2>::zero();
        let v = Vector::<{Variance::Contra}, f64, 4>::zero();
        let q = u * v;
        println!("{:?}", q);

        let x = Matrix::<f64, 2, 4>::zero();
        let y = Vector::<{Variance::Contra}, f64, 4>::zero();
        let z : Vector<{Variance::Contra}, f64, 2> = x * y;
        println!("{:?}", z);

        let a = Matrix::<f64, 1, 2>::zero();
        let b = Matrix::<f64, 2, 3>::zero();
        let c : Matrix<f64, 1, 3> = a * b;
        println!("{:?}", c);

        let r = TwoForm::<f64, 10>::zero();
        let s = Vector::<{Variance::Contra}, f64, 10>::zero();
        let t : f64 = r * s * s;
        println!("{:?}", t);
        assert_eq!(2 + 2, 4);
    }
}

use core::ops::{Add, Mul};

#[derive(Eq, PartialEq)]
pub enum Variance {
    Co,
    Contra,
}

pub auto trait Distributive {}

#[derive(Clone, Copy, Debug)]
pub struct Vector<const V: Variance, T, const N: usize>([T; N]);

impl<const V: Variance, T, const N: usize> !Distributive for Vector<V, T, N> {}


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
S: Clone + Mul<T, Output = U>,
T: Clone + Distributive,
Vector<V, U, N>: Zero,
{
    type Output = Vector<V, U, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut x = Vector::<V, U, N>::zero();
        for i in 1..N {
            x.0[i] = self.0[i].clone() * rhs.clone();
        }
        x
    }
}

impl<const V: Variance, const N: usize> Mul<Vector<V, f64, N>> for f64 where
{
    type Output = Vector<V, f64, N>;

    fn mul(self, rhs: Vector<V, f64, N>) -> Self::Output {
        Vector(rhs.0.map(|s| s * self.clone()))
    }
}

impl<const V: Variance, S, T, U, const M: usize, const N: usize> Mul<Vector<V, T, M>> for Vector<V, S, N> where
S: Clone + Mul<Vector<V, T, M>, Output = U>,
Vector<V, T, M>: Clone,
Vector<V, U, N>: Zero,
{
    type Output = Vector<V, U, N>;

    fn mul(self, rhs: Vector<V, T, M>) -> Self::Output {
        let mut x = Vector::<V, U, N>::zero();
        for i in 1..N {
            x.0[i] = self.0[i].clone() * rhs.clone();
        }
        x
    }
}

impl<const N: usize, S, T, U> Mul<Vector<{Variance::Contra}, T, N>> for Vector<{Variance::Co}, S, N> where
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
}

impl<const N: usize, S, T, U> Mul<Vector<{Variance::Co}, T, N>> for Vector<{Variance::Contra}, S, N> where
S: Clone + Mul<T, Output = U>,
T: Clone,
U: Add<U, Output = U> + Zero,
{
    type Output = U;

    fn mul(self, rhs: Vector<{Variance::Co}, T, N>) -> Self::Output {
        let mut x = U::zero();
        for i in 0..N {
            x = x + self.0[i].clone() * rhs.0[i].clone();
        }
        x
    }
}

pub type Matrix<T, const R: usize, const C: usize> = Vector<{Variance::Contra}, Vector<{Variance::Co}, T, C>, R>;

pub type TwoForm<T, const N: usize> = Vector<{Variance::Co}, Vector<{Variance::Co}, T, N>, N>;
