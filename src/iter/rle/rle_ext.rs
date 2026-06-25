use num::{Bounded, Unsigned};

use crate::iter::PeekableIterator;
use super::Rle;

pub trait RleExt : PeekableIterator<Item: Clone + Eq> {
	fn rle<N: Unsigned + Bounded + Copy + Into<usize> + TryFrom<usize>>(self) -> Rle<Self, N>
	where
		Self: Sized
	{
		Rle::new(self)
	}
}

impl<I: PeekableIterator<Item: Clone + Eq>> RleExt for I {}
