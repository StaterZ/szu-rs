pub struct DynamicZip<I: Iterator<Item = T>, T> {
	iters: Vec<I>
}

impl<I: Iterator<Item = T>, T> DynamicZip<I, T> {
	pub fn new(iters: Vec<I>) -> Self {
		Self {
			iters
		}
	}
}

impl<I: Iterator<Item = T>, T> Iterator for DynamicZip<I, T> {
	type Item = Vec<T>;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.iters
			.iter_mut()
			.map(|iter| iter.next())
			.collect()
	}
}