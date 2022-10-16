use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Time:
    Sized
    + Clone
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Neg
    + PartialEq
    + Eq
    + PartialOrd
{
    const ZERO: Self;
    const ONE: Self;

    /// Return the minimum of two times
    fn min<'a>(&'a self, rhs: &'a Self) -> &'a Self;

    /// Return the maximum of two times
    fn max<'a>(&'a self, rhs: &'a Self) -> &'a Self;
}
