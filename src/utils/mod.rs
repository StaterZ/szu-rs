use std::ops::Mul;

pub fn sqr<T: Mul<Output = T> + Copy>(x: T) -> T {
	x * x
}
