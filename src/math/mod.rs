use std::ops::{Add, Div, Mul, Sub};

use num::{Float, Integer, Num, NumCast, One, PrimInt};
use num_traits::{ConstOne, ConstZero, NumAssignOps};

pub trait GoodNum:
    Sized
    + Copy
    + Num
	+ ConstZero
	+ ConstOne
    + NumCast
    + PartialOrd
	+ NumAssignOps
{ }
//impl<T: Integer + Copy + NumCast + NumAssignOps> GoodNum for T { }
//impl<T: Float + Copy + NumCast + NumAssignOps> GoodNum for T { }
impl GoodNum for i8    { }
impl GoodNum for i16   { }
impl GoodNum for i32   { }
impl GoodNum for i64   { }
impl GoodNum for i128  { }
impl GoodNum for isize { }
impl GoodNum for u8    { }
impl GoodNum for u16   { }
impl GoodNum for u32   { }
impl GoodNum for u64   { }
impl GoodNum for u128  { }
impl GoodNum for usize { }
impl GoodNum for f32   { }
impl GoodNum for f64   { }

pub trait GoodInt: GoodNum + PrimInt {
	fn gcd(x: Self, y: Self) -> Self;
}
impl<T: Integer + GoodNum + PrimInt> GoodInt for T {
	fn gcd(x: Self, y: Self) -> Self {
		num::integer::gcd(x, y)
	}
}

pub trait GoodFloat: GoodNum + Float { }
impl<T: GoodNum + Float> GoodFloat for T { }



pub fn sqr<T: Mul + Copy>(x: T) -> T::Output {
	x * x
}

pub fn ilog10ceil(x: usize) -> usize {
	(std::cmp::max(x, 2) - 1).ilog10() as usize + 1
}

pub fn int_div_round<T: Add<Output = T> + Div<Output = T> + One + Copy>(lhs: T, rhs: T) -> T {
	let two = T::one() + T::one();
	(lhs + (rhs / two)) / rhs
}

pub fn int_div_ceil<T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + One + Copy>(lhs: T, rhs: T) -> T {
	(lhs + (rhs - T::one())) / rhs
}
