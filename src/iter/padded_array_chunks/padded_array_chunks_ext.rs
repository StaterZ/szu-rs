use super::PaddedArrayChunks;

pub trait PaddedArrayChunksExt : Iterator {
	fn padded_array_chunks<const N: usize, F>(self, filler: F) -> PaddedArrayChunks<Self, N, F>
	where
		Self: Sized,
		F: Fn() -> Self::Item,
	{
		assert!(N != 0, "chunk size must be non-zero");
		PaddedArrayChunks::new(self, filler)
	}
}

impl<I: Iterator> PaddedArrayChunksExt for I {}
