pub struct Consumed<I: Iterator> {
	iter: I,
	consumed: usize,
}

impl<I: Iterator> Consumed<I> {
	pub fn new(iter: I) -> Self {
		Self {
			iter,
			consumed: 0
		}
	}

	pub fn get_consumed(&self) -> usize {
		self.consumed
	}
}

impl<I: Iterator> Iterator for Consumed<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let result = self.iter.next();
		if result.is_some() {
			self.consumed += 1;
		}
		result
	}
}

impl<I: Iterator + Clone> Clone for Consumed<I> {
	fn clone(&self) -> Self {
		Self { 
			iter: self.iter.clone(),
			consumed: self.consumed.clone(),
		}
	}
}

pub trait ConsumedExt : Iterator {
	fn consumed(self) -> Consumed<Self>
	where
		Self: Sized
	{
		Consumed::new(self)
	}
}

impl<I: Iterator> ConsumedExt for I {}
