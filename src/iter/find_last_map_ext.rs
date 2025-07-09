use std::ops::ControlFlow;

pub trait FindLastMapExt : Iterator {
	fn find_last_map<P, A>(mut self, mut pred: P) -> Option<A>
	where
		Self: Iterator + Sized,
		P: FnMut(Self::Item) -> Option<A>,
	{
		let result = self.try_fold(None, move |acc, item| match pred(item) {
			Some(artifact) => ControlFlow::Continue(Some(artifact)),
			None => ControlFlow::Break(acc),
		});

		match result {
			ControlFlow::Break(artifact) => artifact,
			ControlFlow::Continue(artifact) => artifact,
		}
	}

	fn find_last<P>(self, mut pred: P) -> Option<Self::Item>
	where
		Self: Iterator + Sized,
		P: FnMut(&Self::Item) -> bool,
	{
		self.find_last_map(|item| pred(&item).then_some(item))
	}
}

impl<I: Iterator> FindLastMapExt for I {}
