use itertools::PeekingNext;

use std::iter::FusedIterator;
use std::num::NonZero;
use std::ops::Try;

/// An iterator that yields the current count and the element during iteration.
///
/// This `struct` is created by the [`enumerate_peek`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`enumerate_peek`]: Iterator::enumerate_peek
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EnumeratePeekable<I> {
	iter: I,
	count: usize,
}
impl<I> EnumeratePeekable<I> {
	pub(in crate::iter) const fn new(iter: I) -> EnumeratePeekable<I> {
		EnumeratePeekable { iter, count: 0 }
	}

	/// Retrieve the current position of the iterator.
	///
	/// If the iterator has not advanced, the position returned will be 0.
	///
	/// The position may also exceed the bounds of the iterator to allow for calculating
	/// the displacement of the iterator from following calls to [`Iterator::next`].
	///
	/// # Examples
	///
	/// ```
	/// #![feature(next_index)]
	///
	/// let arr = ['a', 'b'];
	///
	/// let mut iter = arr.iter().enumerate_peek();
	///
	/// assert_eq!(iter.next_index(), 0);
	/// assert_eq!(iter.next(), Some((0, &'a')));
	///
	/// assert_eq!(iter.next_index(), 1);
	/// assert_eq!(iter.next_index(), 1);
	/// assert_eq!(iter.next(), Some((1, &'b')));
	///
	/// assert_eq!(iter.next_index(), 2);
	/// assert_eq!(iter.next(), None);
	/// assert_eq!(iter.next_index(), 2);
	/// ```
	#[inline]
	pub fn next_index(&self) -> usize {
		self.count
	}
}

impl<I> Iterator for EnumeratePeekable<I>
where
	I: Iterator,
{
	type Item = (usize, <I as Iterator>::Item);

	/// # Overflow Behavior
	///
	/// The method does no guarding against overflows, so enumerating more than
	/// `usize::MAX` elements either produces the wrong result or panics. If
	/// overflow checks are enabled, a panic is guaranteed.
	///
	/// # Panics
	///
	/// Might panic if the index of the element overflows a `usize`.
	#[inline]
	fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
		let a = self.iter.next()?;
		let i = self.count;
		self.count += 1;
		Some((i, a))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<(usize, I::Item)> {
		let a = self.iter.nth(n)?;
		let i = self.count + n;
		self.count = i + 1;
		Some((i, a))
	}

	#[inline]
	fn count(self) -> usize {
		self.iter.count()
	}

	#[inline]
	fn try_fold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
	where
		Self: Sized,
		Fold: FnMut(Acc, Self::Item) -> R,
		R: Try<Output = Acc>,
	{
		#[inline]
		fn enumerate_peek<'a, T, Acc, R>(
			count: &'a mut usize,
			mut fold: impl FnMut(Acc, (usize, T)) -> R + 'a,
		) -> impl FnMut(Acc, T) -> R + 'a {
			move |acc, item| {
				let acc = fold(acc, (*count, item));
				*count += 1;
				acc
			}
		}

		self.iter.try_fold(init, enumerate_peek(&mut self.count, fold))
	}

	#[inline]
	fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
	where
		Fold: FnMut(Acc, Self::Item) -> Acc,
	{
		#[inline]
		fn enumerate_peek<T, Acc>(
			mut count: usize,
			mut fold: impl FnMut(Acc, (usize, T)) -> Acc,
		) -> impl FnMut(Acc, T) -> Acc {
			move |acc, item| {
				let acc = fold(acc, (count, item));
				count += 1;
				acc
			}
		}

		self.iter.fold(init, enumerate_peek(self.count, fold))
	}

	#[inline]
	fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
		let remaining = self.iter.advance_by(n);
		let advanced = match remaining {
			Ok(()) => n,
			Err(rem) => n - rem.get(),
		};
		self.count += advanced;
		remaining
	}
}

