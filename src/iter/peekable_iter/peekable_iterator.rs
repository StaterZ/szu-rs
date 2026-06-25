use std::iter::Peekable;

use itertools::PeekingNext;

use super::MapPeekable;

pub trait PeekableIterator : Iterator + PeekingNext {
	fn peek(&mut self) -> Option<&Self::Item>;
	fn peek_mut(&mut self) -> Option<&mut Self::Item>;

	fn next_if(&mut self, pred: impl FnOnce(&Self::Item) -> bool) -> Option<Self::Item> {
		self
		.peek()
		.map_or(false, pred)
		.then(|| self.next().unwrap()) //unwrap is safe here since 'pred' returned true, meaning we managed to peek something
	}
	
	fn next_if_eq<T>(&mut self, expected: &T) -> Option<Self::Item> where Self::Item: PartialEq<T> {
		self.next_if(|item| item == expected)
	}

	fn peeking_map<PRF, PMF, NF, T>(self, rf: PRF, pmf: PMF, nf: NF) -> MapPeekable<Self, PRF, PMF, NF, T> where
		Self: Sized,
		PRF: Fn(&Self::Item) -> &T,
		PMF: Fn(&mut Self::Item) -> &mut T,
		NF: Fn(Self::Item) -> T,
	{
		MapPeekable::new(self, rf, pmf, nf)
	}
}

impl<I: Iterator> PeekableIterator for Peekable<I> {
	fn peek(&mut self) -> Option<&Self::Item> {
		Peekable::peek(self)
	}

	fn peek_mut(&mut self) -> Option<&mut Self::Item> {
		Peekable::peek_mut(self)
	}

	fn next_if(&mut self, func: impl FnOnce(&Self::Item) -> bool) -> Option<Self::Item> {
		Peekable::next_if(self, func)
	}

	fn next_if_eq<T>(&mut self, expected: &T) -> Option<Self::Item> where Self::Item: PartialEq<T> {
		Peekable::next_if_eq(self, expected)
	}

	// fn next_if_map<R>(&mut self, f: impl FnOnce(Self::Item) -> Result<R, Self::Item>) -> Option<R> {
	// 	Peekable::next_if_map(self, f)
	// }

	// fn next_if_map_mut<R>(&mut self, f: impl FnOnce(&mut Self::Item) -> Option<R>) -> Option<R> {
	// 	Peekable::next_if_map_mut(self, f)
	// }
}

// impl<I: PeekableIterator> PeekingNext for I {
// 	fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
// 	where
// 		Self: Sized,
// 		F: FnOnce(&Self::Item) -> bool
// 	{
// 		self.next_if(accept)
// 	}
// }
