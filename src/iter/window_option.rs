use streaming_iterator::StreamingIterator;

pub struct WindowOption<I: Iterator> {
	iter: I,
	item: Option<(Option<I::Item>, I::Item)>,
}

impl<I: Iterator> WindowOption<I> {
	pub fn new(iter: I) -> Self {
		Self {
			iter,
			item: None,
		}
	}
}

impl<I: Iterator> StreamingIterator for WindowOption<I> {
	type Item = (Option<I::Item>, I::Item);

	fn advance(&mut self) {
		self.item = self.iter.next().map(|next_item| (
			self.item.take().map(|(_, item)| item),
			next_item
		));
	}

	fn get(&self) -> Option<&Self::Item> {
		self.item.as_ref()
	}
}

pub trait WindowOptionExt<I: Iterator> {
	fn window_option(self) -> WindowOption<I>;
}

impl<I: Iterator> WindowOptionExt<I> for I {
	fn window_option(self) -> WindowOption<I> {
		WindowOption::new(self)
	}
}
