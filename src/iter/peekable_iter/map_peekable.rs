use std::iter::FusedIterator;

use itertools::PeekingNext;

use super::PeekableIterator;

pub struct MapPeekable<I, PRF, PMF, NF, B> where
	I: PeekableIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{
	iter: I,
	prf: PRF,
	pmf: PMF,
	nf: NF,
}

impl<I, PRF, PMF, NF, B> MapPeekable<I, PRF, PMF, NF, B> where
	I: PeekableIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{
	pub fn new(iter: I, prf: PRF, pmf: PMF, nf: NF) -> Self {
		return Self {
			iter,
			prf,
			pmf,
			nf,
		};
	}

	pub fn as_inner(&mut self) -> &mut I {
		&mut self.iter
	}
}

impl<I, PRF, PMF, NF, B> Clone for MapPeekable<I, PRF, PMF, NF, B> where
	I: PeekableIterator + Clone,
	I::Item: Clone,
	PRF: Fn(&I::Item) -> &B,
	PRF: Clone,
	PMF: Fn(&mut I::Item) -> &mut B,
	PMF: Clone,
	NF: Fn(I::Item) -> B,
	NF: Clone,
{
	fn clone(&self) -> Self {
		Self {
			iter: self.iter.clone(),
			prf: self.prf.clone(),
			pmf: self.pmf.clone(),
			nf: self.nf.clone(),
		}
	}
}

impl<I, PRF, PMF, NF, B> Iterator for MapPeekable<I, PRF, PMF, NF, B> where
	I: PeekableIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{
	type Item = B;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.next()
			.map(|item| (self.nf)(item))
	}
}

impl<I, PRF, PMF, NF, B> PeekableIterator for MapPeekable<I, PRF, PMF, NF, B> where
	I: PeekableIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{
	fn peek(&mut self) -> Option<&Self::Item> {
		self.iter.peek().map(&self.prf)
	}

	fn peek_mut(&mut self) -> Option<&mut Self::Item> {
		self.iter.peek_mut().map(&self.pmf)
	}
}


impl<I, PRF, PMF, NF, B> ExactSizeIterator for MapPeekable<I, PRF, PMF, NF, B>
where
	I: PeekableIterator + ExactSizeIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{
	fn len(&self) -> usize {
		self.iter.len()
	}
}

impl<I, PRF, PMF, NF, B> DoubleEndedIterator for MapPeekable<I, PRF, PMF, NF, B>
where
	I: PeekableIterator + DoubleEndedIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter
			.next_back()
			.map(|item| (self.nf)(item))
	}

	#[inline]
	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.iter
			.nth_back(n)
			.map(|item| (self.nf)(item))
	}
}

impl<I, PRF, PMF, NF, B> FusedIterator for MapPeekable<I, PRF, PMF, NF, B>
where
	I: PeekableIterator + FusedIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{ }

// TODO: too unstable to use ATM, use later when it works
// unsafe impl<I, PRF, PMF, NF, B> SourceIter for MapPeekable<I, PRF, PMF, NF, B>
// where
// 	I: PeekableIterator,
// 	PRF: Fn(&I::Item) -> &B,
// 	PMF: Fn(&mut I::Item) -> &mut B,
// 	NF: Fn(I::Item) -> B,
// {
// 	type Source = I;

// 	unsafe fn as_inner(&mut self) -> &mut Self::Source {
// 		&mut self.iter
// 	}
// }

impl<I, PRF, PMF, NF, B> PeekingNext for MapPeekable<I, PRF, PMF, NF, B> where
	I: PeekableIterator,
	PRF: Fn(&I::Item) -> &B,
	PMF: Fn(&mut I::Item) -> &mut B,
	NF: Fn(I::Item) -> B,
{
	fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
	where
		Self: Sized,
		F: FnOnce(&Self::Item) -> bool
	{
		self.next_if(accept)
	}
}
