use crate::set_chain;

#[derive(Clone)]
pub struct Bufferd<I: Iterator> {
	iter: I,
	prev: Option<I::Item>,
}

impl<I: Iterator> Bufferd< I> {
	pub fn new(iter: I) -> Self {
		Self {
			iter,
			prev: None,
		}
	}

	fn get_prev(&self) -> &Option<I::Item> {
		self.prev
	}
}

impl<'a, I: Iterator> Iterator for Bufferd<I> {
	type Item = &'a I::Item;

	fn next(&'a mut self) -> Option<&'a I::Item> {
		&set_chain!(self.prev, self.iter.next()).as_ref();
	}
}

pub trait BufferdExt<'a> : Iterator {
	fn bufferd(self) -> Bufferd<Self>
	where
		Self: Sized
	{
		Bufferd::new(self)
	}
}

impl<'a, I: Iterator> BufferdExt<'a> for I {}
