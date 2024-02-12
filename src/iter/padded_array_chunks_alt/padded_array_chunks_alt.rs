use std::array;

#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PaddedArrayChunks<I: Iterator, const N: usize, F: Fn() -> I::Item> {
	iter: I,
	remainder: Option<array::IntoIter<I::Item, N>>,
	filler: F,
}

impl<I, const N: usize, F: Fn() -> I::Item> PaddedArrayChunks<I, N, F>
where
	I: Iterator,
{
	#[track_caller]
	pub(in crate::iter) fn new(iter: I, filler: F) -> Self {
		assert!(N != 0, "chunk size must be non-zero");
		Self {
			iter,
			remainder: None,
			filler,
		}
	}
}
