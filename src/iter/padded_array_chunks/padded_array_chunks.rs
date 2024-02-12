use std::{iter::ArrayChunks, array};

pub struct PaddedArrayChunks<I: Iterator, const N: usize, F: Fn() -> I::Item> {
	iter: Option<ArrayChunks<I, N>>,
	filler: F,
}

impl<I: Iterator, const N: usize, F: Fn() -> I::Item> PaddedArrayChunks<I, N, F> {
	pub fn new(iter: I, filler: F) -> Self {
		Self {
			iter: Some(iter.array_chunks()),
			filler,
		}
	}
}

impl<I: Iterator, const N: usize, F: Fn() -> I::Item> Iterator for PaddedArrayChunks<I, N, F> {
	type Item = <ArrayChunks<I, N> as Iterator>::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.as_mut()
			.map(|iter| iter.next())
			.and_then(|item| item.or_else(|| self.iter
				.take()
				.unwrap() //we know it's safe since we're inside the 'iter.as_mut().map'. thi9s means we only run if iter is the 'Some' variant
				.into_remainder()
				.map(|rem| {
					let rem_iter = &mut rem.fuse();
					let chunk: Self::Item = array::from_fn(|_| rem_iter
						.next()
						.unwrap_or_else(|| (self.filler)()));
					chunk
				})
			))
	}
}
