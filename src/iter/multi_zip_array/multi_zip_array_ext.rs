use super::MultiZipArray;

/// Converts a Iter<Iter<T>> to a Iter<Vec<T>>
/// an input like 'Iter[Iter[a1, a2], Iter[b1, b2]]' would produce an output like 'Iter[Vec[a1, b1], Vec[a2, b2]]'
pub trait MultiZipArrayExt<I: Iterator, const N: usize>: Sized + AsRef<[I]> + AsMut<[I]> {
	fn multi_zip_array(self) -> MultiZipArray<I, N>;
}

impl<I: Iterator, const N: usize> MultiZipArrayExt<I, N> for [I; N] {
	fn multi_zip_array(self) -> MultiZipArray<I, N> {
		MultiZipArray::new(self)
	}
}