impl<I> DoubleEndedIterator for EnumeratePeekable<I>
where
	I: ExactSizeIterator + DoubleEndedIterator,
{
	#[inline]
	fn next_back(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
		let a = self.iter.next_back()?;
		let len = self.iter.len();
		// Can safely add, `ExactSizeIterator` promises that the number of
		// elements fits into a `usize`.
		Some((self.count + len, a))
	}

	#[inline]
	fn nth_back(&mut self, n: usize) -> Option<(usize, <I as Iterator>::Item)> {
		let a = self.iter.nth_back(n)?;
		let len = self.iter.len();
		// Can safely add, `ExactSizeIterator` promises that the number of
		// elements fits into a `usize`.
		Some((self.count + len, a))
	}

	#[inline]
	fn try_rfold<Acc, Fold, R>(&mut self, init: Acc, fold: Fold) -> R
	where
		Self: Sized,
		Fold: FnMut(Acc, Self::Item) -> R,
		R: Try<Output = Acc>,
	{
		// Can safely add and subtract the count, as `ExactSizeIterator` promises
		// that the number of elements fits into a `usize`.
		fn enumerate_peek<T, Acc, R>(
			mut count: usize,
			mut fold: impl FnMut(Acc, (usize, T)) -> R,
		) -> impl FnMut(Acc, T) -> R {
			move |acc, item| {
				count -= 1;
				fold(acc, (count, item))
			}
		}

		let count = self.count + self.iter.len();
		self.iter.try_rfold(init, enumerate_peek(count, fold))
	}

	#[inline]
	fn rfold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
	where
		Fold: FnMut(Acc, Self::Item) -> Acc,
	{
		// Can safely add and subtract the count, as `ExactSizeIterator` promises
		// that the number of elements fits into a `usize`.
		fn enumerate_peek<T, Acc>(
			mut count: usize,
			mut fold: impl FnMut(Acc, (usize, T)) -> Acc,
		) -> impl FnMut(Acc, T) -> Acc {
			move |acc, item| {
				count -= 1;
				fold(acc, (count, item))
			}
		}

		let count = self.count + self.iter.len();
		self.iter.rfold(init, enumerate_peek(count, fold))
	}

	#[inline]
	fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
		// we do not need to update the count since that only tallies the number of items
		// consumed from the front. consuming items from the back can never reduce that.
		self.iter.advance_back_by(n)
	}
}

impl<I> ExactSizeIterator for EnumeratePeekable<I>
where
	I: ExactSizeIterator,
{
	fn len(&self) -> usize {
		self.iter.len()
	}

	fn is_empty(&self) -> bool {
		self.iter.is_empty()
	}
}

// #[doc(hidden)]
// #[unstable(feature = "trusted_random_access", issue = "none")]
// unsafe impl<I> TrustedRandomAccess for EnumeratePeekable<I> where I: TrustedRandomAccess {}

// #[doc(hidden)]
// #[unstable(feature = "trusted_random_access", issue = "none")]
// unsafe impl<I> TrustedRandomAccessNoCoerce for EnumeratePeekable<I>
// where
//     I: TrustedRandomAccessNoCoerce,
// {
//     const MAY_HAVE_SIDE_EFFECT: bool = I::MAY_HAVE_SIDE_EFFECT;
// }

impl<I> FusedIterator for EnumeratePeekable<I> where I: FusedIterator {}

// #[unstable(issue = "none", feature = "trusted_fused")]
// unsafe impl<I: TrustedFused> TrustedFused for EnumeratePeekable<I> {}

// #[unstable(feature = "trusted_len", issue = "37572")]
// unsafe impl<I> TrustedLen for EnumeratePeekable<I> where I: TrustedLen {}

// #[unstable(issue = "none", feature = "inplace_iteration")]
// unsafe impl<I> SourceIter for EnumeratePeekable<I>
// where
//     I: SourceIter,
// {
//     type Source = I::Source;

//     #[inline]
//     unsafe fn as_inner(&mut self) -> &mut I::Source {
//         // SAFETY: unsafe function forwarding to unsafe function with the same requirements
//         unsafe { SourceIter::as_inner(&mut self.iter) }
//     }
// }

// #[unstable(issue = "none", feature = "inplace_iteration")]
// unsafe impl<I: InPlaceIterable> InPlaceIterable for EnumeratePeekable<I> {
//     const EXPAND_BY: Option<NonZero<usize>> = I::EXPAND_BY;
//     const MERGE_BY: Option<NonZero<usize>> = I::MERGE_BY;
// }

impl<I: Default> Default for EnumeratePeekable<I> {
	/// Creates an `EnumeratePeekable` iterator from the default value of `I`
	/// ```
	/// # use core::slice;
	/// # use std::iter::EnumeratePeekable;
	/// let iter: EnumeratePeekable<slice::Iter<'_, u8>> = Default::default();
	/// assert_eq!(iter.len(), 0);
	/// ```
	fn default() -> Self {
		EnumeratePeekable::new(Default::default())
	}
}

// impl<I: PeekableIterator> PeekableIterator for EnumeratePeekable<I> {
// 	type PeekItem<'a> = (usize, &'a I::Item);
// 	type PeekMutItem<'a> = (usize, &'a mut I::Item);
	
// 	fn peek(&mut self) -> Option<Self::PeekItem<'_>> {
// 		self.iter.peek().map(|x| &(self.count, x))
// 	}

// 	fn peek_mut(&mut self) -> Option<Self::PeekMutItem<'_>> {
// 		self.iter.peek().map(|x| &mut (self.count, x))
// 	}
// }

impl<'a, I: PeekingNext> PeekingNext for EnumeratePeekable<I> {
	fn peeking_next<F>(&'a mut self, accept: F) -> Option<Self::Item>
	where
		Self: Sized,
		F: FnOnce(&Self::Item) -> bool
	{
		self.iter.peeking_next(|item| accept(&(self.count, *item)))
	}
}
