#![allow(incomplete_features)]
#![feature(auto_traits, adt_const_params, negative_impls)]

mod vector;

pub use vector::Zero;

pub type Vector<T, const N: usize> = vector::Vector<{vector::Variance::Contra}, T, N>;
pub type Covector<T, const N: usize> = vector::Vector<{vector::Variance::Co}, T, N>;
pub type Matrix<T, const R: usize, const C: usize> = Vector<Covector<T, C>, R>;
pub type BilinearForm<T, const N: usize> = Covector<Covector<T, N>, N>;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        /* Just check that these things type check */
        let u = Vector::<f64, 2>::zero();
        let v = Vector::<f64, 4>::zero();
        let q = u * v;
        println!("{:?}", q);

        let x = Matrix::<f64, 2, 4>::zero();
        let y = Vector::<f64, 4>::zero();
        let z : Vector<f64, 2> = x * y;
        println!("{:?}", z);

        let a = Matrix::<f64, 1, 2>::zero();
        let b = Matrix::<f64, 2, 3>::zero();
        let c : Matrix<f64, 1, 3> = a * b;
        println!("{:?}", c);

        let r = BilinearForm::<f64, 10>::zero();
        let s = Vector::<f64, 10>::zero();
        let t : f64 = r * s * s;
        println!("{:?}", t);
        assert_eq!(2 + 2, 4);
    }
}
