use std::{iter::FusedIterator, marker::PhantomData};

pub struct MultiZip<V, B>
where
	V: IntoIterator<Item: Iterator> + AsRef<[V::Item]> + AsMut<[V::Item]>,
	B: FromIterator<<V::Item as Iterator>::Item>,
{
	iters: V,
	phantom: PhantomData<B>,
}

impl<V, B> MultiZip<V, B>
where
	V: IntoIterator<Item: Iterator> + AsRef<[V::Item]> + AsMut<[V::Item]>,
	B: FromIterator<<V::Item as Iterator>::Item>,
{
	pub fn new(iters: V) -> Self {
		Self {
			iters,
			phantom: PhantomData,
		}
	}
}

impl<V, B> Iterator for MultiZip<V, B>
where
	V: IntoIterator<Item: Iterator> + AsRef<[V::Item]> + AsMut<[V::Item]>,
	B: FromIterator<<V::Item as Iterator>::Item>,
{
	type Item = B;
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.iters.as_ref().is_empty() { return None; }

		self.iters
			.as_mut()
			.iter_mut()
			.map(V::Item::next)
			.collect()
	}
}

impl<V, B> FusedIterator for MultiZip<V, B>
where
	V: IntoIterator<Item: FusedIterator> + AsRef<[V::Item]> + AsMut<[V::Item]>,
	B: FromIterator<<V::Item as Iterator>::Item>,
{}
