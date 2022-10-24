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

    /// Round toward the nearest integer
    fn round(&self) -> Self;

    /// Round toward the integer below
    fn floor(&self) -> Self;

    /// Round toward the integer above
    fn ceil(&self) -> Self;

    /// Convert the time to a truncated integer
    fn to_integer(&self) -> i64;

    /// Return the minimum of two times
    fn min<'a>(&'a self, rhs: &'a Self) -> &'a Self;

    /// Return the maximum of two times
    fn max<'a>(&'a self, rhs: &'a Self) -> &'a Self;
}
