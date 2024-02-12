use std::slice::ArrayChunks;

use itertools::{Itertools, EitherOrBoth};

pub struct PaddedArrayChunks<'a, T, const N: usize, F: Fn() -> T> {
	iter: ArrayChunks<'a, T, N>,
	filler: F,
	rem: Option<[T; N]>,
}

impl<'a, T, const N: usize, F: Fn() -> I::Item> PaddedArrayChunks<'a, T, N, F> {
	pub fn new(iter: I, filler: F) -> Self {
		Self {
			iter: iter.array_chunks(),
			filler,
			rem: None,
		}
	}
}

impl<'a, T, const N: usize, F: Fn() -> I::Item> Iterator for PaddedArrayChunks<'a, I, N, F> {
	type Item = <ArrayChunks<'a, T, N> as Iterator>::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.as_mut()
			.and_then(|iter| iter
				.next()
				.or_else(|| {
					self.iter
						.into_remainder()
						.map(|rem| {
							for pair in self.rem.)
							{
								match pair {
									EitherOrBoth::Both(rem, item) => *item = rem,
									EitherOrBoth::Left(_rem) => unreachable!(),
									EitherOrBoth::Right(item) => *item = (self.filler)(),
								}
							}

							self.rem
						})
					}
				))
	}
}
