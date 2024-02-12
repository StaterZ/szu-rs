use super::PaddedArrayChunks;

impl<T> [T] {
	fn padded_array_chunks<const N: usize, F>(&self, filler: F) -> PaddedArrayChunks<'_, T, N, F> where
		F: Fn() -> T,
	{
		assert!(N != 0, "chunk size must be non-zero");
		PaddedArrayChunks::new(self, filler)
	}
}
