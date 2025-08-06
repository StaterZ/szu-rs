use std::{array, iter::{ArrayChunks, FusedIterator}};

///Functions identically to ArrayChunks, but instead of providing the remainder as a slice, this pads it out and returns it as the last item
pub struct PaddedArrayChunks<I: Iterator, const N: usize, F: Fn() -> I::Item> {
	iter: Option<ArrayChunks<I, N>>,
	filler: F,
}

impl<I: Iterator, const N: usize, F: Fn() -> I::Item> PaddedArrayChunks<I, N, F> {
	pub fn new(iter: I, filler: F) -> Self {
		debug_assert!(N != 0, "chunk size must be non-zero");
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
			.as_mut()?
			.next()
			.or_else(|| self.iter
				.take()
				.unwrap() //we know it's safe since we're inside the 'iter.as_mut().map'. this means we only run if iter is the 'Some' variant
				.into_remainder()
				.map(|rem| {
					let mut rem = rem.into_iter();
					let chunk: Self::Item = array::from_fn(|_| rem
						.next()
						.unwrap_or_else(&self.filler));
					chunk
				}))
	}
}

impl<I: FusedIterator, const N: usize, F: Fn() -> I::Item> FusedIterator for PaddedArrayChunks<I, N, F> {}
