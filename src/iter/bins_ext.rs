use std::{collections::HashMap, hash::Hash, ops::AddAssign};
use num_traits::{One, Zero};
use vector_map::VecMap;

pub trait BinsExt : Iterator {
	fn bins_hash_map<TSum: AddAssign + Zero + One>(self, capacity: usize) -> HashMap<Self::Item, TSum>
	where
		Self: Sized + Iterator<Item: Hash + Eq>
	{
		let mut result = HashMap::with_capacity(capacity);
		for item in self {
			*result.entry(item).or_insert(TSum::zero()) += TSum::one();
		}
		result
	}

	fn bins_vec_map<TSum: AddAssign + Zero + One>(self, capacity: usize) -> VecMap<Self::Item, TSum>
	where
		Self: Sized + Iterator<Item: Eq>
	{
		let mut result = VecMap::with_capacity(capacity);
		for item in self {
			match result.get_mut(&item) { //TODO: both .get_mut() and .insert() iterates the internal vec, this is inefficient...
				Some(entry) => *entry += TSum::one(),
				None => _ = result.insert(item, TSum::one()),
			}
		}
		result
	}
}

impl<I: Iterator> BinsExt for I {}
