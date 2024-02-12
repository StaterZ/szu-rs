use itertools::{Itertools, IntoChunks};

pub struct BitPacker<I: Iterator<Item = bool>> {
	iter: IntoChunks<I>,
}

impl<I: Iterator<Item = bool>> BitPacker<I> {
	pub fn new(iter: I) -> Self {
		Self {
			iter: iter.chunks(u8::BITS as usize),
		}
	}
}

impl<I: Iterator<Item = bool>> Iterator for BitPacker<I> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.into_iter()
			.next()
			.map(|item| item
				.into_iter()
				.enumerate()
				.filter_map(|(i, bit)| bit.then_some(1 << i))
				.sum())
	}
}
