use itertools::Itertools;

use super::MultiZip;

/// Converts a Iter<Iter<T>> to a Iter<Vec<T>>
/// an input like 'Iter[Iter[a1, a2], Iter[b1, b2]]' would produce an output like 'Iter[Vec[a1, b1], Vec[a2, b2]]'
pub trait MultiZipExt: Sized + Iterator<Item: Iterator> {
	fn multi_zip(self) -> MultiZip<Vec<Self::Item>, Vec<<Self::Item as Iterator>::Item>>;
}

impl<I: Iterator<Item: Iterator>> MultiZipExt for I {
	fn multi_zip(self) -> MultiZip<Vec<Self::Item>, Vec<<Self::Item as Iterator>::Item>> {
		MultiZip::new(self.collect_vec())
	}
}
