use std::{iter::{ArrayChunks, FusedIterator}, mem::MaybeUninit};

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
			.or_else(|| {
				let mut rem = self.iter
					.take()
					.unwrap() //we know it's safe since we're inside the 'iter.as_mut().map'. this means we only run if iter is the 'Some' variant
					.into_remainder();

				let mut chunk = [const { MaybeUninit::uninit() }; _];

				let mut i = 0;
				while let Some(v) = rem.next() {
					chunk[i].write(v);
					i += 1;
				}
				while i < chunk.len() {
					chunk[i].write((self.filler)());
					i += 1;
				}
				Some(unsafe { MaybeUninit::assume_init(chunk.transpose()) }) //SAFETY: All elements were written
			})
	}
}

impl<I: FusedIterator, const N: usize, F: Fn() -> I::Item> FusedIterator for PaddedArrayChunks<I, N, F> {}
