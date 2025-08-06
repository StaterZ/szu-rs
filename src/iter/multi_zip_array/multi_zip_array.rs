use std::iter::FusedIterator;

pub struct MultiZipArray<I: Iterator, const N: usize> {
	iters: [I; N]
}

impl<I: Iterator, const N: usize> MultiZipArray<I, N> {
	pub fn new(iters: [I; N]) -> Self {
		Self { iters }
	}
}

impl<I: Iterator, const N: usize> Iterator for MultiZipArray<I, N> {
	type Item = [I::Item; N];
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.iters.is_empty() { return None; }

		std::array::try_from_fn(|i| self.iters[i].next())
	}
}

impl<I: FusedIterator, const N: usize> FusedIterator for MultiZipArray<I, N> {}
