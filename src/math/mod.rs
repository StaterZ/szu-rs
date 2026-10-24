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
impl<T: Copy + Num + ConstZero + ConstOne + NumCast + PartialOrd + NumAssignOps> GoodNum for T { }

pub trait GoodInt: GoodNum + PrimInt + Integer { }
impl<T: GoodNum + PrimInt + Integer> GoodInt for T { }

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
