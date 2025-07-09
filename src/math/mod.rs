pub use abs_diff::AbsDiff;

use num_traits::PrimInt;

mod abs_diff;

pub fn sqr<T: PrimInt>(x: T) -> T {
	x * x
}

pub fn ilog10ceil(x: usize) -> usize {
	(std::cmp::max(x, 2) - 1).ilog10() as usize + 1
}

pub fn int_div_round<T: PrimInt>(lhs: T, rhs: T) -> T {
	let two = T::one() + T::one();
	(lhs + (rhs / two)) / rhs
}

pub fn int_div_ceil<T: PrimInt>(lhs: T, rhs: T) -> T {
	let one = T::one();
	(lhs + (rhs - one)) / rhs
}
