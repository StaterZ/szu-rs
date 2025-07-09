use std::iter::Peekable;

pub struct Rle<I: Iterator> {
	iter: Peekable<I>,
}

pub struct RleGroup<T> {
	pub pattern: T,
	pub len: u8,
}

impl<I: Iterator> Rle<I> {
	pub fn new(iter: Peekable<I>) -> Self {
		Self { iter }
	}
}

impl<I: Iterator<Item: Eq>> Iterator for Rle<I> {
	type Item = RleGroup<I::Item>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut result = Self::Item {
			pattern: self.iter.next()?,
			len: 0
		};
		while self.iter.next_if_eq(&result.pattern).is_some() {
			result.len += 1;
			if result.len >= u8::MAX { break; }
		}
		Some(result)
	}
}
