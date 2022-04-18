use core::ops::{Add, Mul};

#[derive(Eq, PartialEq)]
pub enum Variance {
    Co,
    Contra,
}

impl Variance {
    const fn flip(self) -> Self {
        match self {
            Variance::Co => Variance::Contra,
            Variance::Contra => Variance::Co,
        }
    }
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

impl<const N: usize, const V: Variance, S, T, U> Mul<Vector<{V.flip()}, T, N>> for Vector<V, S, N> where
S: Clone + Mul<T, Output = U>,
T: Clone,
U: Add<U, Output = U> + Zero,
{
    type Output = U;

    fn mul(self, rhs: Vector<{V.flip()}, T, N>) -> Self::Output {
        let mut x = U::zero();
        for i in 0..N {
            x = x + self.0[i].clone() * rhs.0[i].clone();
        }
        x
    }
}
