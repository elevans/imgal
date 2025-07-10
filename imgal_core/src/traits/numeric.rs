use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};
use std::iter::Sum;

pub trait ToFloat64:
    Copy
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + MulAssign
    + Sum
    + Into<f64>
    + Sync
{
}

impl<T> ToFloat64 for T where
    T: Copy
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign
        + MulAssign
        + Sum
        + Into<f64>
        + Sync
{
}
