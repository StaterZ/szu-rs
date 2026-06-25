use std::{iter::FusedIterator, marker::PhantomData};
use deku::prelude::*;
use itertools::Itertools;
use num::{Bounded, Unsigned};

use crate::iter::PeekableIterator;

#[derive(Debug, Clone, Copy)]
pub struct RleRun<T: Clone, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>> {
	pub pattern: T,
	pub len: N,
}

impl<T: Clone, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>> RleRun<T, N> {
	pub fn iter(&self) -> RleRunIter<&T> {
		RleRunIter {
			pattern: &self.pattern,
			len: self.len.into(),
		}
	}

	pub fn map<U: Clone, M: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize> + From<N>>(self, f: impl FnOnce(T) -> U) -> RleRun<U, M> {
		RleRun {
			pattern: f(self.pattern),
			len: M::from(self.len),
		}
	}
}

impl<T: Clone, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>> IntoIterator for RleRun<T, N> {
	type Item = T;

	type IntoIter = RleRunIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		RleRunIter {
			pattern: self.pattern,
			len: self.len.into(),
		}
	}
}

impl<Ctx: Copy, T: Clone + DekuWriter<Ctx>, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize> + DekuWriter<Ctx>> DekuWriter<Ctx> for RleRun<T, N> {
	fn to_writer<W: deku::no_std_io::Write + deku::no_std_io::Seek>(&self, writer: &mut Writer<W>, ctx: Ctx) -> Result<(), DekuError> {
		self.pattern.to_writer(writer, ctx)?;
		self.len.to_writer(writer, ctx)?;
		Ok(())
	}
}

impl<'a, Ctx: Copy, T: Clone + DekuReader<'a, Ctx>, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize> + DekuReader<'a, Ctx>> DekuReader<'a, Ctx> for RleRun<T, N> {
	fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(reader: &mut Reader<R>, ctx: Ctx) -> Result<Self, DekuError> {
		Ok(Self {
			pattern: T::from_reader_with_ctx(reader, ctx)?,
			len: N::from_reader_with_ctx(reader, ctx)?,
		})
	}
}

#[derive(Debug, Clone, Copy)]
pub struct RleRunIter<T: Clone> {
	pub pattern: T,
	pub len: usize,
}
impl<T: Clone> Iterator for RleRunIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		(self.len > 0).then(|| {
			self.len -= 1;
			self.pattern.clone()
		})
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

impl<T: Clone> ExactSizeIterator for RleRunIter<T> {
	fn len(&self) -> usize {
		self.len
	}
}

impl<T: Clone> FusedIterator for RleRunIter<T> { }

///Performs run length encoding on the iterator, merging items if they're equal
pub struct Rle<I: PeekableIterator<Item: Clone + Eq>, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>> {
	iter: I,
	phantom: PhantomData<N>,
}

impl<I: PeekableIterator<Item: Clone + Eq>, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>> Rle<I, N> {
	pub fn new(iter: I) -> Self {
		Self {
			iter,
			phantom: PhantomData::default(),
		}
	}
}

impl<I: PeekableIterator<Item: Clone + Eq>, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>> Iterator for Rle<I, N> {
	type Item = RleRun<I::Item, N>;

	fn next(&mut self) -> Option<Self::Item> {
		let pattern = self.iter.next()?;
		let len = 1 + self.iter
			.by_ref()
			.peeking_take_while(|c| *c == pattern)
			.take(N::max_value().into())
			.count();
		Some(Self::Item {
			pattern,
			len: unsafe { len.try_into().unwrap_unchecked() },
		})
	}
}

impl<I: PeekableIterator<Item: Clone + Eq> + FusedIterator, N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>> FusedIterator for Rle<I, N> {}
