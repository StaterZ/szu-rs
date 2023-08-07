use std::ops::Deref;

use streaming_iterator::StreamingIterator;

pub struct WindowOption<I>
where
	I: StreamingIterator,
	I::Item: Sized
{
	iter: I,
	item: Option<(Option<I::Item>, I::Item)>,
}

impl<I> WindowOption<I>
where
	I: StreamingIterator,
	I::Item: Sized
{
	pub fn new(iter: I) -> Self {
		Self {
			iter,
			item: None,
		}
	}
}

impl<I> StreamingIterator for WindowOption<I>
where
	I: StreamingIterator,
	I::Item: Sized
{
	type Item = (Option<I::Item>, I::Item);

	fn advance(&mut self) {
		self.item = self.iter.next().map(|next_item| (
			self.item.as_mut().map(|(_, item)| item),
			next_item
		));
	}

	fn get(&self) -> Option<&Self::Item> {
		self.item.as_ref()
	}
}

pub trait WindowOptionExt<I>
where
	I: StreamingIterator,
	I::Item: Sized
{
	fn window_option(self) -> WindowOption<I>;
}

impl<I> WindowOptionExt<I> for I
where
	I: StreamingIterator,
	I::Item: Sized
{
	fn window_option(self) -> WindowOption<I> {
		WindowOption::new(self)
	}
}