use core::ops::{Add, Mul};

#[derive(std::marker::ConstParamTy, Eq, PartialEq)]
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

#[derive(Clone, Debug)]
pub struct Vector<const V: Variance, T, const N: usize>([T; N]);

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0f32
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0f64
    }
}

impl<const V: Variance, T, const N: usize> Zero for Vector<V, T, N> where
T: Zero,
{
    fn zero() -> Self {
        Self(core::array::from_fn(|_| T::zero()))
    }
}

impl<const V: Variance, S, T, U, const N: usize> Add<Vector<V, S, N>> for Vector<V, T, N> where
T: Add<S, Output = U> + Clone,
S: Clone,
Vector<V, U, N>: Zero,
{
    type Output = Vector<V, U, N>;

    fn add(self, rhs: Vector<V, S, N>) -> Self::Output {
        let mut x = Vector::<V, U, N>::zero();
        for i in 0..N {
            x.0[i] = self.0[i].clone() + rhs.0[i].clone();
        }
        x
    }
}

impl<const V: Variance, S, T, U, const M: usize, const N: usize> Mul<&Vector<V, S, M>> for &Vector<V, T, N> where
for<'a, 'b> &'a T: Mul<&'b Vector<V, S, M>, Output = U>,
Vector<V, U, N>: Zero,
{
    type Output = Vector<V, U, N>;

    fn mul(self, rhs: &Vector<V, S, M>) -> Self::Output {
        let mut x = Vector::<V, U, N>::zero();
        for i in 0..N {
            x.0[i] = &self.0[i] * rhs;
        }
        x
    }
}


macro_rules! impl_inner_product_mul {
    ($lhs:ty, $rhs:ty) => {
        impl<S, T, U, const N: usize> Mul<$rhs> for $lhs where
        for<'a, 'b> &'a T: Mul<&'b S, Output = U>,
        U: Add<U, Output = U> + Zero,
        {
            type Output = U;

            fn mul<'b>(self, rhs: $rhs) -> Self::Output {
                let mut x = U::zero();
                for i in 0..N {
                    x = x + &self.0[i] * &rhs.0[i];
                }
                x
            }
        }

    }
}

impl_inner_product_mul!(&Vector<{Variance::Contra}, T, N>, &Vector<{Variance::Co}, S, N>);
impl_inner_product_mul!(Vector<{Variance::Contra}, T, N>, &Vector<{Variance::Co}, S, N>);
impl_inner_product_mul!(&Vector<{Variance::Co}, T, N>, &Vector<{Variance::Contra}, S, N>);
impl_inner_product_mul!(Vector<{Variance::Co}, T, N>, &Vector<{Variance::Contra}, S, N>);

impl<const V: Variance, T, U, const N: usize> Mul<&f64> for &Vector<V, T, N> where
for<'a, 'b> &'a T: Mul<&'b f64, Output = U>,
Vector<V, U, N>: Zero,
{
    type Output = Vector<V, U, N>;

    fn mul(self, rhs: &f64) -> Self::Output {
        let mut x = Vector::<V, U, N>::zero();
        for i in 0..N {
            x.0[i] = &self.0[i] * &rhs;
        }
        x
    }
}

impl<const V: Variance, T, U, const N: usize> Mul<&Vector<V, T, N>> for &f64 where
for<'a, 'b> &'a f64: Mul<&'b T, Output = U>,
Vector<V, U, N>: Zero,
{
    type Output = Vector<V, U, N>;

    fn mul(self, rhs: &Vector<V, T, N>) -> Self::Output {
        let mut x = Vector::<V, U, N>::zero();
        for i in 0..N {
            x.0[i] = self * &rhs.0[i];
        }
        x
    }
}
