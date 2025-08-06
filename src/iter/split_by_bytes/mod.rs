use std::iter::FusedIterator;

pub struct SplitByBytes<'a> {
	input: &'a str,
	max_bytes: usize,
	pos: usize,
}

impl<'a> SplitByBytes<'a> {
	pub fn new(input: &'a str, max_bytes: usize) -> Self {
		Self {
			input,
			max_bytes,
			pos: 0,
		}
	}
}

impl<'a> Iterator for SplitByBytes<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		if self.pos >= self.input.len() {
			return None;
		}

		let begin = self.pos;
		let mut current_bytes = 0;
		while self.pos < self.input.len() {
			let c = self.input[self.pos..].chars().next().unwrap();
			let c_bytes = c.len_utf8();

			if current_bytes + c_bytes > self.max_bytes {
				break;
			}

			current_bytes += c_bytes;
			self.pos += c_bytes;
		}

		Some(&self.input[begin..self.pos])
	}
}

impl<'a> FusedIterator for SplitByBytes<'a> {}
